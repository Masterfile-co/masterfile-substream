use substreams_ethereum::block_view::LogView;
use substreams_ethereum::Event;

use crate::{
    abi::GnosisSafeL2::events::{
        AddedOwner, ApproveHash, ChangedFallbackHandler, ChangedGuard, ChangedThreshold,
        DisabledModule, EnabledModule, SafeModuleTransaction, SafeMultiSigTransaction,
        SafeReceived, SafeSetup,
    },
    pb::masterfile::safe::v1::safe_event::{self, SafeOwner},
    utils::pretty_hex,
};

pub fn extract_safe_event(log: &LogView) -> Option<safe_event::Event> {
    if let Some(event) = AddedOwner::match_and_decode(log) {
        return Some(safe_event::Event::AddedOwner(safe_event::AddedOwner {
            owner: pretty_hex(&event.owner),
        }));
    }
    if let Some(event) = ApproveHash::match_and_decode(log) {
        return Some(safe_event::Event::ApproveHash(safe_event::ApproveHash {
            owner: pretty_hex(&event.owner),
            hash: pretty_hex(&event.approved_hash),
        }));
    }
    if let Some(event) = ChangedFallbackHandler::match_and_decode(log) {
        return Some(safe_event::Event::ChangedFallbackHandler(
            safe_event::ChangedFallbackHandler {
                handler: pretty_hex(&event.handler),
            },
        ));
    }
    if let Some(event) = ChangedGuard::match_and_decode(log) {
        return Some(safe_event::Event::ChangedGuard(safe_event::ChangedGuard {
            guard: pretty_hex(&event.guard),
        }));
    }
    if let Some(event) = ChangedThreshold::match_and_decode(log) {
        return Some(safe_event::Event::ChangedThreshold(
            safe_event::ChangedThreshold {
                threshold: event.threshold.to_u64(),
            },
        ));
    }
    if let Some(event) = DisabledModule::match_and_decode(log) {
        return Some(safe_event::Event::DisabledModule(
            safe_event::DisabledModule {
                module: pretty_hex(&event.module),
            },
        ));
    }
    if let Some(event) = EnabledModule::match_and_decode(log) {
        return Some(safe_event::Event::EnabledModule(
            safe_event::EnabledModule {
                module: pretty_hex(&event.module),
            },
        ));
    }
    if let Some(event) = SafeModuleTransaction::match_and_decode(log) {
        // TODO!
    }
    if let Some(event) = SafeMultiSigTransaction::match_and_decode(log) {
        // TODO!
    }
    if let Some(event) = SafeReceived::match_and_decode(log) {
        return Some(safe_event::Event::SafeReceived(safe_event::SafeReceived {
            sender: pretty_hex(&event.sender),
            value: event.value.to_string(),
        }));
    }
    if let Some(event) = SafeSetup::match_and_decode(log) {
        return Some(safe_event::Event::SafeSetup(safe_event::SafeSetup {
            initiator: pretty_hex(&event.initiator),
            threshold: event.threshold.to_u64(),
            initializer: pretty_hex(&event.initializer),
            fallback_handler: pretty_hex(&event.fallback_handler),
            owners: event
                .owners
                .iter()
                .map(|o| SafeOwner {
                    address: pretty_hex(o),
                })
                .collect(),
        }));
    }

    None
}
