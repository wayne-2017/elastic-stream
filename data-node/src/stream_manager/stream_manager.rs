use std::{collections::HashMap, rc::Rc};

use model::{
    data_node::DataNode,
    range::{Range, StreamRange},
    stream::Stream,
};
use slog::{error, info, trace, warn, Logger};
use store::{ElasticStore, Store};

use crate::{error::ServiceError, stream_manager::append_window::AppendWindow};

use super::fetcher::Fetcher;

pub(crate) struct StreamManager {
    log: Logger,

    streams: HashMap<i64, Stream>,

    // TODO: `AppendWindow` should be a nested member of `Stream`.
    windows: HashMap<i64, AppendWindow>,

    fetcher: Fetcher,

    store: Rc<ElasticStore>,
}

impl StreamManager {
    pub(crate) fn new(log: Logger, fetcher: Fetcher, store: Rc<ElasticStore>) -> Self {
        Self {
            log,
            streams: HashMap::new(),
            windows: HashMap::new(),
            fetcher,
            store,
        }
    }

    pub(crate) async fn start(&mut self) -> Result<(), ServiceError> {
        self.fetcher.start();
        let mut bootstrap = false;
        if let Fetcher::PlacementClient { .. } = &self.fetcher {
            bootstrap = true;
        }

        if bootstrap {
            self.bootstrap().await?;
        }
        Ok(())
    }

    /// Bootstrap all stream ranges that are assigned to current data node.
    ///
    /// # Panic
    /// If failed to access store to acquire max offset of the stream with mutable range.
    async fn bootstrap(&mut self) -> Result<(), ServiceError> {
        let ranges = self.fetcher.bootstrap(&self.log).await?;

        for range in ranges {
            let stream = self
                .streams
                .entry(range.stream_id())
                .or_insert(Stream::with_id(range.stream_id()));
            stream.push(range);
        }

        self.streams.iter_mut().for_each(|(_, stream)| {
            stream.sort();
            if stream.is_mut() {
                if let Some(range) = stream.last() {
                    let stream_id = range.stream_id();
                    let start = if let Some(offset) = self
                        .store
                        .max_record_offset(stream_id)
                        .expect("Should get max record offset of given stream")
                    {
                        if offset > range.start() {
                            offset
                        } else {
                            range.start()
                        }
                    } else {
                        range.start()
                    };
                    let append_window = AppendWindow::new(range.index(), start);
                    self.windows.insert(stream_id, append_window);
                    trace!(
                        self.log,
                        "Create a new AppendWindow for stream={} with next={}",
                        stream_id,
                        range.start()
                    );
                }
            }
        });

        Ok(())
    }

    /// Build and update `AppendWindow` for the specified stream.
    ///
    fn build_append_window(&mut self, stream_id: i64) -> Result<(), ServiceError> {
        if let Some(stream) = self.streams.get_mut(&stream_id) {
            if stream.is_mut() {
                let range = stream.last().ok_or_else(|| {
                    error!(
                        self.log,
                        "Last range of a mutable stream shall always be present and mutable"
                    );
                    ServiceError::Internal(String::from("Inconsistent internal state"))
                })?;
                let window = AppendWindow::new(range.index(), range.start());
                trace!(
                    self.log,
                    "Created AppendWindow {:?} for stream={}",
                    window,
                    stream_id
                );
                if let Some(prev) = self.windows.insert(stream_id, window) {
                    if !prev.inflight.is_empty() {
                        warn!(
                            self.log,
                            "Replaced AppendWindow still have {} inflight slots",
                            prev.inflight.len()
                        );
                    }
                }
            } else {
                trace!(
                    self.log,
                    "Stream={} is immutable on current data-node",
                    stream_id
                );
            }
        } else {
            warn!(
                self.log,
                "No stream={} is found on current data-node", stream_id
            );
        }

        Ok(())
    }

    /// Create `Stream` and `AppendWindow` according to metadata from placement manager.
    ///
    ///
    async fn create_stream_if_missing(&mut self, stream_id: i64) -> Result<bool, ServiceError> {
        // If, though unlikely, the stream is firstly assigned to it.
        // TODO: https://doc.rust-lang.org/std/intrinsics/fn.unlikely.html
        if !self.streams.contains_key(&stream_id) {
            trace!(
                self.log,
                "About to fetch ranges for stream[id={}]",
                stream_id
            );
            let mut stream = Stream::with_id(stream_id);
            let node_id = self.store.id();
            self.fetcher
                .fetch(stream_id, &self.log)
                .await?
                .into_iter()
                .filter(|range| {
                    // TODO: Enable filter after PM uses correct data-node ID
                    range
                        .replica()
                        .iter()
                        .map(|data_node| data_node.node_id)
                        .any(|id| node_id == id || true)
                })
                .for_each(|range| {
                    stream.push(range);
                });
            stream.sort();
            self.streams.insert(stream_id, stream);
            self.build_append_window(stream_id)?;
            trace!(self.log, "Created Stream[id={}]", stream_id);
            return Ok(true);
        }
        Ok(false)
    }

