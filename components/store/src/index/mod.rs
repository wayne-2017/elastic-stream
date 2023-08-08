use crate::error::StoreError;
use model::range::{RangeLifecycleEvent, RangeMetadata};
use tokio::sync::mpsc;

pub(crate) mod compaction;
pub(crate) mod driver;
pub(crate) mod entry;
pub(crate) mod indexer;
pub(crate) mod record_handle;

/// Expose minimum WAL offset.
///
/// WAL file sequence would periodically check and purge deprecated segment files. Once a segment file is removed, min offset of the
/// WAL is be updated. Their index entries, that map to the removed file should be compacted away.
pub trait MinOffset {
    fn min_offset(&self) -> u64;
}

/// Trait of local range manager.
pub trait LocalRangeManager {
    // TODO: error propagation
    fn list_by_stream(&self, stream_id: i64, tx: mpsc::UnboundedSender<RangeMetadata>);

    // TODO: error propagation
    fn list(&self, tx: mpsc::UnboundedSender<RangeMetadata>);

    fn seal(&self, stream_id: i64, range: &RangeMetadata) -> Result<(), StoreError>;

    fn add(&self, stream_id: i64, range: &RangeMetadata) -> Result<(), StoreError>;

    async fn handle_range_lifecycle_event(&self, event: Vec<RangeLifecycleEvent>);
}
