pub mod abi;
pub mod channel_factory;
pub mod drop;
pub mod pb;
pub mod safe;
pub mod split;
pub mod utils;

use crate::utils::pretty_hex;
use abi::Registry::events;
use channel_factory::extract_channel_factory_event;
use drop::extract_drop_event;
use pb::masterfile::common::v1::TransactionMetadata;
use pb::masterfile::drop::v1::DropEvent;
use pb::masterfile::events::v1::{masterfile_event, MasterfileEvent, MasterfileEvents};
use pb::masterfile::registry::v1::deployment::Channel;
use pb::masterfile::registry::v1::{
    deployment::{Contract, ContractType, DeploymentType, Drop, Factory, Split, Unknown},
    Deployment, Deployments,
};
use pb::masterfile::safe::v1::{ChannelFactoryEvent, SafeEvent};
use safe::extract_safe_event;
use substreams::prelude::*;
use substreams::{errors::Error, log, store::StoreSetProto, Hex};
use substreams_ethereum::block_view::LogView;
use substreams_ethereum::{pb::eth::v2::Block, Event};

#[substreams::handlers::map]
fn map_deployments(param: String, block: Block) -> Result<Deployments, Error> {
    let mut deployments = vec![];

    let factory_address = &Hex::decode(&param).unwrap();

    for log in block.logs() {
        if log.address() == factory_address {
            if let Some(event) = events::FactoryAdded::match_and_decode(log) {
                log::info!("FactoryAdded: {:?}", pretty_hex(&event.name));
                deployments.push(Deployment {
                    address: pretty_hex(&event.factory),
                    deployment_type: Some(DeploymentType::Factory(Factory {})),
                    contract_type: map_contract_type(&pretty_hex(&event.name)).into(),
                    ordinal: log.block_index() as u64,
                })
            }
            if let Some(event) = events::DeploymentAdded::match_and_decode(log) {
                log::info!("DeploymentAdded: {:?}", pretty_hex(&event.name));
                deployments.push(Deployment {
                    address: pretty_hex(&event.deployment),
                    deployment_type: Some(DeploymentType::Contract(Contract {})),
                    contract_type: map_contract_type(&pretty_hex(&event.name)).into(),
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
fn map_events(block: Block, store: StoreGetProto<Deployment>) -> Result<MasterfileEvents, Error> {
    let mut events = vec![];

    for log in block.logs() {
        let address = pretty_hex(&log.address());
        let metadata = extract_metadata(&log, &block);
        let ordinal = log.block_index() as u64;

        // TODO: If registry

        // TODO: If split main

        if let Some(deployment) = store.get_last(&address) {
            match deployment.deployment_type.unwrap() {
                DeploymentType::Factory(_) => match deployment.contract_type.unwrap() {
                    ContractType::Channel(_) => events.push(MasterfileEvent {
                        event: Some(masterfile_event::Event::ChannelFactory(
                            ChannelFactoryEvent {
                                event: extract_channel_factory_event(&log),
                                factory_address: address.clone(),
                            },
                        )),
                        metadata,
                        ordinal,
                    }),
                    ContractType::Drop(_) => {
                        // TODO: Drop factory events
                    }
                    ContractType::Split(_) => {
                        // TODO: Split factory events
                    }
                    ContractType::UnknownContract(_) => {}
                },
                DeploymentType::Contract(_) => match deployment.contract_type.unwrap() {
                    ContractType::Channel(_) => events.push(MasterfileEvent {
                        event: Some(masterfile_event::Event::Safe(SafeEvent {
                            event: extract_safe_event(&log),
                            safe_address: address.clone(),
                        })),
                        ordinal,
                        metadata,
                    }),
                    ContractType::Drop(_) => {
                        events.push(MasterfileEvent {
                            event: Some(masterfile_event::Event::Drop(DropEvent {
                                event: extract_drop_event(&log),
                                drop_address: address.clone(),
                            })),
                            ordinal,
                            metadata,
                        });
                    }

                    // TODO: Drop events
                    ContractType::Split(_) => {
                        // TODO: Split events
                    }
                    ContractType::UnknownContract(_) => {
                        // TODO: Unknown events
                    }
                },
                DeploymentType::UnknownType(_) => {}
            }
        }
    }

    Ok(MasterfileEvents { events })
}

const CHANNEL_TYPE: &str = "0x446e88dcc2f366f48c68cb0da4f16d5c3aa0bd3060e71140482c0cc6bd95d989"; // keccak256(CHANNEL)
const SPLITS_TYPE: &str = "0x7672f8498473759579af06bd97e96383fed5dbe521f62fc2207b9880b99310b8"; // keccak256(SPLITS)
const DROP_TYPE: &str = "0xfc317125979e6b084fe695d396a56db3b2625567ed41d059d55099c394840719"; // keccak256(MASTERFILE_DROP)

fn map_contract_type(name: &str) -> Option<ContractType> {
    let contract_type = match name {
        CHANNEL_TYPE => ContractType::Channel(Channel {}),
        SPLITS_TYPE => ContractType::Split(Split {}),
        DROP_TYPE => ContractType::Drop(Drop {}),
        _ => ContractType::UnknownContract(Unknown {}),
    };
    Some(contract_type)
}

pub fn extract_metadata(log: &LogView, block: &Block) -> Option<TransactionMetadata> {
    Some(TransactionMetadata {
        tx_hash: pretty_hex(&log.receipt.transaction.hash),
        block_number: block.number,
        block_timestamp: block.timestamp_seconds(),
        to: pretty_hex(&log.receipt.transaction.to),
        from: pretty_hex(&log.receipt.transaction.from),
        log_index: log.log.index,
        block_index: log.log.block_index,
    })
}
