use crate::request;
use bytes::Bytes;
use codec::frame::Frame;
use log::debug;
use log::error;
use log::info;
use log::trace;
use log::warn;
use model::error::EsError;
use model::object::ObjectMetadata;
use model::resource::Resource;
use model::resource::ResourceEvent;
use model::stream::StreamMetadata;
use model::AppendResultEntry;
use protocol::rpc::header::AppendResponse;
use protocol::rpc::header::CommitObjectResponse;
use protocol::rpc::header::CreateRangeResponse;
use protocol::rpc::header::CreateStreamResponse;
use protocol::rpc::header::DescribePlacementDriverClusterResponse;
use protocol::rpc::header::DescribeStreamResponse;
use protocol::rpc::header::ErrorCode;
use protocol::rpc::header::FetchResponse;
use protocol::rpc::header::HeartbeatResponse;
use protocol::rpc::header::IdAllocationResponse;
use protocol::rpc::header::ListRangeResponse;
use protocol::rpc::header::ListResourceResponse;
use protocol::rpc::header::OperationCode;
use protocol::rpc::header::ReportMetricsResponse;
use protocol::rpc::header::ReportRangeProgressResponse;
use protocol::rpc::header::SealRangeResponse;
use protocol::rpc::header::SystemError;

use model::range::RangeMetadata;
use model::PlacementDriverNode;
use model::Status;
use protocol::rpc::header::UpdateStreamEpochResponse;
use protocol::rpc::header::WatchResourceResponse;

use crate::invocation_context::InvocationContext;

#[derive(Debug, Clone)]
pub struct Response {
    /// The operation code of the response.
    pub operation_code: OperationCode,

    /// Status line
    pub status: Status,

    /// Optional response extension, containing additional operation-code-specific data.
    pub headers: Option<Headers>,

    pub payload: Option<Bytes>,
}

impl From<&Response> for EsError {
    fn from(r: &Response) -> Self {
        EsError::new(r.status.code, &r.status.message)
    }
}

#[derive(Debug, Clone)]
pub enum Headers {
    CreateStream {
        stream: StreamMetadata,
    },

    DescribeStream {
        stream: StreamMetadata,
    },

    ListRange {
        ranges: Option<Vec<RangeMetadata>>,
    },

    AllocateId {
        id: i32,
    },

    DescribePlacementDriver {
        nodes: Option<Vec<PlacementDriverNode>>,
    },

    SealRange {
        range: Option<RangeMetadata>,
    },

    Append {
        entries: Vec<AppendResultEntry>,
    },

    Fetch {
        throttle: Option<std::time::Duration>,
        object_metadata_list: Option<Vec<ObjectMetadata>>,
    },

    CreateRange {
        range: RangeMetadata,
    },

    ListResource {
        resources: Vec<Resource>,
        version: i64,
        continuation: Option<Bytes>,
    },

    WatchResource {
        events: Vec<ResourceEvent>,
        version: i64,
    },
}

impl Response {
    pub fn new(operation_code: OperationCode) -> Self {
        Self {
            operation_code,
            status: Status::decode(),
            headers: None,
            payload: None,
        }
    }

    pub fn ok(&self) -> bool {
        self.status.code == ErrorCode::OK
    }

    pub fn on_system_error(&mut self, frame: &Frame) {
        if let Some(ref buf) = frame.header {
            match flatbuffers::root::<SystemError>(buf) {
                Ok(system_error) => {
                    let system_error = system_error.unpack();
                    self.status = system_error.status.as_ref().into();
                }
                Err(e) => {
                    // Deserialize error
                    warn!(
                        "Failed to decode `SystemError` using FlatBuffers. Cause: {}",
                        e
                    );
                }
            }
        }
    }

    pub fn on_heartbeat(&mut self, frame: &Frame) {
        if let Some(ref buf) = frame.header {
            match flatbuffers::root::<HeartbeatResponse>(buf) {
                Ok(heartbeat) => {
                    trace!("Received Heartbeat response: {:?}", heartbeat);
                    let hb = heartbeat.unpack();
                    let _client_id = hb.client_id;
                    let _client_role = hb.client_role;
                    let _status = hb.status;
                    self.status = _status.as_ref().into();
                }

                Err(e) => {
                    error!("Failed to parse Heartbeat response header: {:?}", e);
                }
            }
        }
    }

