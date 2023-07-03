use substreams_ethereum::{block_view::LogView, Event};

use crate::{
    abi::Registry::events::{
        DeploymentAdded, FactoryAdded, RoleAdminChanged, RoleGranted, RoleRevoked,
    },
    pb::masterfile::registry::v1::{
        contract_type::{self, Channel, Drop, Split, Unknown},
        registry_event, ContractType,
    },
    utils::pretty_hex,
};

const CHANNEL_TYPE: &str = "0x446e88dcc2f366f48c68cb0da4f16d5c3aa0bd3060e71140482c0cc6bd95d989"; // keccak256(CHANNEL)
const SPLITS_TYPE: &str = "0x7672f8498473759579af06bd97e96383fed5dbe521f62fc2207b9880b99310b8"; // keccak256(SPLITS)
const DROP_TYPE: &str = "0xfc317125979e6b084fe695d396a56db3b2625567ed41d059d55099c394840719"; // keccak256(MASTERFILE_DROP)

pub fn extract_registry_events(log: &LogView) -> Option<registry_event::Event> {
    if let Some(event) = DeploymentAdded::match_and_decode(log) {
        return Some(registry_event::Event::DeploymentAdded(
            registry_event::DeploymentAdded {
                deployment: pretty_hex(&event.deployment),
                name: pretty_hex(&event.name),
                contract_type: map_contract_type(&pretty_hex(&event.name)),
            },
        ));
    }
    if let Some(event) = FactoryAdded::match_and_decode(log) {
        return Some(registry_event::Event::FactoryAdded(
            registry_event::FactoryAdded {
                factory: pretty_hex(&event.factory),
                name: pretty_hex(&event.name),
                contract_type: map_contract_type(&pretty_hex(&event.name)),
                version: event.version.to_u64(),
            },
        ));
    }
    if let Some(event) = RoleAdminChanged::match_and_decode(log) {
        return Some(registry_event::Event::RoleAdminChanged(
            registry_event::RoleAdminChanged {
                role: pretty_hex(&event.role),
                previous_admin_role: pretty_hex(&event.previous_admin_role),
                new_admin_role: pretty_hex(&event.new_admin_role),
            },
        ));
    }
    if let Some(event) = RoleGranted::match_and_decode(log) {
        return Some(registry_event::Event::RoleGranted(
            registry_event::RoleGranted {
                role: pretty_hex(&event.role),
                account: pretty_hex(&event.account),
                sender: pretty_hex(&event.sender),
            },
        ));
    }
    if let Some(event) = RoleRevoked::match_and_decode(log) {
        return Some(registry_event::Event::RoleRevoked(
            registry_event::RoleRevoked {
                role: pretty_hex(&event.role),
                account: pretty_hex(&event.account),
                sender: pretty_hex(&event.sender),
            },
        ));
    }
    None
}

pub fn map_contract_type(name: &str) -> Option<ContractType> {
    let contract_type = match name {
        CHANNEL_TYPE => ContractType {
            r#type: Some(contract_type::Type::Channel(Channel {})),
        },
        SPLITS_TYPE => ContractType {
            r#type: Some(contract_type::Type::Split(Split {})),
        },
        DROP_TYPE => ContractType {
            r#type: Some(contract_type::Type::Drop(Drop {})),
        },
        _ => ContractType {
            r#type: Some(contract_type::Type::Unknown(Unknown {})),
        },
    };
    Some(contract_type)
}
