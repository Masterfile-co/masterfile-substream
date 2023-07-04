use substreams_ethereum::{block_view::LogView, Event};

use crate::{
    abi::MysteryBoxModule::events::{
        MysteryBoxItemRedeemed, MysteryBoxItemSet, MysteryBoxPurchased, MysteryBoxRevealed,
        MysteryBoxSet,
    },
    pb::masterfile::mystery_box::v1::mystery_box_module_event,
    utils::pretty_hex,
};

pub fn extract_mystery_box_module_event(log: &LogView) -> Option<mystery_box_module_event::Event> {
    if let Some(event) = MysteryBoxItemRedeemed::match_and_decode(log) {
        return Some(mystery_box_module_event::Event::MysteryBoxItemRedeemed(
            mystery_box_module_event::MysteryBoxItemRedeemed {
                id: event.id.to_string(),
                request_id: event.request_id.to_string(),
                recipient: pretty_hex(&event.recipient),
                item_index: event.item_index.to_u64(),
            },
        ));
    }
    if let Some(event) = MysteryBoxItemSet::match_and_decode(log) {
        return Some(mystery_box_module_event::Event::MysteryBoxItemSet(
            mystery_box_module_event::MysteryBoxItemSet {
                id: event.id.to_string(),
                index: event.index.to_u64(),
                drop: pretty_hex(&event.drop),
                token_identifier: event.token_identifier.to_u64(),
                probability: event.probability.to_u64(),
                quantity: event.quantity.to_u64(),
            },
        ));
    }
    if let Some(event) = MysteryBoxPurchased::match_and_decode(log) {
        return Some(mystery_box_module_event::Event::MysteryBoxPurchased(
            mystery_box_module_event::MysteryBoxPurchased {
                id: event.id.to_string(),
                funder: pretty_hex(&event.funder),
                recipient: pretty_hex(&event.recipient),
                quantity: event.quantity.to_string(),
                total_price: event.total_price.to_string(),
            },
        ));
    }
    if let Some(event) = MysteryBoxRevealed::match_and_decode(log) {
        return Some(mystery_box_module_event::Event::MysteryBoxRevealed(
            mystery_box_module_event::MysteryBoxRevealed {
                id: event.id.to_string(),
                request_id: event.request_id.to_string(),
                recipient: pretty_hex(&event.recipient),
            },
        ));
    }
    if let Some(event) = MysteryBoxSet::match_and_decode(log) {
        return Some(mystery_box_module_event::Event::MysteryBoxSet(
            mystery_box_module_event::MysteryBoxSet {
                id: event.id.to_string(),
                recipient: pretty_hex(&event.recipient),
                price: event.price.to_string(),
                start_date: event.start_date.to_u64(),
                end_date: event.end_date.to_u64(),
                redemptions: event.redemptions.to_u64(),
            },
        ));
    }

    None
}