    pub fn on_list_ranges(&mut self, frame: &Frame) {
        if let Some(ref buf) = frame.header {
            match flatbuffers::root::<ListRangeResponse>(buf) {
                Ok(response) => {
                    self.status = Into::<Status>::into(&response.status().unpack());
                    if self.status.code != ErrorCode::OK {
                        return;
                    }
                    let range = response
                        .ranges()
                        .iter()
                        .map(|item| Into::<RangeMetadata>::into(&item.unpack()))
                        .collect::<Vec<_>>();
                    self.headers = Some(Headers::ListRange {
                        ranges: Some(range),
                    });
                }
                Err(e) => {
                    error!("Failed to decode `ListRangeResponse` using FlatBuffers. Cause: {e}");
                }
            }
        }
    }

    pub fn on_allocate_id(&mut self, frame: &Frame) {
        if let Some(ref buf) = frame.header {
            match flatbuffers::root::<IdAllocationResponse>(buf) {
                Ok(response) => {
                    if response.status().code() != ErrorCode::OK {
                        self.status = Into::<Status>::into(&response.status().unpack());
                        return;
                    }
                    self.status = Status::ok();
                    self.headers = Some(Headers::AllocateId { id: response.id() });
                }
                Err(e) => {
                    // Deserialize error
                    warn!( "Failed to decode `IdAllocation` response header using FlatBuffers. Cause: {}", e);
                }
            }
        }
    }

    pub fn on_append(&mut self, frame: &Frame, _ctx: &InvocationContext) {
        if let Some(ref buf) = frame.header {
            match flatbuffers::root::<AppendResponse>(buf) {
                Ok(response) => {
                    let response = response.unpack();
                    if response.status.code != ErrorCode::OK {
                        self.status = response.status.as_ref().into();
                        return;
                    }
                    self.status = Status::ok();

                    if let Some(items) = response.entries {
                        let entries = items
                            .into_iter()
                            .map(Into::<AppendResultEntry>::into)
                            .collect();
                        self.headers = Some(Headers::Append { entries });
                    }
                }
                Err(e) => {
                    error!(
                        "Failed to decode AppendResponse using FlatBuffers. Cause: {}",
                        e
                    );
                }
            }
        }
    }

    pub fn on_fetch(&mut self, frame: &Frame, _ctx: &InvocationContext) {
        if let Some(ref buf) = frame.header {
            match flatbuffers::root::<FetchResponse>(buf) {
                Ok(response) => {
                    let response = response.unpack();
                    if response.status.code != ErrorCode::OK {
                        self.status = response.status.as_ref().into();
                        return;
                    }
                    self.status = Status::ok();
                    let throttle = if response.throttle_time_ms < 0 {
                        None
                    } else {
                        Some(std::time::Duration::from_millis(
                            response.throttle_time_ms as u64,
                        ))
                    };
                    let object_metadata_list = response.object_metadata_list.map(|items| {
                        items
                            .iter()
                            .map(Into::<ObjectMetadata>::into)
                            .collect::<Vec<_>>()
                    });
                    self.headers = Some(Headers::Fetch {
                        throttle,
                        object_metadata_list,
                    });
                    self.payload = frame.get_response_payload();
                }
                Err(e) => {
                    error!(
                        "Failed to decode FetchResponse using FlatBuffers. Cause: {}",
                        e
                    );
                }
            }
        }
    }

    pub fn on_create_range(&mut self, frame: &Frame, _ctx: &InvocationContext) {
        if let Some(ref buf) = frame.header {
            match flatbuffers::root::<CreateRangeResponse>(buf) {
                Ok(response) => {
                    self.status = Into::<Status>::into(&response.status().unpack());
                    if self.status.code != ErrorCode::OK {
                        return;
                    }
                    if let Some(range) = response
                        .range()
                        .map(|r| Into::<RangeMetadata>::into(&r.unpack()))
                    {
                        self.headers = Some(Headers::CreateRange { range });
                    }
                }
                Err(e) => {
                    error!(
                        "Failed to decode CreateRangeResponse using FlatBuffers. Cause: {}",
                        e
                    );
                }
            }
        }
    }