    /// Refresh stream ranges from placement-manager.
    ///
    async fn refresh_stream(&mut self, stream_id: i64) -> Result<(), ServiceError> {
        let node_id = self.store.id();
        let mut ranges = self
            .fetcher
            .fetch(stream_id, &self.log)
            .await?
            .into_iter()
            .filter(|range| {
                range
                    .replica()
                    .iter()
                    .map(|data_node| data_node.node_id)
                    .any(|id| node_id == id)
            })
            .collect::<Vec<_>>();
        ranges.sort_by(|a, b| a.index().cmp(&b.index()));

        if let Some(stream) = self.streams.get_mut(&stream_id) {
            // Max range index known
            let start = match stream.last() {
                Some(range) => range.index(),
                None => -1,
            };

            // Filter and push new ranges into stream
            ranges
                .into_iter()
                .filter(|range| range.index() > start)
                .for_each(|range| {
                    stream.push(range);
                });
        }
        self.build_append_window(stream_id)?;
        Ok(())
    }

    async fn ensure_mutable(&mut self, stream_id: i64) -> Result<(), ServiceError> {
        if let Some(stream) = self.streams.get_mut(&stream_id) {
            if stream.is_mut() {
                return Ok(());
            }
        }

        let ranges = self.fetcher.fetch(stream_id, &self.log).await?;
        if let Some(range) = ranges.last() {
            if range.is_sealed() {
                return Err(ServiceError::AlreadySealed);
            }
        }
        if let Some(stream) = self.streams.get_mut(&stream_id) {
            stream.refresh(ranges);
        }

        Ok(())
    }

    pub(crate) fn alloc_record_batch_slots(
        &mut self,
        range: protocol::rpc::header::Range,
        batch_size: usize,
    ) -> Result<u64, ServiceError> {
        trace!(
            self.log,
            "Allocate record slots in batch for stream={}, range-index={}, batch-size={}",
            range.stream_id(),
            range.range_index(),
            batch_size
        );

        let stream_id = range.stream_id();
        let range_index = range.range_index();

        if let Some(window) = self.windows.get_mut(&stream_id) {
            debug_assert_eq!(range_index, window.range_index);
            let start_slot = window.alloc_batch_slots(batch_size);
            return Ok(start_slot);
        }

        let stream = self
            .streams
            .entry(stream_id)
            .or_insert_with(|| Stream::with_id(stream_id));

        if let Some(range) = stream.last() {
            if range.index() > range_index {
                error!(
                    self.log,
                    "Target range to append has been sealed. Stream={}, target-range-index={}, last={}",
                    stream_id,
                    range_index,
                    range.index()
                );
                return Err(ServiceError::AlreadySealed);
            }

            if range.index() == range_index && range.is_sealed() {
                error!(
                    self.log,
                    "Target range to append has been sealed. Target range-index={}, stream={}",
                    range_index,
                    stream_id
                );
                return Err(ServiceError::AlreadySealed);
            }

            // The last range known should have been sealed.
            debug_assert!(range.is_sealed());
            // TODO: if the last range on data-node is not sealed, we need to double-check with placement managers
        }

        // Target range to append into is a new one. Let us create it, and its `AppendWindow`.
        info!(
            self.log,
            "Stream={} has a new range=[{}, -1)",
            stream_id,
            range.start_offset()
        );
        debug_assert_eq!(-1, range.end_offset());
        let mut stream_range = StreamRange::new(
            stream_id,
            range_index,
            range.start_offset() as u64,
            range.start_offset() as u64,
            None,
        );
        range
            .replica_nodes()
            .iter()
            .flatten()
            .for_each(|replica_node| {
                if let Some(node) = replica_node.data_node() {
                    let data_node = DataNode {
                        node_id: node.node_id(),
                        advertise_address: node
                            .advertise_addr()
                            .map(|addr| addr.to_owned())
                            .unwrap_or_default(),
                    };
                    stream_range.replica_mut().push(data_node);
                }
            });

        let mut append_window = AppendWindow::new(range_index, range.start_offset() as u64);
        let offset = append_window.alloc_batch_slots(batch_size);
        stream.push(stream_range);
        self.windows.insert(stream_id, append_window);
        Ok(offset)
    }

