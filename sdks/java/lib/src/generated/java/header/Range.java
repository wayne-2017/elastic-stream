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
public final class Range extends Table {
  public static void ValidateVersion() { Constants.FLATBUFFERS_23_1_21(); }
  public static Range getRootAsRange(ByteBuffer _bb) { return getRootAsRange(_bb, new Range()); }
  public static Range getRootAsRange(ByteBuffer _bb, Range obj) { _bb.order(ByteOrder.LITTLE_ENDIAN); return (obj.__assign(_bb.getInt(_bb.position()) + _bb.position(), _bb)); }
  public void __init(int _i, ByteBuffer _bb) { __reset(_i, _bb); }
  public Range __assign(int _i, ByteBuffer _bb) { __init(_i, _bb); return this; }

  public long streamId() { int o = __offset(4); return o != 0 ? bb.getLong(o + bb_pos) : 0L; }
  public int rangeIndex() { int o = __offset(6); return o != 0 ? bb.getInt(o + bb_pos) : 0; }
  public long startOffset() { int o = __offset(8); return o != 0 ? bb.getLong(o + bb_pos) : 0L; }
  public long endOffset() { int o = __offset(10); return o != 0 ? bb.getLong(o + bb_pos) : 0L; }
  public long nextOffset() { int o = __offset(12); return o != 0 ? bb.getLong(o + bb_pos) : 0L; }
  public header.ReplicaNode replicaNodes(int j) { return replicaNodes(new header.ReplicaNode(), j); }
  public header.ReplicaNode replicaNodes(header.ReplicaNode obj, int j) { int o = __offset(14); return o != 0 ? obj.__assign(__indirect(__vector(o) + j * 4), bb) : null; }
  public int replicaNodesLength() { int o = __offset(14); return o != 0 ? __vector_len(o) : 0; }
  public header.ReplicaNode.Vector replicaNodesVector() { return replicaNodesVector(new header.ReplicaNode.Vector()); }
  public header.ReplicaNode.Vector replicaNodesVector(header.ReplicaNode.Vector obj) { int o = __offset(14); return o != 0 ? obj.__assign(__vector(o), 4, bb) : null; }

  public static int createRange(FlatBufferBuilder builder,
      long streamId,
      int rangeIndex,
      long startOffset,
      long endOffset,
      long nextOffset,
      int replicaNodesOffset) {
    builder.startTable(6);
    Range.addNextOffset(builder, nextOffset);
    Range.addEndOffset(builder, endOffset);
    Range.addStartOffset(builder, startOffset);
    Range.addStreamId(builder, streamId);
    Range.addReplicaNodes(builder, replicaNodesOffset);
    Range.addRangeIndex(builder, rangeIndex);
    return Range.endRange(builder);
  }

  public static void startRange(FlatBufferBuilder builder) { builder.startTable(6); }
  public static void addStreamId(FlatBufferBuilder builder, long streamId) { builder.addLong(0, streamId, 0L); }
  public static void addRangeIndex(FlatBufferBuilder builder, int rangeIndex) { builder.addInt(1, rangeIndex, 0); }
  public static void addStartOffset(FlatBufferBuilder builder, long startOffset) { builder.addLong(2, startOffset, 0L); }
  public static void addEndOffset(FlatBufferBuilder builder, long endOffset) { builder.addLong(3, endOffset, 0L); }
  public static void addNextOffset(FlatBufferBuilder builder, long nextOffset) { builder.addLong(4, nextOffset, 0L); }
  public static void addReplicaNodes(FlatBufferBuilder builder, int replicaNodesOffset) { builder.addOffset(5, replicaNodesOffset, 0); }
  public static int createReplicaNodesVector(FlatBufferBuilder builder, int[] data) { builder.startVector(4, data.length, 4); for (int i = data.length - 1; i >= 0; i--) builder.addOffset(data[i]); return builder.endVector(); }
  public static void startReplicaNodesVector(FlatBufferBuilder builder, int numElems) { builder.startVector(4, numElems, 4); }
  public static int endRange(FlatBufferBuilder builder) {
    int o = builder.endTable();
    return o;
  }

  public static final class Vector extends BaseVector {
    public Vector __assign(int _vector, int _element_size, ByteBuffer _bb) { __reset(_vector, _element_size, _bb); return this; }

    public Range get(int j) { return get(new Range(), j); }
    public Range get(Range obj, int j) {  return obj.__assign(__indirect(__element(j), bb), bb); }
  }
  public RangeT unpack() {
    RangeT _o = new RangeT();
    unpackTo(_o);
    return _o;
  }
  public void unpackTo(RangeT _o) {
    long _oStreamId = streamId();
    _o.setStreamId(_oStreamId);
    int _oRangeIndex = rangeIndex();
    _o.setRangeIndex(_oRangeIndex);
    long _oStartOffset = startOffset();
    _o.setStartOffset(_oStartOffset);
    long _oEndOffset = endOffset();
    _o.setEndOffset(_oEndOffset);
    long _oNextOffset = nextOffset();
    _o.setNextOffset(_oNextOffset);
    header.ReplicaNodeT[] _oReplicaNodes = new header.ReplicaNodeT[replicaNodesLength()];
    for (int _j = 0; _j < replicaNodesLength(); ++_j) {_oReplicaNodes[_j] = (replicaNodes(_j) != null ? replicaNodes(_j).unpack() : null);}
    _o.setReplicaNodes(_oReplicaNodes);
  }
  public static int pack(FlatBufferBuilder builder, RangeT _o) {
    if (_o == null) return 0;
    int _replicaNodes = 0;
    if (_o.getReplicaNodes() != null) {
      int[] __replicaNodes = new int[_o.getReplicaNodes().length];
      int _j = 0;
      for (header.ReplicaNodeT _e : _o.getReplicaNodes()) { __replicaNodes[_j] = header.ReplicaNode.pack(builder, _e); _j++;}
      _replicaNodes = createReplicaNodesVector(builder, __replicaNodes);
    }
    return createRange(
      builder,
      _o.getStreamId(),
      _o.getRangeIndex(),
      _o.getStartOffset(),
      _o.getEndOffset(),
      _o.getNextOffset(),
      _replicaNodes);
  }
}

