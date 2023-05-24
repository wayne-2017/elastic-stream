// automatically generated by the FlatBuffers compiler, do not modify

package com.automq.elasticstream.client.flatc.header;

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
public final class AppendResultEntry extends Table {
  public static void ValidateVersion() { Constants.FLATBUFFERS_23_3_3(); }
  public static AppendResultEntry getRootAsAppendResultEntry(ByteBuffer _bb) { return getRootAsAppendResultEntry(_bb, new AppendResultEntry()); }
  public static AppendResultEntry getRootAsAppendResultEntry(ByteBuffer _bb, AppendResultEntry obj) { _bb.order(ByteOrder.LITTLE_ENDIAN); return (obj.__assign(_bb.getInt(_bb.position()) + _bb.position(), _bb)); }
  public void __init(int _i, ByteBuffer _bb) { __reset(_i, _bb); }
  public AppendResultEntry __assign(int _i, ByteBuffer _bb) { __init(_i, _bb); return this; }

  public com.automq.elasticstream.client.flatc.header.Status status() { return status(new com.automq.elasticstream.client.flatc.header.Status()); }
  public com.automq.elasticstream.client.flatc.header.Status status(com.automq.elasticstream.client.flatc.header.Status obj) { int o = __offset(4); return o != 0 ? obj.__assign(__indirect(o + bb_pos), bb) : null; }
  public long timestampMs() { int o = __offset(6); return o != 0 ? bb.getLong(o + bb_pos) : 0L; }

  public static int createAppendResultEntry(FlatBufferBuilder builder,
      int statusOffset,
      long timestampMs) {
    builder.startTable(2);
    AppendResultEntry.addTimestampMs(builder, timestampMs);
    AppendResultEntry.addStatus(builder, statusOffset);
    return AppendResultEntry.endAppendResultEntry(builder);
  }

  public static void startAppendResultEntry(FlatBufferBuilder builder) { builder.startTable(2); }
  public static void addStatus(FlatBufferBuilder builder, int statusOffset) { builder.addOffset(0, statusOffset, 0); }
  public static void addTimestampMs(FlatBufferBuilder builder, long timestampMs) { builder.addLong(1, timestampMs, 0L); }
  public static int endAppendResultEntry(FlatBufferBuilder builder) {
    int o = builder.endTable();
    builder.required(o, 4);  // status
    return o;
  }

  public static final class Vector extends BaseVector {
    public Vector __assign(int _vector, int _element_size, ByteBuffer _bb) { __reset(_vector, _element_size, _bb); return this; }

    public AppendResultEntry get(int j) { return get(new AppendResultEntry(), j); }
    public AppendResultEntry get(AppendResultEntry obj, int j) {  return obj.__assign(__indirect(__element(j), bb), bb); }
  }
  public AppendResultEntryT unpack() {
    AppendResultEntryT _o = new AppendResultEntryT();
    unpackTo(_o);
    return _o;
  }
  public void unpackTo(AppendResultEntryT _o) {
    if (status() != null) _o.setStatus(status().unpack());
    else _o.setStatus(null);
    long _oTimestampMs = timestampMs();
    _o.setTimestampMs(_oTimestampMs);
  }
  public static int pack(FlatBufferBuilder builder, AppendResultEntryT _o) {
    if (_o == null) return 0;
    int _status = _o.getStatus() == null ? 0 : com.automq.elasticstream.client.flatc.header.Status.pack(builder, _o.getStatus());
    return createAppendResultEntry(
      builder,
      _status,
      _o.getTimestampMs());
  }
}
