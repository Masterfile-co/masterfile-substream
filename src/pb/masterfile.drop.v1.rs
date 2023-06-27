// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DropEvent {
    #[prost(string, tag="101")]
    pub drop_address: ::prost::alloc::string::String,
    #[prost(oneof="drop_event::Event", tags="1, 2, 3, 4, 5, 6, 7, 8, 9, 10")]
    pub event: ::core::option::Option<drop_event::Event>,
}
/// Nested message and enum types in `DropEvent`.
pub mod drop_event {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Approval {
        #[prost(string, tag="1")]
        pub owner: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub approved: ::prost::alloc::string::String,
        #[prost(string, tag="3")]
        pub token_id: ::prost::alloc::string::String,
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
    pub struct DropCreated {
        #[prost(string, tag="1")]
        pub creator: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub name: ::prost::alloc::string::String,
        #[prost(string, tag="3")]
        pub symbol: ::prost::alloc::string::String,
        #[prost(string, tag="4")]
        pub series: ::prost::alloc::string::String,
        #[prost(uint64, tag="5")]
        pub volume: u64,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DropSaleWindowSet {
        #[prost(string, tag="1")]
        pub drop_start_date: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub drop_end_date: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ListingSet {
        #[prost(uint64, tag="1")]
        pub edition_number: u64,
        #[prost(string, tag="2")]
        pub quantity: ::prost::alloc::string::String,
        #[prost(string, tag="3")]
        pub price: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MysteryBoxSet {
        #[prost(uint64, repeated, tag="1")]
        pub probabilities: ::prost::alloc::vec::Vec<u64>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EditionSet {
        #[prost(uint64, tag="1")]
        pub edition_number: u64,
        #[prost(string, tag="2")]
        pub arweave_cid: ::prost::alloc::string::String,
        #[prost(string, tag="3")]
        pub recipient: ::prost::alloc::string::String,
    }
    /// TODO: Move to shared proto
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
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PrimarySale {
        #[prost(string, tag="1")]
        pub buyer: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub recipient: ::prost::alloc::string::String,
        #[prost(uint64, tag="3")]
        pub edition_number: u64,
        #[prost(string, tag="4")]
        pub quantity: ::prost::alloc::string::String,
        #[prost(string, tag="5")]
        pub total_price: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TokenEditionSet {
        #[prost(string, tag="1")]
        pub from_token_id: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub to_token_id: ::prost::alloc::string::String,
        #[prost(uint64, tag="3")]
        pub edition_number: u64,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Transfer {
        #[prost(string, tag="1")]
        pub from: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub to: ::prost::alloc::string::String,
        #[prost(string, tag="3")]
        pub token_id: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RandomnessRequested {
        #[prost(string, tag="1")]
        pub token_id: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub request_id: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RandomnessReceived {
        #[prost(string, tag="1")]
        pub token_id: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub request_id: ::prost::alloc::string::String,
        #[prost(string, tag="3")]
        pub randomness: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Event {
        #[prost(message, tag="1")]
        Approval(Approval),
        #[prost(message, tag="2")]
        ApprovalForAll(ApprovalForAll),
        #[prost(message, tag="3")]
        DropCreated(DropCreated),
        #[prost(message, tag="4")]
        DropSaleWindowSet(DropSaleWindowSet),
        #[prost(message, tag="5")]
        EditionSet(EditionSet),
        #[prost(message, tag="6")]
        ListingSet(ListingSet),
        #[prost(message, tag="7")]
        MetaTransactionExecuted(MetaTransactionExecuted),
        #[prost(message, tag="8")]
        PrimarySale(PrimarySale),
        #[prost(message, tag="9")]
        TokenEditionSet(TokenEditionSet),
        #[prost(message, tag="10")]
        Transfer(Transfer),
    }
}
// @@protoc_insertion_point(module)
