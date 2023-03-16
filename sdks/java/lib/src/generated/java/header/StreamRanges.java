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
public final class StreamRanges extends Table {
  public static void ValidateVersion() { Constants.FLATBUFFERS_23_1_21(); }
  public static StreamRanges getRootAsStreamRanges(ByteBuffer _bb) { return getRootAsStreamRanges(_bb, new StreamRanges()); }
  public static StreamRanges getRootAsStreamRanges(ByteBuffer _bb, StreamRanges obj) { _bb.order(ByteOrder.LITTLE_ENDIAN); return (obj.__assign(_bb.getInt(_bb.position()) + _bb.position(), _bb)); }
  public void __init(int _i, ByteBuffer _bb) { __reset(_i, _bb); }
  public StreamRanges __assign(int _i, ByteBuffer _bb) { __init(_i, _bb); return this; }

  public long streamId() { int o = __offset(4); return o != 0 ? bb.getLong(o + bb_pos) : 0L; }
  public header.Range ranges(int j) { return ranges(new header.Range(), j); }
  public header.Range ranges(header.Range obj, int j) { int o = __offset(6); return o != 0 ? obj.__assign(__indirect(__vector(o) + j * 4), bb) : null; }
  public int rangesLength() { int o = __offset(6); return o != 0 ? __vector_len(o) : 0; }
  public header.Range.Vector rangesVector() { return rangesVector(new header.Range.Vector()); }
  public header.Range.Vector rangesVector(header.Range.Vector obj) { int o = __offset(6); return o != 0 ? obj.__assign(__vector(o), 4, bb) : null; }

  public static int createStreamRanges(FlatBufferBuilder builder,
      long streamId,
      int rangesOffset) {
    builder.startTable(2);
    StreamRanges.addStreamId(builder, streamId);
    StreamRanges.addRanges(builder, rangesOffset);
    return StreamRanges.endStreamRanges(builder);
  }

  public static void startStreamRanges(FlatBufferBuilder builder) { builder.startTable(2); }
  public static void addStreamId(FlatBufferBuilder builder, long streamId) { builder.addLong(0, streamId, 0L); }
  public static void addRanges(FlatBufferBuilder builder, int rangesOffset) { builder.addOffset(1, rangesOffset, 0); }
  public static int createRangesVector(FlatBufferBuilder builder, int[] data) { builder.startVector(4, data.length, 4); for (int i = data.length - 1; i >= 0; i--) builder.addOffset(data[i]); return builder.endVector(); }
  public static void startRangesVector(FlatBufferBuilder builder, int numElems) { builder.startVector(4, numElems, 4); }
  public static int endStreamRanges(FlatBufferBuilder builder) {
    int o = builder.endTable();
    return o;
  }

  public static final class Vector extends BaseVector {
    public Vector __assign(int _vector, int _element_size, ByteBuffer _bb) { __reset(_vector, _element_size, _bb); return this; }

    public StreamRanges get(int j) { return get(new StreamRanges(), j); }
    public StreamRanges get(StreamRanges obj, int j) {  return obj.__assign(__indirect(__element(j), bb), bb); }
  }
  public StreamRangesT unpack() {
    StreamRangesT _o = new StreamRangesT();
    unpackTo(_o);
    return _o;
  }
  public void unpackTo(StreamRangesT _o) {
    long _oStreamId = streamId();
    _o.setStreamId(_oStreamId);
    header.RangeT[] _oRanges = new header.RangeT[rangesLength()];
    for (int _j = 0; _j < rangesLength(); ++_j) {_oRanges[_j] = (ranges(_j) != null ? ranges(_j).unpack() : null);}
    _o.setRanges(_oRanges);
  }
  public static int pack(FlatBufferBuilder builder, StreamRangesT _o) {
    if (_o == null) return 0;
    int _ranges = 0;
    if (_o.getRanges() != null) {
      int[] __ranges = new int[_o.getRanges().length];
      int _j = 0;
      for (header.RangeT _e : _o.getRanges()) { __ranges[_j] = header.Range.pack(builder, _e); _j++;}
      _ranges = createRangesVector(builder, __ranges);
    }
    return createStreamRanges(
      builder,
      _o.getStreamId(),
      _ranges);
  }
}
