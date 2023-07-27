// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MysteryBoxModuleEvent {
    #[prost(string, tag="101")]
    pub module_address: ::prost::alloc::string::String,
    #[prost(oneof="mystery_box_module_event::Event", tags="1, 2, 3, 4, 5, 6, 7, 8, 9")]
    pub event: ::core::option::Option<mystery_box_module_event::Event>,
}
/// Nested message and enum types in `MysteryBoxModuleEvent`.
pub mod mystery_box_module_event {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MysteryBoxItemRedeemed {
        #[prost(string, tag="1")]
        pub id: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub request_id: ::prost::alloc::string::String,
        #[prost(string, tag="3")]
        pub recipient: ::prost::alloc::string::String,
        #[prost(uint64, tag="4")]
        pub item_index: u64,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MysteryBoxItemSet {
        #[prost(string, tag="1")]
        pub id: ::prost::alloc::string::String,
        #[prost(uint64, tag="2")]
        pub index: u64,
        #[prost(string, tag="3")]
        pub drop: ::prost::alloc::string::String,
        #[prost(uint64, tag="4")]
        pub token_identifier: u64,
        #[prost(uint64, tag="5")]
        pub probability: u64,
        #[prost(uint64, tag="6")]
        pub quantity: u64,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MysteryBoxPurchased {
        #[prost(string, tag="1")]
        pub id: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub funder: ::prost::alloc::string::String,
        #[prost(string, tag="3")]
        pub recipient: ::prost::alloc::string::String,
        #[prost(string, tag="4")]
        pub quantity: ::prost::alloc::string::String,
        #[prost(string, tag="5")]
        pub total_price: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MysteryBoxRevealed {
        #[prost(string, tag="1")]
        pub id: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub request_id: ::prost::alloc::string::String,
        #[prost(string, tag="3")]
        pub recipient: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MysteryBoxSet {
        #[prost(string, tag="1")]
        pub id: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub recipient: ::prost::alloc::string::String,
        #[prost(string, tag="3")]
        pub price: ::prost::alloc::string::String,
        #[prost(uint64, tag="4")]
        pub start_date: u64,
        #[prost(uint64, tag="5")]
        pub end_date: u64,
        #[prost(uint64, tag="6")]
        pub redemptions: u64,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ApprovalForAll {
        #[prost(string, tag="1")]
        pub owner: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub operator: ::prost::alloc::string::String,
        #[prost(bool, tag="3")]
        pub approved: bool,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TransferSingle {
        #[prost(string, tag="1")]
        pub operator: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub from: ::prost::alloc::string::String,
        #[prost(string, tag="3")]
        pub to: ::prost::alloc::string::String,
        #[prost(string, tag="4")]
        pub id: ::prost::alloc::string::String,
        #[prost(string, tag="5")]
        pub amount: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TransferBatch {
        #[prost(string, tag="1")]
        pub operator: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub from: ::prost::alloc::string::String,
        #[prost(string, tag="3")]
        pub to: ::prost::alloc::string::String,
        #[prost(string, repeated, tag="4")]
        pub ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        #[prost(string, repeated, tag="5")]
        pub amounts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MetaTransactionExecuted {
        #[prost(string, tag="1")]
        pub user_address: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub relayer_address: ::prost::alloc::string::String,
        #[prost(string, tag="3")]
        pub function_signature: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Event {
        #[prost(message, tag="1")]
        MysteryBoxItemRedeemed(MysteryBoxItemRedeemed),
        #[prost(message, tag="2")]
        MysteryBoxItemSet(MysteryBoxItemSet),
        #[prost(message, tag="3")]
        MysteryBoxPurchased(MysteryBoxPurchased),
        #[prost(message, tag="4")]
        MysteryBoxRevealed(MysteryBoxRevealed),
        #[prost(message, tag="5")]
        MysteryBoxSet(MysteryBoxSet),
        #[prost(message, tag="6")]
        ApprovalForAll(ApprovalForAll),
        #[prost(message, tag="7")]
        TransferSingle(TransferSingle),
        #[prost(message, tag="8")]
        TransferBatch(TransferBatch),
        #[prost(message, tag="9")]
        MetaTransactionExecuted(MetaTransactionExecuted),
    }
}
// @@protoc_insertion_point(module)
