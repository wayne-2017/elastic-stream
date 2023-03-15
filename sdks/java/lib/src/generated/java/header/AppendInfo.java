// automatically generated by the FlatBuffers compiler, do not modify

package header;

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
public final class AppendInfo extends Table {
  public static void ValidateVersion() { Constants.FLATBUFFERS_23_1_21(); }
  public static AppendInfo getRootAsAppendInfo(ByteBuffer _bb) { return getRootAsAppendInfo(_bb, new AppendInfo()); }
  public static AppendInfo getRootAsAppendInfo(ByteBuffer _bb, AppendInfo obj) { _bb.order(ByteOrder.LITTLE_ENDIAN); return (obj.__assign(_bb.getInt(_bb.position()) + _bb.position(), _bb)); }
  public void __init(int _i, ByteBuffer _bb) { __reset(_i, _bb); }
  public AppendInfo __assign(int _i, ByteBuffer _bb) { __init(_i, _bb); return this; }

  public long streamId() { int o = __offset(4); return o != 0 ? bb.getLong(o + bb_pos) : 0L; }
  public int requestIndex() { int o = __offset(6); return o != 0 ? bb.getInt(o + bb_pos) : 0; }
  public int batchLength() { int o = __offset(8); return o != 0 ? bb.getInt(o + bb_pos) : 0; }

  public static int createAppendInfo(FlatBufferBuilder builder,
      long streamId,
      int requestIndex,
      int batchLength) {
    builder.startTable(3);
    AppendInfo.addStreamId(builder, streamId);
    AppendInfo.addBatchLength(builder, batchLength);
    AppendInfo.addRequestIndex(builder, requestIndex);
    return AppendInfo.endAppendInfo(builder);
  }

  public static void startAppendInfo(FlatBufferBuilder builder) { builder.startTable(3); }
  public static void addStreamId(FlatBufferBuilder builder, long streamId) { builder.addLong(0, streamId, 0L); }
  public static void addRequestIndex(FlatBufferBuilder builder, int requestIndex) { builder.addInt(1, requestIndex, 0); }
  public static void addBatchLength(FlatBufferBuilder builder, int batchLength) { builder.addInt(2, batchLength, 0); }
  public static int endAppendInfo(FlatBufferBuilder builder) {
    int o = builder.endTable();
    return o;
  }

  public static final class Vector extends BaseVector {
    public Vector __assign(int _vector, int _element_size, ByteBuffer _bb) { __reset(_vector, _element_size, _bb); return this; }

    public AppendInfo get(int j) { return get(new AppendInfo(), j); }
    public AppendInfo get(AppendInfo obj, int j) {  return obj.__assign(__indirect(__element(j), bb), bb); }
  }
  public AppendInfoT unpack() {
    AppendInfoT _o = new AppendInfoT();
    unpackTo(_o);
    return _o;
  }
  public void unpackTo(AppendInfoT _o) {
    long _oStreamId = streamId();
    _o.setStreamId(_oStreamId);
    int _oRequestIndex = requestIndex();
    _o.setRequestIndex(_oRequestIndex);
    int _oBatchLength = batchLength();
    _o.setBatchLength(_oBatchLength);
  }
  public static int pack(FlatBufferBuilder builder, AppendInfoT _o) {
    if (_o == null) return 0;
    return createAppendInfo(
      builder,
      _o.getStreamId(),
      _o.getRequestIndex(),
      _o.getBatchLength());
  }
}

