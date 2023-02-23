// automatically generated by the FlatBuffers compiler, do not modify

// @generated

use core::cmp::Ordering;
use core::mem;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

pub enum RecordBatchMetaOffset {}
#[derive(Copy, Clone, PartialEq)]

/// Don't delete any field from the schema once released.
/// Asign a id for each filed to keep compatibility easily.
pub struct RecordBatchMeta<'a> {
    pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for RecordBatchMeta<'a> {
    type Inner = RecordBatchMeta<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table::new(buf, loc),
        }
    }
}

impl<'a> RecordBatchMeta<'a> {
    pub const VT_STREAM_ID: flatbuffers::VOffsetT = 4;
    pub const VT_FLAGS: flatbuffers::VOffsetT = 6;
    pub const VT_BASE_OFFSET: flatbuffers::VOffsetT = 8;
    pub const VT_LAST_OFFSET_DELTA: flatbuffers::VOffsetT = 10;
    pub const VT_BASE_TIMESTAMP: flatbuffers::VOffsetT = 12;

    #[inline]
    pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        RecordBatchMeta { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args RecordBatchMetaArgs,
    ) -> flatbuffers::WIPOffset<RecordBatchMeta<'bldr>> {
        let mut builder = RecordBatchMetaBuilder::new(_fbb);
        builder.add_base_timestamp(args.base_timestamp);
        builder.add_base_offset(args.base_offset);
        builder.add_stream_id(args.stream_id);
        builder.add_last_offset_delta(args.last_offset_delta);
        builder.add_flags(args.flags);
        builder.finish()
    }

    /// The stream id of this record batch.
    #[inline]
    pub fn stream_id(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(RecordBatchMeta::VT_STREAM_ID, Some(0))
                .unwrap()
        }
    }
    /// The flags of this record batch. Each bit is used to indicate a specific flag.
    #[inline]
    pub fn flags(&self) -> i16 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i16>(RecordBatchMeta::VT_FLAGS, Some(0))
                .unwrap()
        }
    }
    /// The base offset of the batch record, also is the logical offset of the first record.
    /// This field may be empty, since rust flatbuffers doesn't support update in place.
    #[inline]
    pub fn base_offset(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(RecordBatchMeta::VT_BASE_OFFSET, Some(0))
                .unwrap()
        }
    }
    /// The delta value between the last offset and the base offset.
    #[inline]
    pub fn last_offset_delta(&self) -> i32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i32>(RecordBatchMeta::VT_LAST_OFFSET_DELTA, Some(0))
                .unwrap()
        }
    }
    /// The create timestap of the first record in this batch.
    #[inline]
    pub fn base_timestamp(&self) -> i64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i64>(RecordBatchMeta::VT_BASE_TIMESTAMP, Some(0))
                .unwrap()
        }
    }
}

impl flatbuffers::Verifiable for RecordBatchMeta<'_> {
    #[inline]
    fn run_verifier(
        v: &mut flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
        use self::flatbuffers::Verifiable;
        v.visit_table(pos)?
            .visit_field::<i64>("stream_id", Self::VT_STREAM_ID, false)?
            .visit_field::<i16>("flags", Self::VT_FLAGS, false)?
            .visit_field::<i64>("base_offset", Self::VT_BASE_OFFSET, false)?
            .visit_field::<i32>("last_offset_delta", Self::VT_LAST_OFFSET_DELTA, false)?
            .visit_field::<i64>("base_timestamp", Self::VT_BASE_TIMESTAMP, false)?
            .finish();
        Ok(())
    }
}
pub struct RecordBatchMetaArgs {
    pub stream_id: i64,
    pub flags: i16,
    pub base_offset: i64,
    pub last_offset_delta: i32,
    pub base_timestamp: i64,
}
impl<'a> Default for RecordBatchMetaArgs {
    #[inline]
    fn default() -> Self {
        RecordBatchMetaArgs {
            stream_id: 0,
            flags: 0,
            base_offset: 0,
            last_offset_delta: 0,
            base_timestamp: 0,
        }
    }
}

pub struct RecordBatchMetaBuilder<'a: 'b, 'b> {
    fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> RecordBatchMetaBuilder<'a, 'b> {
    #[inline]
    pub fn add_stream_id(&mut self, stream_id: i64) {
        self.fbb_
            .push_slot::<i64>(RecordBatchMeta::VT_STREAM_ID, stream_id, 0);
    }
    #[inline]
    pub fn add_flags(&mut self, flags: i16) {
        self.fbb_
            .push_slot::<i16>(RecordBatchMeta::VT_FLAGS, flags, 0);
    }
    #[inline]
    pub fn add_base_offset(&mut self, base_offset: i64) {
        self.fbb_
            .push_slot::<i64>(RecordBatchMeta::VT_BASE_OFFSET, base_offset, 0);
    }
    #[inline]
    pub fn add_last_offset_delta(&mut self, last_offset_delta: i32) {
        self.fbb_
            .push_slot::<i32>(RecordBatchMeta::VT_LAST_OFFSET_DELTA, last_offset_delta, 0);
    }
    #[inline]
    pub fn add_base_timestamp(&mut self, base_timestamp: i64) {
        self.fbb_
            .push_slot::<i64>(RecordBatchMeta::VT_BASE_TIMESTAMP, base_timestamp, 0);
    }
    #[inline]
    pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> RecordBatchMetaBuilder<'a, 'b> {
        let start = _fbb.start_table();
        RecordBatchMetaBuilder {
            fbb_: _fbb,
            start_: start,
        }
    }
    #[inline]
    pub fn finish(self) -> flatbuffers::WIPOffset<RecordBatchMeta<'a>> {
        let o = self.fbb_.end_table(self.start_);
        flatbuffers::WIPOffset::new(o.value())
    }
}

