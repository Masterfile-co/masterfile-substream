pub mod abi;
pub mod channel_factory;
pub mod drop;
pub mod drop_factory;
pub mod modules;
pub mod pb;
pub mod registry;
pub mod safe;
pub mod split;
pub mod split_factory;
pub mod utils;

use abi::GnosisSafeL2::events::EnabledModule;
use abi::Registry::events;
use channel_factory::extract_channel_factory_event;
use drop::extract_drop_event;
use drop_factory::extract_drop_factory_event;
use modules::extract_mystery_box_module_event;
use pb::masterfile::mystery_box::v1::MysteryBoxModuleEvent;
use pb::masterfile::registry::v1::{contract_type, Module, Modules, RegistryEvent};
use registry::{extract_registry_events, map_contract_type};
use safe::{extract_safe_event, hydrate_tx_results};
use split::extract_split_event;
use split_factory::extract_split_factory_event;
use utils::{extract_metadata, pretty_hex};

use pb::masterfile::drop::v1::{DropEvent, DropFactoryEvent};
use pb::masterfile::events::v1::{masterfile_event, MasterfileEvent, MasterfileEvents};
use pb::masterfile::registry::v1::{
    deployment_type::{self, Contract, Factory},
    Deployment, DeploymentType, Deployments,
};
use pb::masterfile::safe::v1::{ChannelFactoryEvent, SafeEvent};
use pb::masterfile::split::v1::{SplitEvent, SplitFactoryEvent};

use substreams::prelude::*;
use substreams::{errors::Error, log, store::StoreSetProto};
use substreams_ethereum::{pb::eth::v2::Block, Event};