    pub(crate) fn ack(&mut self, stream_id: i64, offset: u64) -> Result<(), ServiceError> {
        if let Some(stream) = self.streams.get_mut(&stream_id) {
            if !stream.is_mut() {
                return Err(ServiceError::AlreadySealed);
            }
        }

        if let Some(window) = self.windows.get_mut(&stream_id) {
            window.ack(offset);
        }
        Ok(())
    }

    pub(crate) async fn seal(
        &mut self,
        stream_id: i64,
        range_index: i32,
    ) -> Result<u64, ServiceError> {
        //
        let just_created = self.create_stream_if_missing(stream_id).await?;

        if let Some(stream) = self.streams.get_mut(&stream_id) {
            if !stream.is_mut() && !just_created {
                let need_refresh = match stream.last() {
                    Some(range) => range.index() < range_index,
                    None => true,
                };
                if need_refresh {
                    self.refresh_stream(stream_id).await?;
                }
            }
        }

        if let Some(stream) = self.streams.get_mut(&stream_id) {
            match stream.range(range_index) {
                Some(range) => {
                    // If the range has already been sealed.
                    if let Some(end) = range.end() {
                        return Ok(end);
                    };

                    // Paranoid check
                    if let Some(window) = self.windows.get(&stream_id) {
                        if range_index != window.range_index {
                            error!(self.log, "Inconsistent state: AppendWindow range-index={}, target stream={}, range={}",
                             window.range_index, stream_id, range_index);
                            return Err(ServiceError::Seal);
                        }
                    }

                    if let Some(range) = stream.last() {
                        if range_index != range.index() {
                            error!(self.log, "Inconsistent state of stream={}: Last range-index={}, target range={}",
                            stream_id, range.index(), range_index);
                            return Err(ServiceError::Seal);
                        }
                    }

                    let committed = self
                        .windows
                        .remove(&stream_id)
                        .ok_or_else(|| {
                            error!(self.log, "Mutable stream shall always have an AppendWindow");
                            ServiceError::Internal(String::from(
                                "Mutable stream without AppendWindow",
                            ))
                        })
                        .map(|window| window.commit)?;
                    stream
                        .seal(committed, range_index)
                        .map_err(|e| ServiceError::Seal)
                }
                None => {
                    error!(
                        self.log,
                        "Try to seal non-existing range[stream={}, index={}]",
                        stream_id,
                        range_index
                    );
                    return Err(ServiceError::Seal);
                }
            }
        } else {
            error!(
                self.log,
                "Try to seal a range[index={}] of a non-existing stream[id={}]",
                range_index,
                stream_id
            );
            return Err(ServiceError::Seal);
        }
    }

    pub(crate) async fn describe_range(
        &mut self,
        stream_id: i64,
        range_id: i32,
    ) -> Result<StreamRange, ServiceError> {
        self.create_stream_if_missing(stream_id).await?;

        if let Some(stream) = self.streams.get(&stream_id) {
            if let Some(mut range) = stream.range(range_id) {
                if let None = range.end() {
                    if let Some(window) = self.windows.get(&stream_id) {
                        range.set_limit(window.next);
                    }
                }
                return Ok(range);
            } else {
                return Err(ServiceError::NotFound(format!("Range[index={}]", range_id)));
            }
        }
        return Err(ServiceError::NotFound(format!("Stream[id={}]", stream_id)));
    }

    /// Get a stream by id.
    /// 
    /// # Arguments
    /// `stream_id` - The id of the stream.
    /// 
    /// # Returns
    /// The stream if it exists, otherwise `None`.
    pub(crate) fn get_stream(&self, stream_id: i64) -> Option<&Stream> {
        self.streams.get(&stream_id)
    }
}

#[cfg(test)]
mod tests {
    use std::{error::Error, rc::Rc};

    use model::range::StreamRange;
    use protocol::rpc::header::{Range, RangeT};
    use slog::trace;
    use tokio::sync::{mpsc, oneshot};

    use crate::stream_manager::{fetcher::Fetcher, StreamManager};
    const TOTAL: i32 = 16;