impl core::fmt::Debug for RecordBatchMeta<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut ds = f.debug_struct("RecordBatchMeta");
        ds.field("stream_id", &self.stream_id());
        ds.field("flags", &self.flags());
        ds.field("base_offset", &self.base_offset());
        ds.field("last_offset_delta", &self.last_offset_delta());
        ds.field("base_timestamp", &self.base_timestamp());
        ds.finish()
    }
}
pub enum KeyValueOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct KeyValue<'a> {
    pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for KeyValue<'a> {
    type Inner = KeyValue<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table::new(buf, loc),
        }
    }
}

impl<'a> KeyValue<'a> {
    pub const VT_KEY: flatbuffers::VOffsetT = 4;
    pub const VT_VALUE: flatbuffers::VOffsetT = 6;

    #[inline]
    pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        KeyValue { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args KeyValueArgs<'args>,
    ) -> flatbuffers::WIPOffset<KeyValue<'bldr>> {
        let mut builder = KeyValueBuilder::new(_fbb);
        if let Some(x) = args.value {
            builder.add_value(x);
        }
        if let Some(x) = args.key {
            builder.add_key(x);
        }
        builder.finish()
    }

    #[inline]
    pub fn key(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<flatbuffers::ForwardsUOffset<&str>>(KeyValue::VT_KEY, None)
        }
    }
    #[inline]
    pub fn value(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<flatbuffers::ForwardsUOffset<&str>>(KeyValue::VT_VALUE, None)
        }
    }
}

impl flatbuffers::Verifiable for KeyValue<'_> {
    #[inline]
    fn run_verifier(
        v: &mut flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
        use self::flatbuffers::Verifiable;
        v.visit_table(pos)?
            .visit_field::<flatbuffers::ForwardsUOffset<&str>>("key", Self::VT_KEY, false)?
            .visit_field::<flatbuffers::ForwardsUOffset<&str>>("value", Self::VT_VALUE, false)?
            .finish();
        Ok(())
    }
}
pub struct KeyValueArgs<'a> {
    pub key: Option<flatbuffers::WIPOffset<&'a str>>,
    pub value: Option<flatbuffers::WIPOffset<&'a str>>,
}
impl<'a> Default for KeyValueArgs<'a> {
    #[inline]
    fn default() -> Self {
        KeyValueArgs {
            key: None,
            value: None,
        }
    }
}

pub struct KeyValueBuilder<'a: 'b, 'b> {
    fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> KeyValueBuilder<'a, 'b> {
    #[inline]
    pub fn add_key(&mut self, key: flatbuffers::WIPOffset<&'b str>) {
        self.fbb_
            .push_slot_always::<flatbuffers::WIPOffset<_>>(KeyValue::VT_KEY, key);
    }
    #[inline]
    pub fn add_value(&mut self, value: flatbuffers::WIPOffset<&'b str>) {
        self.fbb_
            .push_slot_always::<flatbuffers::WIPOffset<_>>(KeyValue::VT_VALUE, value);
    }
    #[inline]
    pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> KeyValueBuilder<'a, 'b> {
        let start = _fbb.start_table();
        KeyValueBuilder {
            fbb_: _fbb,
            start_: start,
        }
    }
    #[inline]
    pub fn finish(self) -> flatbuffers::WIPOffset<KeyValue<'a>> {
        let o = self.fbb_.end_table(self.start_);
        flatbuffers::WIPOffset::new(o.value())
    }
}

impl core::fmt::Debug for KeyValue<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut ds = f.debug_struct("KeyValue");
        ds.field("key", &self.key());
        ds.field("value", &self.value());
        ds.finish()
    }
}
pub enum RecordMetaOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct RecordMeta<'a> {
    pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for RecordMeta<'a> {
    type Inner = RecordMeta<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table::new(buf, loc),
        }
    }
}

impl<'a> RecordMeta<'a> {
    pub const VT_OFFSET_DELTA: flatbuffers::VOffsetT = 4;
    pub const VT_TIMESTAMP_DELTA: flatbuffers::VOffsetT = 6;
    pub const VT_HEADERS: flatbuffers::VOffsetT = 8;
    pub const VT_PROPERTIES: flatbuffers::VOffsetT = 10;

