// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Deployment {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub ordinal: u64,
    #[prost(oneof="deployment::DeploymentType", tags="100, 101, 102")]
    pub deployment_type: ::core::option::Option<deployment::DeploymentType>,
    #[prost(oneof="deployment::ContractType", tags="200, 201, 202, 203")]
    pub contract_type: ::core::option::Option<deployment::ContractType>,
}
/// Nested message and enum types in `Deployment`.
pub mod deployment {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Contract {
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Factory {
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Channel {
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Drop {
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Split {
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Unknown {
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum DeploymentType {
        #[prost(message, tag="100")]
        Contract(Contract),
        #[prost(message, tag="101")]
        Factory(Factory),
        #[prost(message, tag="102")]
        UnknownType(Unknown),
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ContractType {
        #[prost(message, tag="200")]
        Channel(Channel),
        #[prost(message, tag="201")]
        Drop(Drop),
        #[prost(message, tag="202")]
        Split(Split),
        #[prost(message, tag="203")]
        UnknownContract(Unknown),
    }
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
// @@protoc_insertion_point(module)
