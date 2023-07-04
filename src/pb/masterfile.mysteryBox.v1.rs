// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MysteryBoxModuleEvent {
    #[prost(string, tag="101")]
    pub module_address: ::prost::alloc::string::String,
    #[prost(oneof="mystery_box_module_event::Event", tags="1, 2, 3, 4, 5")]
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
    }
}
// @@protoc_insertion_point(module)