    #[inline]
    pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        RecordMeta { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args RecordMetaArgs<'args>,
    ) -> flatbuffers::WIPOffset<RecordMeta<'bldr>> {
        let mut builder = RecordMetaBuilder::new(_fbb);
        if let Some(x) = args.properties {
            builder.add_properties(x);
        }
        if let Some(x) = args.headers {
            builder.add_headers(x);
        }
        builder.add_timestamp_delta(args.timestamp_delta);
        builder.add_offset_delta(args.offset_delta);
        builder.finish()
    }

    #[inline]
    pub fn offset_delta(&self) -> i32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i32>(RecordMeta::VT_OFFSET_DELTA, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn timestamp_delta(&self) -> i32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<i32>(RecordMeta::VT_TIMESTAMP_DELTA, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn headers(
        &self,
    ) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<KeyValue<'a>>>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<flatbuffers::ForwardsUOffset<
                flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<KeyValue>>,
            >>(RecordMeta::VT_HEADERS, None)
        }
    }
    #[inline]
    pub fn properties(
        &self,
    ) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<KeyValue<'a>>>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<flatbuffers::ForwardsUOffset<
                flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<KeyValue>>,
            >>(RecordMeta::VT_PROPERTIES, None)
        }
    }
}

impl flatbuffers::Verifiable for RecordMeta<'_> {
    #[inline]
    fn run_verifier(
        v: &mut flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
        use self::flatbuffers::Verifiable;
        v.visit_table(pos)?
            .visit_field::<i32>("offset_delta", Self::VT_OFFSET_DELTA, false)?
            .visit_field::<i32>("timestamp_delta", Self::VT_TIMESTAMP_DELTA, false)?
            .visit_field::<flatbuffers::ForwardsUOffset<
                flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<KeyValue>>,
            >>("headers", Self::VT_HEADERS, false)?
            .visit_field::<flatbuffers::ForwardsUOffset<
                flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<KeyValue>>,
            >>("properties", Self::VT_PROPERTIES, false)?
            .finish();
        Ok(())
    }
}
pub struct RecordMetaArgs<'a> {
    pub offset_delta: i32,
    pub timestamp_delta: i32,
    pub headers: Option<
        flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<KeyValue<'a>>>>,
    >,
    pub properties: Option<
        flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<KeyValue<'a>>>>,
    >,
}
impl<'a> Default for RecordMetaArgs<'a> {
    #[inline]
    fn default() -> Self {
        RecordMetaArgs {
            offset_delta: 0,
            timestamp_delta: 0,
            headers: None,
            properties: None,
        }
    }
}

pub struct RecordMetaBuilder<'a: 'b, 'b> {
    fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> RecordMetaBuilder<'a, 'b> {
    #[inline]
    pub fn add_offset_delta(&mut self, offset_delta: i32) {
        self.fbb_
            .push_slot::<i32>(RecordMeta::VT_OFFSET_DELTA, offset_delta, 0);
    }
    #[inline]
    pub fn add_timestamp_delta(&mut self, timestamp_delta: i32) {
        self.fbb_
            .push_slot::<i32>(RecordMeta::VT_TIMESTAMP_DELTA, timestamp_delta, 0);
    }
    #[inline]
    pub fn add_headers(
        &mut self,
        headers: flatbuffers::WIPOffset<
            flatbuffers::Vector<'b, flatbuffers::ForwardsUOffset<KeyValue<'b>>>,
        >,
    ) {
        self.fbb_
            .push_slot_always::<flatbuffers::WIPOffset<_>>(RecordMeta::VT_HEADERS, headers);
    }
    #[inline]
    pub fn add_properties(
        &mut self,
        properties: flatbuffers::WIPOffset<
            flatbuffers::Vector<'b, flatbuffers::ForwardsUOffset<KeyValue<'b>>>,
        >,
    ) {
        self.fbb_
            .push_slot_always::<flatbuffers::WIPOffset<_>>(RecordMeta::VT_PROPERTIES, properties);
    }
    #[inline]
    pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> RecordMetaBuilder<'a, 'b> {
        let start = _fbb.start_table();
        RecordMetaBuilder {
            fbb_: _fbb,
            start_: start,
        }
    }
    #[inline]
    pub fn finish(self) -> flatbuffers::WIPOffset<RecordMeta<'a>> {
        let o = self.fbb_.end_table(self.start_);
        flatbuffers::WIPOffset::new(o.value())
    }
}

impl core::fmt::Debug for RecordMeta<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut ds = f.debug_struct("RecordMeta");
        ds.field("offset_delta", &self.offset_delta());
        ds.field("timestamp_delta", &self.timestamp_delta());
        ds.field("headers", &self.headers());
        ds.field("properties", &self.properties());
        ds.finish()
    }
}
