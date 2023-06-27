// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Deployment {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(enumeration="DeploymentType", tag="2")]
    pub deployment: i32,
    #[prost(enumeration="ContractType", tag="3")]
    pub contract: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Deployments {
    #[prost(message, repeated, tag="1")]
    pub deployments: ::prost::alloc::vec::Vec<Deployment>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegistryEvent {
    #[prost(string, tag="100")]
    pub address: ::prost::alloc::string::String,
    #[prost(message, optional, tag="101")]
    pub metadata: ::core::option::Option<super::super::common::v1::TransactionMetadata>,
    #[prost(uint64, tag="200")]
    pub ordinal: u64,
    #[prost(oneof="registry_event::Event", tags="1, 2, 3, 4, 5")]
    pub event: ::core::option::Option<registry_event::Event>,
}
/// Nested message and enum types in `RegistryEvent`.
pub mod registry_event {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DeploymentAdded {
        #[prost(string, tag="1")]
        pub deployment: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub name: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FactoryAdded {
        #[prost(string, tag="1")]
        pub factory: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub name: ::prost::alloc::string::String,
        #[prost(uint64, tag="3")]
        pub version: u64,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RoleAdminChanged {
        #[prost(string, tag="1")]
        pub role: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub previous_admin_role: ::prost::alloc::string::String,
        #[prost(string, tag="3")]
        pub new_admin_role: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RoleGranted {
        #[prost(string, tag="1")]
        pub role: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub account: ::prost::alloc::string::String,
        #[prost(string, tag="3")]
        pub sender: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RoleRevoked {
        #[prost(string, tag="1")]
        pub role: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub account: ::prost::alloc::string::String,
        #[prost(string, tag="3")]
        pub sender: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Event {
        #[prost(message, tag="1")]
        DeploymentAdded(DeploymentAdded),
        #[prost(message, tag="2")]
        FactoryAdded(FactoryAdded),
        #[prost(message, tag="3")]
        RoleAdminChanged(RoleAdminChanged),
        #[prost(message, tag="4")]
        RoleGranted(RoleGranted),
        #[prost(message, tag="5")]
        RoleRevoked(RoleRevoked),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegistryEvents {
    #[prost(message, repeated, tag="1")]
    pub events: ::prost::alloc::vec::Vec<RegistryEvent>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DeploymentType {
    UnknownDeployment = 0,
    Contract = 1,
    Factory = 2,
}
impl DeploymentType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DeploymentType::UnknownDeployment => "UNKNOWN_DEPLOYMENT",
            DeploymentType::Contract => "CONTRACT",
            DeploymentType::Factory => "FACTORY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN_DEPLOYMENT" => Some(Self::UnknownDeployment),
            "CONTRACT" => Some(Self::Contract),
            "FACTORY" => Some(Self::Factory),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ContractType {
    UnknownContract = 0,
    Channel = 1,
    Drop = 2,
    Split = 3,
}
impl ContractType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ContractType::UnknownContract => "UNKNOWN_CONTRACT",
            ContractType::Channel => "CHANNEL",
            ContractType::Drop => "DROP",
            ContractType::Split => "SPLIT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN_CONTRACT" => Some(Self::UnknownContract),
            "CHANNEL" => Some(Self::Channel),
            "DROP" => Some(Self::Drop),
            "SPLIT" => Some(Self::Split),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