    pub fn on_seal_range(&mut self, frame: &Frame, ctx: &InvocationContext) {
        if let Some(ref buf) = frame.header {
            match flatbuffers::root::<SealRangeResponse>(buf) {
                Ok(response) => {
                    self.status = Into::<Status>::into(&response.status().unpack());
                    if self.status.code != ErrorCode::OK {
                        if let request::Headers::SealRange { kind, range } = &ctx.request().headers
                        {
                            warn!(
                                "Seal range failed: seal-kind={:?}, range={:?}, status={:?}",
                                kind, range, self.status
                            );
                        }
                        return;
                    }
                    self.headers = Some(Headers::SealRange {
                        range: response
                            .range()
                            .map(|range| Into::<RangeMetadata>::into(&range.unpack())),
                    });
                }
                Err(e) => {
                    error!(
                        "Failed to decode SealRangesResponse using FlatBuffers. Cause: {}",
                        e
                    );
                }
            }
        }
    }

    pub fn on_report_metrics(&mut self, frame: &Frame) {
        if let Some(ref buf) = frame.header {
            match flatbuffers::root::<ReportMetricsResponse>(buf) {
                Ok(response) => {
                    trace!("Received Report Metrics response: {:?}", response);
                    self.status = Into::<Status>::into(&response.status().unpack());
                }

                Err(e) => {
                    println!("buf = {:?}", buf);
                    error!("Failed to parse Report Metrics response header: {:?}", e);
                }
            }
        }
    }

    pub fn on_describe_placement_driver(&mut self, frame: &Frame) {
        if let Some(ref buf) = frame.header {
            match flatbuffers::root::<DescribePlacementDriverClusterResponse>(buf) {
                Ok(response) => {
                    debug!("Received {response:?}");
                    self.status = Into::<Status>::into(&response.status().unpack());
                    if ErrorCode::OK != self.status.code {
                        return;
                    }

                    let nodes = response
                        .cluster()
                        .unpack()
                        .nodes
                        .iter()
                        .map(Into::into)
                        .collect::<Vec<PlacementDriverNode>>();

                    self.headers = Some(Headers::DescribePlacementDriver { nodes: Some(nodes) });
                }
                Err(_e) => {
                    // Deserialize error
                    warn!( "Failed to decode `DescribePlacementDriverClusterResponse` response header using FlatBuffers. Cause: {}", _e);
                }
            }
        }
    }

    pub(crate) fn on_create_stream(&mut self, frame: &Frame, ctx: &InvocationContext) {
        if let Some(ref buf) = frame.header {
            match flatbuffers::root::<CreateStreamResponse>(buf) {
                Ok(response) => {
                    let status_t = response.status().unpack();
                    self.status = Into::<Status>::into(&status_t);
                    if self.status.code != ErrorCode::OK {
                        warn!("Failed to create stream: {status_t:?}");
                        return;
                    }
                    if let Some(stream) = response.stream() {
                        let metadata = Into::<StreamMetadata>::into(&stream.unpack());
                        info!("Created {:?} on {}", metadata, ctx.target());
                        self.headers = Some(Headers::CreateStream { stream: metadata });
                    } else {
                        // Expected stream metadata is missing
                        self.status = Status::pd_internal(
                            "Required stream is missing even if status is OK".to_owned(),
                        );
                        error!("Required stream field is missing in CreateStreamResponse even if status is OK");
                    }
                }
                Err(e) => {
                    error!(
                        "Failed to decode CreateStreamResponse using FlatBuffers. Cause: {}",
                        e
                    );
                }
            }
        }
    }

