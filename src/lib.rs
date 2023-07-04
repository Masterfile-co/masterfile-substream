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
use pb::masterfile::registry::v1::{contract_type, ContractType, Module, Modules, RegistryEvent};
use registry::{extract_registry_events, map_contract_type};
use safe::extract_safe_event;
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

#[substreams::handlers::map]
fn map_deployments(param: String, block: Block) -> Result<Deployments, Error> {
    let mut deployments = vec![];

    let factory_address = param.to_lowercase();

    for log in block.logs() {
        if pretty_hex(&log.address()) == factory_address {
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
    log::info!("Deployments: {:?}", deployments);
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
            match deployment.contract_type.unwrap().r#type.unwrap() {
                contract_type::Type::Channel(_) => {
                    if let Some(event) = EnabledModule::match_and_decode(log) {
                        modules.push(Module {
                            address: pretty_hex(&event.module),
                            ordinal: log.block_index() as u64,
                        })
                    }
                }
                _ => {}
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
    let addr = param.split("&&").collect::<Vec<&str>>();

    if addr.len() != 2 {
        panic!("Invalid param")
    }

    let registry_address = addr[0].to_lowercase();
    let split_main_address = addr[1].to_lowercase();

    let mut events = vec![];

    for log in block.logs() {
        let address = pretty_hex(&log.address());
        let metadata = extract_metadata(&log, &block);
        let ordinal = log.block_index() as u64;

        // TODO: If registry
        if address == registry_address {
            events.push(MasterfileEvent {
                metadata,
                ordinal,
                event: Some(masterfile_event::Event::Registry(RegistryEvent {
                    event: extract_registry_events(&log),
                })),
            })
        }
        // All Split events emitted from split main contract
        else if address == split_main_address {
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
                        contract_type::Type::Channel(_) => events.push(MasterfileEvent {
                            event: Some(masterfile_event::Event::Safe(SafeEvent {
                                event: extract_safe_event(&log),
                                safe_address: address.clone(),
                            })),
                            ordinal,
                            metadata,
                        }),
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
        } else if let Some(module) = modules.get_last(&address) {
            // Extract mystery box module event
        }
    }

    Ok(MasterfileEvents { events })
}
