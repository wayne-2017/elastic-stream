// automatically generated by the FlatBuffers compiler, do not modify


// @generated

use core::mem;
use core::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

#[allow(unused_imports, dead_code)]
pub mod elastic {

  use core::mem;
  use core::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::{EndianScalar, Follow};
#[allow(unused_imports, dead_code)]
pub mod store {

  use core::mem;
  use core::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::{EndianScalar, Follow};

pub enum HeartbeatOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct Heartbeat<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Heartbeat<'a> {
  type Inner = Heartbeat<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> Heartbeat<'a> {
  pub const VT_CLIENT_ID: flatbuffers::VOffsetT = 4;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    Heartbeat { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args HeartbeatArgs<'args>
  ) -> flatbuffers::WIPOffset<Heartbeat<'bldr>> {
    let mut builder = HeartbeatBuilder::new(_fbb);
    if let Some(x) = args.client_id { builder.add_client_id(x); }
    builder.finish()
  }


  #[inline]
  pub fn client_id(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(Heartbeat::VT_CLIENT_ID, None)}
  }
}

impl flatbuffers::Verifiable for Heartbeat<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("client_id", Self::VT_CLIENT_ID, false)?
     .finish();
    Ok(())
  }
}
pub struct HeartbeatArgs<'a> {
    pub client_id: Option<flatbuffers::WIPOffset<&'a str>>,
}
impl<'a> Default for HeartbeatArgs<'a> {
  #[inline]
  fn default() -> Self {
    HeartbeatArgs {
      client_id: None,
    }
  }
}

pub struct HeartbeatBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> HeartbeatBuilder<'a, 'b> {
  #[inline]
  pub fn add_client_id(&mut self, client_id: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Heartbeat::VT_CLIENT_ID, client_id);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> HeartbeatBuilder<'a, 'b> {
    let start = _fbb.start_table();
    HeartbeatBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Heartbeat<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for Heartbeat<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("Heartbeat");
      ds.field("client_id", &self.client_id());
      ds.finish()
  }
}
pub enum ListRangeOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct ListRange<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for ListRange<'a> {
  type Inner = ListRange<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> ListRange<'a> {
  pub const VT_PARTITION_ID: flatbuffers::VOffsetT = 4;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    ListRange { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args ListRangeArgs
  ) -> flatbuffers::WIPOffset<ListRange<'bldr>> {
    let mut builder = ListRangeBuilder::new(_fbb);
    builder.add_partition_id(args.partition_id);
    builder.finish()
  }


  #[inline]
  pub fn partition_id(&self) -> i64 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<i64>(ListRange::VT_PARTITION_ID, Some(0)).unwrap()}
  }
}

impl flatbuffers::Verifiable for ListRange<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<i64>("partition_id", Self::VT_PARTITION_ID, false)?
     .finish();
    Ok(())
  }
}
pub struct ListRangeArgs {
    pub partition_id: i64,
}
impl<'a> Default for ListRangeArgs {
  #[inline]
  fn default() -> Self {
    ListRangeArgs {
      partition_id: 0,
    }
  }
}

pub struct ListRangeBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> ListRangeBuilder<'a, 'b> {
  #[inline]
  pub fn add_partition_id(&mut self, partition_id: i64) {
    self.fbb_.push_slot::<i64>(ListRange::VT_PARTITION_ID, partition_id, 0);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> ListRangeBuilder<'a, 'b> {
    let start = _fbb.start_table();
    ListRangeBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<ListRange<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for ListRange<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("ListRange");
      ds.field("partition_id", &self.partition_id());
      ds.finish()
  }
}
#[inline]
/// Verifies that a buffer of bytes contains a `ListRange`
/// and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_list_range_unchecked`.
pub fn root_as_list_range(buf: &[u8]) -> Result<ListRange, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root::<ListRange>(buf)
}
#[inline]
/// Verifies that a buffer of bytes contains a size prefixed
/// `ListRange` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `size_prefixed_root_as_list_range_unchecked`.
pub fn size_prefixed_root_as_list_range(buf: &[u8]) -> Result<ListRange, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root::<ListRange>(buf)
}
#[inline]
/// Verifies, with the given options, that a buffer of bytes
/// contains a `ListRange` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_list_range_unchecked`.
pub fn root_as_list_range_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<ListRange<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root_with_opts::<ListRange<'b>>(opts, buf)
}
#[inline]
/// Verifies, with the given verifier options, that a buffer of
/// bytes contains a size prefixed `ListRange` and returns
/// it. Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_list_range_unchecked`.
pub fn size_prefixed_root_as_list_range_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<ListRange<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root_with_opts::<ListRange<'b>>(opts, buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a ListRange and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid `ListRange`.
pub unsafe fn root_as_list_range_unchecked(buf: &[u8]) -> ListRange {
  flatbuffers::root_unchecked::<ListRange>(buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a size prefixed ListRange and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid size prefixed `ListRange`.
pub unsafe fn size_prefixed_root_as_list_range_unchecked(buf: &[u8]) -> ListRange {
  flatbuffers::size_prefixed_root_unchecked::<ListRange>(buf)
}
#[inline]
pub fn finish_list_range_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<ListRange<'a>>) {
  fbb.finish(root, None);
}

#[inline]
pub fn finish_size_prefixed_list_range_buffer<'a, 'b>(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>, root: flatbuffers::WIPOffset<ListRange<'a>>) {
  fbb.finish_size_prefixed(root, None);
}
}  // pub mod Store
}  // pub mod Elastic
