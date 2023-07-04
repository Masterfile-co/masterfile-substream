// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MasterfileEvent {
    #[prost(message, optional, tag="101")]
    pub metadata: ::core::option::Option<super::super::common::v1::TransactionMetadata>,
    #[prost(uint64, tag="200")]
    pub ordinal: u64,
    #[prost(oneof="masterfile_event::Event", tags="1, 2, 3, 10, 11, 12, 99, 100")]
    pub event: ::core::option::Option<masterfile_event::Event>,
}
/// Nested message and enum types in `MasterfileEvent`.
pub mod masterfile_event {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Event {
        #[prost(message, tag="1")]
        ChannelFactory(super::super::super::safe::v1::ChannelFactoryEvent),
        #[prost(message, tag="2")]
        SplitFactory(super::super::super::split::v1::SplitFactoryEvent),
        #[prost(message, tag="3")]
        DropFactory(super::super::super::drop::v1::DropFactoryEvent),
        #[prost(message, tag="10")]
        Safe(super::super::super::safe::v1::SafeEvent),
        #[prost(message, tag="11")]
        Split(super::super::super::split::v1::SplitEvent),
        #[prost(message, tag="12")]
        Drop(super::super::super::drop::v1::DropEvent),
        #[prost(message, tag="99")]
        Registry(super::super::super::registry::v1::RegistryEvent),
        #[prost(message, tag="100")]
        MysteryBoxModule(super::super::super::mystery_box::v1::MysteryBoxModuleEvent),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MasterfileEvents {
    #[prost(message, repeated, tag="1")]
    pub events: ::prost::alloc::vec::Vec<MasterfileEvent>,
}
// @@protoc_insertion_point(module)
