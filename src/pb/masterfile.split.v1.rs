// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SplitEvent {
    #[prost(oneof="split_event::Event", tags="1, 2, 3, 4, 5, 6, 7")]
    pub event: ::core::option::Option<split_event::Event>,
}
/// Nested message and enum types in `SplitEvent`.
pub mod split_event {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CancelControlTransfer {
        #[prost(string, tag="1")]
        pub split: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ControlTransfer {
        #[prost(string, tag="1")]
        pub split: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub previous_controller: ::prost::alloc::string::String,
        #[prost(string, tag="3")]
        pub new_controller: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CreateSplit {
        #[prost(string, tag="1")]
        pub split: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub distributor_fee: ::prost::alloc::string::String,
        #[prost(string, tag="3")]
        pub controller: ::prost::alloc::string::String,
        #[prost(message, repeated, tag="4")]
        pub allocations: ::prost::alloc::vec::Vec<SplitAllocation>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DistributeErc20 {
        #[prost(string, tag="1")]
        pub split: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub token: ::prost::alloc::string::String,
        #[prost(string, tag="3")]
        pub amount: ::prost::alloc::string::String,
        #[prost(string, tag="4")]
        pub distributor: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DistributeEth {
        #[prost(string, tag="1")]
        pub split: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub amount: ::prost::alloc::string::String,
        #[prost(string, tag="3")]
        pub distributor: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InitiateControlTransfer {
        #[prost(string, tag="1")]
        pub split: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub new_potential_controller: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UpdateSplit {
        #[prost(string, tag="1")]
        pub split: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub distributor_fee: ::prost::alloc::string::String,
        #[prost(message, repeated, tag="3")]
        pub allocations: ::prost::alloc::vec::Vec<SplitAllocation>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SplitAllocation {
        #[prost(string, tag="1")]
        pub account: ::prost::alloc::string::String,
        #[prost(uint64, tag="2")]
        pub percent_allocation: u64,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Event {
        #[prost(message, tag="1")]
        CancelControlTransfer(CancelControlTransfer),
        #[prost(message, tag="2")]
        ControlTransfer(ControlTransfer),
        #[prost(message, tag="3")]
        CreateSplit(CreateSplit),
        #[prost(message, tag="4")]
        DistributeErc20(DistributeErc20),
        #[prost(message, tag="5")]
        DistributeEth(DistributeEth),
        #[prost(message, tag="6")]
        InitiateControlTransfer(InitiateControlTransfer),
        /// Withdrawl withdrawl = 8;
        #[prost(message, tag="7")]
        UpdateSplit(UpdateSplit),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SplitFactoryEvent {
    #[prost(string, tag="100")]
    pub factory_address: ::prost::alloc::string::String,
    #[prost(oneof="split_factory_event::Event", tags="1")]
    pub event: ::core::option::Option<split_factory_event::Event>,
}
/// Nested message and enum types in `SplitFactoryEvent`.
pub mod split_factory_event {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SplitDeployed {
        #[prost(string, tag="1")]
        pub split: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub channel: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Event {
        #[prost(message, tag="1")]
        SplitDeployed(SplitDeployed),
    }
}
// @@protoc_insertion_point(module)