    async fn create_fetcher() -> Fetcher {
        let (tx, mut rx) = mpsc::unbounded_channel();
        let fetcher = Fetcher::Channel { sender: tx };

        tokio_uring::spawn(async move {
            loop {
                match rx.recv().await {
                    Some(task) => {
                        let stream_id = task.stream_id;
                        let ranges = (0..TOTAL)
                            .map(|i| {
                                if i < TOTAL - 1 {
                                    StreamRange::new(
                                        stream_id,
                                        i,
                                        (i * 100) as u64,
                                        ((i + 1) * 100) as u64,
                                        Some(((i + 1) * 100) as u64),
                                    )
                                } else {
                                    StreamRange::new(stream_id, i, (i * 100) as u64, 0, None)
                                }
                            })
                            .collect::<Vec<_>>();
                        if let Err(e) = task.tx.send(Ok(ranges)) {
                            panic!("Failed to transfer mocked ranges");
                        }
                    }
                    None => {
                        break;
                    }
                }
            }
        });

        fetcher
    }

    #[test]
    fn test_seal() -> Result<(), Box<dyn Error>> {
        let logger = test_util::terminal_logger();
        let path = test_util::create_random_path()?;
        trace!(logger, "Test directory: {}", path.to_str().unwrap());
        let _guard = test_util::DirectoryRemovalGuard::new(logger.clone(), path.as_path());
        let wal_path = path.join("wal");
        let index_path = path.join("index");

        let (port_tx, port_rx) = oneshot::channel();
        let (stop_tx, stop_rx) = oneshot::channel();
        let log = logger.clone();
        let handle = std::thread::spawn(move || {
            tokio_uring::start(async {
                let port = test_util::run_listener(log).await;
                let _ = port_tx.send(port);
                let _ = stop_rx.await;
            });
        });
        let port = port_rx.blocking_recv()?;
        let store = test_util::build_store(
            format!("localhost:{}", port),
            wal_path.to_str().unwrap(),
            index_path.to_str().unwrap(),
        );
        let store = Rc::new(store);

        tokio_uring::start(async {
            let fetcher = create_fetcher().await;
            let stream_id = 1;
            let mut stream_manager = StreamManager::new(logger, fetcher, store);
            let mut range = RangeT::default();
            range.stream_id = stream_id;
            range.range_index = TOTAL - 1;
            range.end_offset = -1;
            let mut builder = flatbuffers::FlatBufferBuilder::new();
            let range = range.pack(&mut builder);
            builder.finish(range, None);
            let data = builder.finished_data();
            let range = flatbuffers::root::<Range>(data).unwrap();
            let offset = stream_manager.alloc_record_batch_slots(range, 1).unwrap();
            stream_manager.ack(stream_id, offset).unwrap();
            let seal_offset = stream_manager.seal(stream_id, TOTAL - 1).await.unwrap();
            assert_eq!(offset + 1, seal_offset);
        });

        let _ = stop_tx.send(());
        let _ = handle.join();

        Ok(())
    }

    #[test]
    fn test_describe_range() -> Result<(), Box<dyn Error>> {
        let logger = test_util::terminal_logger();
        let path = test_util::create_random_path()?;
        let _guard = test_util::DirectoryRemovalGuard::new(logger.clone(), path.as_path());
        let wal_path = path.join("wal");
        let index_path = path.join("index");

        let (port_tx, port_rx) = oneshot::channel();
        let (stop_tx, stop_rx) = oneshot::channel();
        let log = logger.clone();
        let handle = std::thread::spawn(move || {
            tokio_uring::start(async {
                let port = test_util::run_listener(log).await;
                let _ = port_tx.send(port);
                let _ = stop_rx.await;
            });
        });
        let port = port_rx.blocking_recv()?;
        let store = test_util::build_store(
            format!("localhost:{}", port),
            wal_path.to_str().unwrap(),
            index_path.to_str().unwrap(),
        );
        let store = Rc::new(store);

        tokio_uring::start(async {
            let fetcher = create_fetcher().await;
            let stream_id = 1;
            let mut stream_manager = StreamManager::new(logger, fetcher, store);
            let mut range = RangeT::default();
            range.stream_id = stream_id;
            range.range_index = TOTAL - 1;
            range.end_offset = -1;
            let mut builder = flatbuffers::FlatBufferBuilder::new();
            let range = range.pack(&mut builder);
            builder.finish(range, None);
            let data = builder.finished_data();
            let range = flatbuffers::root::<Range>(data).unwrap();
            let offset = stream_manager.alloc_record_batch_slots(range, 1).unwrap();
            stream_manager.ack(stream_id, offset).unwrap();
            let range = stream_manager
                .describe_range(stream_id, TOTAL - 1)
                .await
                .unwrap();
            assert_eq!(offset + 1, range.limit());
        });
        let _ = stop_tx.send(());
        let _ = handle.join();
        Ok(())
    }
}
