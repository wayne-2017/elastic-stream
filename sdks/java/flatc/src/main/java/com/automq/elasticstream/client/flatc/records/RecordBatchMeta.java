// automatically generated by the FlatBuffers compiler, do not modify

package com.automq.elasticstream.client.flatc.records;

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
  public static void ValidateVersion() { Constants.FLATBUFFERS_23_3_3(); }
  public static RecordBatchMeta getRootAsRecordBatchMeta(ByteBuffer _bb) { return getRootAsRecordBatchMeta(_bb, new RecordBatchMeta()); }
  public static RecordBatchMeta getRootAsRecordBatchMeta(ByteBuffer _bb, RecordBatchMeta obj) { _bb.order(ByteOrder.LITTLE_ENDIAN); return (obj.__assign(_bb.getInt(_bb.position()) + _bb.position(), _bb)); }
  public void __init(int _i, ByteBuffer _bb) { __reset(_i, _bb); }
  public RecordBatchMeta __assign(int _i, ByteBuffer _bb) { __init(_i, _bb); return this; }

  /**
   * The stream id of this record batch.
   */
  public long streamId() { int o = __offset(4); return o != 0 ? bb.getLong(o + bb_pos) : 0L; }
  /**
   * The range index of this record batch.
   */
  public int rangeIndex() { int o = __offset(6); return o != 0 ? bb.getInt(o + bb_pos) : 0; }
  /**
   * The flags of this record batch. Each bit is used to indicate a specific flag.
   */
  public short flags() { int o = __offset(8); return o != 0 ? bb.getShort(o + bb_pos) : 0; }
  /**
   * The base offset of the batch record, also is the logical offset of the first record.
   * This field may be empty, since rust flatbuffers doesn't support update in place.
   */
  public long baseOffset() { int o = __offset(10); return o != 0 ? bb.getLong(o + bb_pos) : 0L; }
  /**
   * The delta value between the last offset and the base offset.
   */
  public int lastOffsetDelta() { int o = __offset(12); return o != 0 ? bb.getInt(o + bb_pos) : 0; }
  /**
   * The create timestamp of the first record in this batch.
   */
  public long baseTimestamp() { int o = __offset(14); return o != 0 ? bb.getLong(o + bb_pos) : 0L; }
  /**
   * Other attributes that may not be corresponding with the storage layer.
   */
  public com.automq.elasticstream.client.flatc.records.KeyValue properties(int j) { return properties(new com.automq.elasticstream.client.flatc.records.KeyValue(), j); }
  public com.automq.elasticstream.client.flatc.records.KeyValue properties(com.automq.elasticstream.client.flatc.records.KeyValue obj, int j) { int o = __offset(16); return o != 0 ? obj.__assign(__indirect(__vector(o) + j * 4), bb) : null; }
  public int propertiesLength() { int o = __offset(16); return o != 0 ? __vector_len(o) : 0; }
  public com.automq.elasticstream.client.flatc.records.KeyValue.Vector propertiesVector() { return propertiesVector(new com.automq.elasticstream.client.flatc.records.KeyValue.Vector()); }
  public com.automq.elasticstream.client.flatc.records.KeyValue.Vector propertiesVector(com.automq.elasticstream.client.flatc.records.KeyValue.Vector obj) { int o = __offset(16); return o != 0 ? obj.__assign(__vector(o), 4, bb) : null; }

  public static int createRecordBatchMeta(FlatBufferBuilder builder,
      long streamId,
      int rangeIndex,
      short flags,
      long baseOffset,
      int lastOffsetDelta,
      long baseTimestamp,
      int propertiesOffset) {
    builder.startTable(7);
    RecordBatchMeta.addBaseTimestamp(builder, baseTimestamp);
    RecordBatchMeta.addBaseOffset(builder, baseOffset);
    RecordBatchMeta.addStreamId(builder, streamId);
    RecordBatchMeta.addProperties(builder, propertiesOffset);
    RecordBatchMeta.addLastOffsetDelta(builder, lastOffsetDelta);
    RecordBatchMeta.addRangeIndex(builder, rangeIndex);
    RecordBatchMeta.addFlags(builder, flags);
    return RecordBatchMeta.endRecordBatchMeta(builder);
  }

  public static void startRecordBatchMeta(FlatBufferBuilder builder) { builder.startTable(7); }
  public static void addStreamId(FlatBufferBuilder builder, long streamId) { builder.addLong(0, streamId, 0L); }
  public static void addRangeIndex(FlatBufferBuilder builder, int rangeIndex) { builder.addInt(1, rangeIndex, 0); }
  public static void addFlags(FlatBufferBuilder builder, short flags) { builder.addShort(2, flags, 0); }
  public static void addBaseOffset(FlatBufferBuilder builder, long baseOffset) { builder.addLong(3, baseOffset, 0L); }
  public static void addLastOffsetDelta(FlatBufferBuilder builder, int lastOffsetDelta) { builder.addInt(4, lastOffsetDelta, 0); }
  public static void addBaseTimestamp(FlatBufferBuilder builder, long baseTimestamp) { builder.addLong(5, baseTimestamp, 0L); }
  public static void addProperties(FlatBufferBuilder builder, int propertiesOffset) { builder.addOffset(6, propertiesOffset, 0); }
  public static int createPropertiesVector(FlatBufferBuilder builder, int[] data) { builder.startVector(4, data.length, 4); for (int i = data.length - 1; i >= 0; i--) builder.addOffset(data[i]); return builder.endVector(); }
  public static void startPropertiesVector(FlatBufferBuilder builder, int numElems) { builder.startVector(4, numElems, 4); }
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
    int _oRangeIndex = rangeIndex();
    _o.setRangeIndex(_oRangeIndex);
    short _oFlags = flags();
    _o.setFlags(_oFlags);
    long _oBaseOffset = baseOffset();
    _o.setBaseOffset(_oBaseOffset);
    int _oLastOffsetDelta = lastOffsetDelta();
    _o.setLastOffsetDelta(_oLastOffsetDelta);
    long _oBaseTimestamp = baseTimestamp();
    _o.setBaseTimestamp(_oBaseTimestamp);
    com.automq.elasticstream.client.flatc.records.KeyValueT[] _oProperties = new com.automq.elasticstream.client.flatc.records.KeyValueT[propertiesLength()];
    for (int _j = 0; _j < propertiesLength(); ++_j) {_oProperties[_j] = (properties(_j) != null ? properties(_j).unpack() : null);}
    _o.setProperties(_oProperties);
  }
  public static int pack(FlatBufferBuilder builder, RecordBatchMetaT _o) {
    if (_o == null) return 0;
    int _properties = 0;
    if (_o.getProperties() != null) {
      int[] __properties = new int[_o.getProperties().length];
      int _j = 0;
      for (com.automq.elasticstream.client.flatc.records.KeyValueT _e : _o.getProperties()) { __properties[_j] = com.automq.elasticstream.client.flatc.records.KeyValue.pack(builder, _e); _j++;}
      _properties = createPropertiesVector(builder, __properties);
    }
    return createRecordBatchMeta(
      builder,
      _o.getStreamId(),
      _o.getRangeIndex(),
      _o.getFlags(),
      _o.getBaseOffset(),
      _o.getLastOffsetDelta(),
      _o.getBaseTimestamp(),
      _properties);
  }
}
