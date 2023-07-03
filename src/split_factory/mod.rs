use substreams_ethereum::{block_view::LogView, Event};

use crate::{pb::masterfile::split::v1::split_factory_event, abi::SplitsFactory::events::SplitDeployed, utils::pretty_hex};


pub fn extract_split_factory_event(log: &LogView) -> Option<split_factory_event::Event> {
	if let Some(event) = SplitDeployed::match_and_decode(log) {
		return Some(split_factory_event::Event::SplitDeployed(
			split_factory_event::SplitDeployed {
				split: pretty_hex(&event.split),
				channel: pretty_hex(&event.channel),
			},
		));
	}

	None
}