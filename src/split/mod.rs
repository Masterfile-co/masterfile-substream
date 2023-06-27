use substreams_ethereum::block_view::LogView;

use crate::pb::masterfile::split::v1::split_event;

pub fn extract_split_event(log: LogView) -> Option<split_event::Event> {
    None
}