    pub(crate) fn on_describe_stream(&mut self, frame: &Frame, ctx: &InvocationContext) {
        if let Some(ref buf) = frame.header {
            match flatbuffers::root::<DescribeStreamResponse>(buf) {
                Ok(response) => {
                    self.status = Into::<Status>::into(&response.status().unpack());
                    if self.status.code != ErrorCode::OK {
                        return;
                    }
                    if let Some(stream) = response.stream() {
                        let metadata = Into::<StreamMetadata>::into(&stream.unpack());
                        debug!("Describe stream={:?} on {}", metadata, ctx.target());
                        self.headers = Some(Headers::DescribeStream { stream: metadata });
                    } else {
                        // Expected stream metadata is missing
                        self.status = Status::pd_internal(
                            "Stream is missing even if status is OK".to_owned(),
                        );
                        error!("DescribeStreamResponse missed required stream field even if status is OK");
                    }
                }
                Err(e) => {
                    error!(
                        "Failed to decode DescribeStreamResponse using FlatBuffers. Cause: {}",
                        e
                    );
                }
            }
        }
    }

    pub(crate) fn on_update_stream_epoch(&mut self, frame: &Frame, _ctx: &InvocationContext) {
        if let Some(ref buf) = frame.header {
            match flatbuffers::root::<UpdateStreamEpochResponse>(buf) {
                Ok(response) => {
                    self.status = Into::<Status>::into(&response.status().unpack());
                }
                Err(e) => {
                    error!(
                        "Failed to decode UpdateStreamEpoch using FlatBuffers. Cause: {}",
                        e
                    );
                }
            }
        }
    }

    pub fn on_report_replica_progress(&mut self, frame: &Frame) {
        if let Some(ref buf) = frame.header {
            match flatbuffers::root::<ReportRangeProgressResponse>(buf) {
                Ok(response) => {
                    trace!("Received Report replica progress response: {:?}", response);
                    self.status = Into::<Status>::into(&response.status().unpack());
                }

                Err(e) => {
                    error!(
                        "Failed to parse Report replica progress response header: {:?}",
                        e
                    );
                }
            }
        }
    }

    pub fn on_commit_object(&mut self, frame: &Frame) {
        if let Some(buf) = frame.header.as_ref() {
            match flatbuffers::root::<CommitObjectResponse>(buf) {
                Ok(response) => {
                    trace!("Received Commit object response: {:?}", response);
                    self.status = Into::<Status>::into(&response.status().unpack());
                }

                Err(e) => {
                    error!("Failed to parse Commit object response header: {:?}", e);
                }
            }
        }
    }

    pub fn on_list_resource(&mut self, frame: &Frame) {
        if let Some(buf) = frame.header.as_ref() {
            match flatbuffers::root::<ListResourceResponse>(buf) {
                Ok(response) => {
                    trace!("Received List resource response: {:?}", response);
                    self.status = Into::<Status>::into(&response.status().unpack());
                    if self.status.code == ErrorCode::OK {
                        self.headers = Some(Headers::ListResource {
                            resources: response
                                .resources()
                                .iter()
                                .map(|r| r.unpack())
                                .map(|rt| Resource::from(&rt))
                                .collect(),
                            version: response.resource_version(),
                            continuation: response
                                .continuation()
                                .map(|c| c.bytes())
                                .map(Bytes::copy_from_slice),
                        })
                    }
                }

                Err(e) => {
                    error!("Failed to parse List resource response header: {:?}", e);
                }
            }
        }
    }

    pub fn on_watch_resource(&mut self, frame: &Frame) {
        if let Some(buf) = frame.header.as_ref() {
            match flatbuffers::root::<WatchResourceResponse>(buf) {
                Ok(response) => {
                    trace!("Received Watch resource response: {:?}", response);
                    self.status = Into::<Status>::into(&response.status().unpack());
                    if self.status.code == ErrorCode::OK {
                        self.headers = Some(Headers::WatchResource {
                            events: response
                                .events()
                                .iter()
                                .map(|e| e.unpack())
                                .map(|et| ResourceEvent::from(&et))
                                .collect(),
                            version: response.resource_version(),
                        })
                    }
                }

                Err(e) => {
                    error!("Failed to parse Watch resource response header: {:?}", e);
                }
            }
        }
    }
}
