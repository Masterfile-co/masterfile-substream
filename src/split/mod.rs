use substreams::store::{StoreGet, StoreGetProto};
use substreams_ethereum::{block_view::LogView, Event};

use crate::{
    abi::SplitMain::{
        self,
        events::{
            CancelControlTransfer, ControlTransfer, CreateSplit, DistributeErc20, DistributeEth,
        },
    },
    pb::masterfile::{
        registry::v1::Deployment,
        split::v1::split_event::{self, SplitAllocation},
    },
    utils::pretty_hex,
};

pub fn extract_split_event(
    log: LogView,
    store: &StoreGetProto<Deployment>,
) -> Option<split_event::Event> {
    let decoded_event: split_event::Event;

    if let Some(event) = CancelControlTransfer::match_and_decode(log) {
        decoded_event =
            split_event::Event::CancelControlTransfer(split_event::CancelControlTransfer {
                split: pretty_hex(&event.split),
            });
    } else if let Some(event) = ControlTransfer::match_and_decode(log) {
        decoded_event = split_event::Event::ControlTransfer(split_event::ControlTransfer {
            split: pretty_hex(&event.split),
            new_controller: pretty_hex(&event.new_controller),
            previous_controller: pretty_hex(&event.previous_controller),
        });
    } else if let Some(event) = CreateSplit::match_and_decode(log) {
        decoded_event = split_event::Event::CreateSplit(split_event::CreateSplit {
            split: pretty_hex(&event.split),
            distributor_fee: event.distributor_fee.to_string(),
            controller: pretty_hex(&event.controller),
            allocations: event
                .percent_allocations
                .iter()
                .zip(event.accounts.iter())
                .map(|(percent, account)| SplitAllocation {
                    account: pretty_hex(account),
                    percent_allocation: percent.to_u64(),
                })
                .collect(),
        });
    } else if let Some(event) = DistributeErc20::match_and_decode(log) {
        decoded_event = split_event::Event::DistributeErc20(split_event::DistributeErc20 {
            split: pretty_hex(&event.split),
            token: pretty_hex(&event.token),
            amount: event.amount.to_string(),
            distributor: pretty_hex(&event.distributor_address),
        });
    } else if let Some(event) = DistributeEth::match_and_decode(log) {
        decoded_event = split_event::Event::DistributeEth(split_event::DistributeEth {
            split: pretty_hex(&event.split),
            amount: event.amount.to_string(),
            distributor: pretty_hex(&event.distributor_address),
        });
    } else if let Some(event) = SplitMain::events::InitiateControlTransfer::match_and_decode(log) {
        decoded_event =
            split_event::Event::InitiateControlTransfer(split_event::InitiateControlTransfer {
                split: pretty_hex(&event.split),
                new_potential_controller: pretty_hex(&event.new_potential_controller),
            });
    } else if let Some(event) = SplitMain::events::UpdateSplit::match_and_decode(log) {
        decoded_event = split_event::Event::UpdateSplit(split_event::UpdateSplit {
            split: pretty_hex(&event.split),
            distributor_fee: event.distributor_fee.to_string(),
            allocations: event
                .percent_allocations
                .iter()
                .zip(event.accounts.iter())
                .map(|(percent, account)| SplitAllocation {
                    account: pretty_hex(account),
                    percent_allocation: percent.to_u64(),
                })
                .collect(),
        });
    } else {
        return None;
    }

    let address = extract_split_address(&decoded_event);

    if let Some(_) = store.get_last(&address) {
        return Some(decoded_event);
    }

    None
}

fn extract_split_address(event: &split_event::Event) -> String {
    return match event {
        split_event::Event::CancelControlTransfer(e) => e.split.clone(),
        split_event::Event::ControlTransfer(e) => e.split.clone(),
        split_event::Event::CreateSplit(e) => e.split.clone(),
        split_event::Event::DistributeErc20(e) => e.split.clone(),
        split_event::Event::DistributeEth(e) => e.split.clone(),
        split_event::Event::InitiateControlTransfer(e) => e.split.clone(),
        split_event::Event::UpdateSplit(e) => e.split.clone(),
    };
}