use serde::{Deserialize, Serialize};
use serde_qs;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct MapDeploymentsParams {
    registry_address: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct MapEventsParams {
    registry_address: String,
    split_main_address: String,
    chain_id: u64,
}

#[substreams::handlers::map]
fn map_deployments(param: String, block: Block) -> Result<Deployments, Error> {
    let mut deployments = vec![];

    let deployment_params: MapDeploymentsParams = serde_qs::from_str(&param).unwrap();

    for log in block.logs() {
        if pretty_hex(&log.address()) == deployment_params.registry_address {
            if let Some(event) = events::FactoryAdded::match_and_decode(log) {
                log::info!("FactoryAdded: {:?}", pretty_hex(&event.name));
                deployments.push(Deployment {
                    address: pretty_hex(&event.factory),
                    deployment_type: Some(DeploymentType {
                        r#type: Some(deployment_type::Type::Factory(Factory {})),
                    }),
                    contract_type: map_contract_type(&pretty_hex(&event.name)),
                    ordinal: log.block_index() as u64,
                })
            }
            if let Some(event) = events::DeploymentAdded::match_and_decode(log) {
                log::info!("DeploymentAdded: {:?}", pretty_hex(&event.name));
                deployments.push(Deployment {
                    address: pretty_hex(&event.deployment),
                    deployment_type: Some(DeploymentType {
                        r#type: Some(deployment_type::Type::Contract(Contract {})),
                    }),
                    contract_type: map_contract_type(&pretty_hex(&event.name)),
                    ordinal: log.block_index() as u64,
                })
            }
        }
    }

    Ok(Deployments { deployments })
}

#[substreams::handlers::store]
fn store_deployments(deployments: Deployments, store: StoreSetProto<Deployment>) {
    for deployment in deployments.deployments {
        store.set(deployment.ordinal, &deployment.address, &deployment);
    }
}

#[substreams::handlers::map]
fn map_modules(store: StoreGetProto<Deployment>, block: Block) -> Result<Modules, Error> {
    let mut modules = vec![];

    for log in block.logs() {
        let address = pretty_hex(&log.address());

        if let Some(deployment) = store.get_last(&address) {
            if let contract_type::Type::Channel(_) =
                deployment.contract_type.unwrap().r#type.unwrap()
            {
                if let Some(event) = EnabledModule::match_and_decode(log) {
                    modules.push(Module {
                        address: pretty_hex(&event.module),
                        ordinal: log.block_index() as u64,
                    })
                }
            }
        }
    }

    Ok(Modules { modules })
}

#[substreams::handlers::store]
fn store_modules(deployments: Deployments, store: StoreSetProto<Deployment>) {
    log::info!("Deployments: {:?}", deployments);
    for deployment in deployments.deployments {
        store.set(deployment.ordinal, &deployment.address, &deployment);
    }
}

#[substreams::handlers::map]
fn map_events(
    param: String,
    deployments: StoreGetProto<Deployment>,
    modules: StoreGetProto<Module>,
    block: Block,
) -> Result<MasterfileEvents, Error> {
    let events_params: MapEventsParams = serde_qs::from_str(&param).unwrap();

    let mut events = vec![];

    for log in block.logs() {
        let address = pretty_hex(&log.address());
        let metadata = extract_metadata(&log, &block);
        let ordinal = log.block_index() as u64;

        // TODO: If registry
        if address == events_params.registry_address {
            events.push(MasterfileEvent {
                metadata,
                ordinal,
                event: Some(masterfile_event::Event::Registry(RegistryEvent {
                    event: extract_registry_events(&log),
                })),
            })
        }
        // All Split events emitted from split main contract
        else if address == events_params.split_main_address {
            events.push(MasterfileEvent {
                event: Some(masterfile_event::Event::Split(SplitEvent {
                    event: extract_split_event(log, &deployments),
                })),
                metadata,
                ordinal,
            })
        }
        // If Events coming from deployed factory or implementation
        else if let Some(deployment) = deployments.get_last(&address) {
            match deployment.deployment_type.unwrap().r#type.unwrap() {
                deployment_type::Type::Factory(_) => {
                    match deployment.contract_type.unwrap().r#type.unwrap() {
                        contract_type::Type::Channel(_) => events.push(MasterfileEvent {
                            event: Some(masterfile_event::Event::ChannelFactory(
                                ChannelFactoryEvent {
                                    event: extract_channel_factory_event(&log),
                                    factory_address: address.clone(),
                                },
                            )),
                            metadata,
                            ordinal,
                        }),
                        contract_type::Type::Drop(_) => events.push(MasterfileEvent {
                            event: Some(masterfile_event::Event::DropFactory(DropFactoryEvent {
                                event: extract_drop_factory_event(&log),
                                factory_address: address.clone(),
                            })),
                            metadata,
                            ordinal,
                        }),
                        contract_type::Type::Split(_) => events.push(MasterfileEvent {
                            event: Some(masterfile_event::Event::SplitFactory(SplitFactoryEvent {
                                event: extract_split_factory_event(&log),
                                factory_address: address.clone(),
                            })),
                            metadata,
                            ordinal,
                        }),
                        contract_type::Type::Unknown(_) => {}
                    }
                }
                deployment_type::Type::Contract(_) => {
                    match deployment.contract_type.unwrap().r#type.unwrap() {
                        contract_type::Type::Channel(_) => {
                            // Find gnosis safe events
                            if let Some(event) = extract_safe_event(&log, &events_params.chain_id) {
                                events.push(MasterfileEvent {
                                    event: Some(masterfile_event::Event::Safe(SafeEvent {
                                        event: Some(event),
                                        safe_address: address.clone(),
                                    })),
                                    ordinal,
                                    metadata,
                                })
                            }
                            // Add results for each safe transaction
                            hydrate_tx_results(&log, &mut events);
                        }
                        contract_type::Type::Drop(_) => {
                            events.push(MasterfileEvent {
                                event: Some(masterfile_event::Event::Drop(DropEvent {
                                    event: extract_drop_event(&log),
                                    drop_address: address.clone(),
                                })),
                                ordinal,
                                metadata,
                            });
                        }
                        contract_type::Type::Split(_) => {
                            // Split events handled by SplitMain
                        }
                        contract_type::Type::Unknown(_) => {
                            // TODO: Unknown events
                        }
                    }
                }
                deployment_type::Type::Unknown(_) => {}
            }
        } else if let Some(_) = modules.get_last(&address) {
            // Extract mystery box module event
            if let Some(event) = extract_mystery_box_module_event(&log) {
                events.push(MasterfileEvent {
                    event: Some(masterfile_event::Event::MysteryBoxModule(
                        MysteryBoxModuleEvent {
                            event: Some(event),
                            module_address: address.clone(),
                        },
                    )),
                    ordinal,
                    metadata,
                })
            }
        }
    }

    Ok(MasterfileEvents { events })
}
