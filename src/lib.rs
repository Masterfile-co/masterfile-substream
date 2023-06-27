pub mod abi;
pub mod pb;
pub mod utils;

use crate::utils::pretty_hex;
use abi::Registry::events;
use pb::masterfile::registry::v1::{ContractType, Deployment, DeploymentType, Deployments};
use substreams::{errors::Error, log, Hex};
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
                    deployment: DeploymentType::Factory.into(),
                    contract: map_contract_type(&pretty_hex(&event.name)).into(),
                })
            }
            if let Some(event) = events::DeploymentAdded::match_and_decode(log) {
                log::info!("DeploymentAdded: {:?}", pretty_hex(&event.name));
                deployments.push(Deployment {
                    address: pretty_hex(&event.deployment),
                    deployment: DeploymentType::Contract.into(),
                    contract: map_contract_type(&pretty_hex(&event.name)).into(),
                })
            }
        }
    }

    Ok(Deployments { deployments })
}

const CHANNEL_TYPE: &str = "0x446e88dcc2f366f48c68cb0da4f16d5c3aa0bd3060e71140482c0cc6bd95d989"; // keccak256(CHANNEL)
const SPLITS_TYPE: &str = "0x7672f8498473759579af06bd97e96383fed5dbe521f62fc2207b9880b99310b8"; // keccak256(SPLITS)
const DROP_TYPE: &str = "0xfc317125979e6b084fe695d396a56db3b2625567ed41d059d55099c394840719"; // keccak256(MASTERFILE_DROP)

fn map_contract_type(name: &str) -> ContractType {
    match name {
        CHANNEL_TYPE => ContractType::Channel,
        SPLITS_TYPE => ContractType::Split,
        DROP_TYPE => ContractType::Drop,
        _ => ContractType::UnknownContract,
    }
}
