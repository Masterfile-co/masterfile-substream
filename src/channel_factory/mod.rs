use substreams_ethereum::{block_view::LogView, Event};

use crate::{
    abi::ChannelFactory::events::ChannelDeployed, pb::masterfile::safe::v1::channel_factory_event,
    utils::pretty_hex,
};

pub fn extract_channel_factory_event(log: &LogView) -> Option<channel_factory_event::Event> {
    if let Some(event) = ChannelDeployed::match_and_decode(log) {
        return Some(channel_factory_event::Event::ChannelDeployed(
            channel_factory_event::ChannelDeployed {
                channel: pretty_hex(&event.channel),
                deployer: pretty_hex(&event.deployer),
            },
        ));
    }

    None
}
