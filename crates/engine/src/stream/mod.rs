mod claim;
mod delete;
mod group;
mod range;
mod stream_types;
mod write;

use types::value::{Entry, StreamValue};

pub use delete::XDelexPolicy;
pub use stream_types::{StreamRangeItem, XPendingSummary};
pub use write::StreamWriteError;

fn get_stream(entry: &Entry) -> Option<&StreamValue> {
    entry.as_stream()
}

fn get_stream_mut(entry: &mut Entry) -> Option<&mut StreamValue> {
    entry.as_stream_mut()
}
