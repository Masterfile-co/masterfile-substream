use crate::abi::MasterfileDrop::events::{
    Approval, ApprovalForAll, DropCreated, DropSaleWindowSet, EditionSet, ListingSet,
    MetaTransactionExecuted, PrimarySale, TokenEditionSet, Transfer,
};
use crate::pb::masterfile::drop::v1::drop_event;
use crate::utils::pretty_hex;
use substreams_ethereum::block_view::LogView;
use substreams_ethereum::Event;

pub fn extract_drop_event(log: &LogView) -> Option<drop_event::Event> {
    if let Some(event) = Approval::match_and_decode(log) {
        return Some(drop_event::Event::Approval(drop_event::Approval {
            owner: pretty_hex(&event.owner),
            approved: pretty_hex(&event.approved),
            token_id: event.token_id.to_string(),
        }));
    }
    if let Some(event) = ApprovalForAll::match_and_decode(log) {
        return Some(drop_event::Event::ApprovalForAll(
            drop_event::ApprovalForAll {
                owner: pretty_hex(&event.owner),
                operator: pretty_hex(&event.operator),
                approved: event.approved,
            },
        ));
    }
    if let Some(event) = DropCreated::match_and_decode(log) {
        return Some(drop_event::Event::DropCreated(drop_event::DropCreated {
            creator: pretty_hex(&event.creator),
            name: event.name,
            symbol: event.symbol,
            series: event.series,
            volume: event.volume.to_u64(),
        }));
    }
    if let Some(event) = DropSaleWindowSet::match_and_decode(log) {
        return Some(drop_event::Event::DropSaleWindowSet(
            drop_event::DropSaleWindowSet {
                drop_start_date: event.drop_start_date.to_string(),
                drop_end_date: event.drop_end_date.to_string(),
            },
        ));
    }
    if let Some(event) = EditionSet::match_and_decode(log) {
        return Some(drop_event::Event::EditionSet(drop_event::EditionSet {
            edition_number: event.edition_number.to_u64(),
            recipient: pretty_hex(&event.recipient),
            arweave_cid: pretty_hex(&event.arweave_cid), // TODO! Check formatting
        }));
    }
    if let Some(event) = ListingSet::match_and_decode(log) {
        return Some(drop_event::Event::ListingSet(drop_event::ListingSet {
            edition_number: event.edition_number.to_u64(),
            quantity: event.quantity.to_string(),
            price: event.price.to_string(),
        }));
    }
    if let Some(event) = MetaTransactionExecuted::match_and_decode(log) {
        return Some(drop_event::Event::MetaTransactionExecuted(
            drop_event::MetaTransactionExecuted {
                user_address: pretty_hex(&event.user_address),
                relayer_address: pretty_hex(&event.relayer_address),
                function_signature: pretty_hex(&event.function_signature),
            },
        ));
    }
    if let Some(event) = PrimarySale::match_and_decode(log) {
        return Some(drop_event::Event::PrimarySale(drop_event::PrimarySale {
            buyer: pretty_hex(&event.buyer),
            recipient: pretty_hex(&event.recipient),
            edition_number: event.edition_number.to_u64(),
            quantity: event.quantity.to_string(),
            total_price: event.total_price.to_string(),
        }));
    }
    if let Some(event) = TokenEditionSet::match_and_decode(log) {
        return Some(drop_event::Event::TokenEditionSet(
            drop_event::TokenEditionSet {
                from_token_id: event.from_token_id.to_string(),
                to_token_id: event.to_token_id.to_string(),
                edition_number: event.edition_number.to_u64(),
            },
        ));
    }
    if let Some(event) = Transfer::match_and_decode(log) {
        return Some(drop_event::Event::Transfer(drop_event::Transfer {
            from: pretty_hex(&event.from),
            to: pretty_hex(&event.to),
            token_id: event.token_id.to_string(),
        }));
    }

    None
}
