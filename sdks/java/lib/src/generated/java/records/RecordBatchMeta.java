// automatically generated by the FlatBuffers compiler, do not modify

package records;

import com.google.flatbuffers.BaseVector;
import com.google.flatbuffers.BooleanVector;
import com.google.flatbuffers.ByteVector;
import com.google.flatbuffers.Constants;
import com.google.flatbuffers.DoubleVector;
import com.google.flatbuffers.FlatBufferBuilder;
import com.google.flatbuffers.FloatVector;
import com.google.flatbuffers.IntVector;
import com.google.flatbuffers.LongVector;
import com.google.flatbuffers.ShortVector;
import com.google.flatbuffers.StringVector;
import com.google.flatbuffers.Struct;
import com.google.flatbuffers.Table;
import com.google.flatbuffers.UnionVector;
import java.nio.ByteBuffer;
import java.nio.ByteOrder;

@SuppressWarnings("unused")
public final class RecordBatchMeta extends Table {
  public static void ValidateVersion() { Constants.FLATBUFFERS_23_1_21(); }
  public static RecordBatchMeta getRootAsRecordBatchMeta(ByteBuffer _bb) { return getRootAsRecordBatchMeta(_bb, new RecordBatchMeta()); }
  public static RecordBatchMeta getRootAsRecordBatchMeta(ByteBuffer _bb, RecordBatchMeta obj) { _bb.order(ByteOrder.LITTLE_ENDIAN); return (obj.__assign(_bb.getInt(_bb.position()) + _bb.position(), _bb)); }
  public void __init(int _i, ByteBuffer _bb) { __reset(_i, _bb); }
  public RecordBatchMeta __assign(int _i, ByteBuffer _bb) { __init(_i, _bb); return this; }

  /**
   * The stream id of this record batch.
   */
  public long streamId() { int o = __offset(4); return o != 0 ? bb.getLong(o + bb_pos) : 0L; }
  /**
   * The flags of this record batch. Each bit is used to indicate a specific flag.
   */
  public short flags() { int o = __offset(6); return o != 0 ? bb.getShort(o + bb_pos) : 0; }
  /**
   * The base offset of the batch record, also is the logical offset of the first record.
   * This field may be empty, since rust flatbuffers doesn't support update in place.
   */
  public long baseOffset() { int o = __offset(8); return o != 0 ? bb.getLong(o + bb_pos) : 0L; }
  /**
   * The delta value between the last offset and the base offset.
   */
  public int lastOffsetDelta() { int o = __offset(10); return o != 0 ? bb.getInt(o + bb_pos) : 0; }
  /**
   * The create timestap of the first record in this batch.
   */
  public long baseTimestamp() { int o = __offset(12); return o != 0 ? bb.getLong(o + bb_pos) : 0L; }

  public static int createRecordBatchMeta(FlatBufferBuilder builder,
      long streamId,
      short flags,
      long baseOffset,
      int lastOffsetDelta,
      long baseTimestamp) {
    builder.startTable(5);
    RecordBatchMeta.addBaseTimestamp(builder, baseTimestamp);
    RecordBatchMeta.addBaseOffset(builder, baseOffset);
    RecordBatchMeta.addStreamId(builder, streamId);
    RecordBatchMeta.addLastOffsetDelta(builder, lastOffsetDelta);
    RecordBatchMeta.addFlags(builder, flags);
    return RecordBatchMeta.endRecordBatchMeta(builder);
  }

  public static void startRecordBatchMeta(FlatBufferBuilder builder) { builder.startTable(5); }
  public static void addStreamId(FlatBufferBuilder builder, long streamId) { builder.addLong(0, streamId, 0L); }
  public static void addFlags(FlatBufferBuilder builder, short flags) { builder.addShort(1, flags, 0); }
  public static void addBaseOffset(FlatBufferBuilder builder, long baseOffset) { builder.addLong(2, baseOffset, 0L); }
  public static void addLastOffsetDelta(FlatBufferBuilder builder, int lastOffsetDelta) { builder.addInt(3, lastOffsetDelta, 0); }
  public static void addBaseTimestamp(FlatBufferBuilder builder, long baseTimestamp) { builder.addLong(4, baseTimestamp, 0L); }
  public static int endRecordBatchMeta(FlatBufferBuilder builder) {
    int o = builder.endTable();
    return o;
  }

  public static final class Vector extends BaseVector {
    public Vector __assign(int _vector, int _element_size, ByteBuffer _bb) { __reset(_vector, _element_size, _bb); return this; }

    public RecordBatchMeta get(int j) { return get(new RecordBatchMeta(), j); }
    public RecordBatchMeta get(RecordBatchMeta obj, int j) {  return obj.__assign(__indirect(__element(j), bb), bb); }
  }
  public RecordBatchMetaT unpack() {
    RecordBatchMetaT _o = new RecordBatchMetaT();
    unpackTo(_o);
    return _o;
  }
  public void unpackTo(RecordBatchMetaT _o) {
    long _oStreamId = streamId();
    _o.setStreamId(_oStreamId);
    short _oFlags = flags();
    _o.setFlags(_oFlags);
    long _oBaseOffset = baseOffset();
    _o.setBaseOffset(_oBaseOffset);
    int _oLastOffsetDelta = lastOffsetDelta();
    _o.setLastOffsetDelta(_oLastOffsetDelta);
    long _oBaseTimestamp = baseTimestamp();
    _o.setBaseTimestamp(_oBaseTimestamp);
  }
  public static int pack(FlatBufferBuilder builder, RecordBatchMetaT _o) {
    if (_o == null) return 0;
    return createRecordBatchMeta(
      builder,
      _o.getStreamId(),
      _o.getFlags(),
      _o.getBaseOffset(),
      _o.getLastOffsetDelta(),
      _o.getBaseTimestamp());
  }
}

