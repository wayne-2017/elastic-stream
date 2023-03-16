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

public class RecordBatchMetaT {
  private long streamId;
  private short flags;
  private long baseOffset;
  private int lastOffsetDelta;
  private long baseTimestamp;

  public long getStreamId() { return streamId; }

  public void setStreamId(long streamId) { this.streamId = streamId; }

  public short getFlags() { return flags; }

  public void setFlags(short flags) { this.flags = flags; }

  public long getBaseOffset() { return baseOffset; }

  public void setBaseOffset(long baseOffset) { this.baseOffset = baseOffset; }

  public int getLastOffsetDelta() { return lastOffsetDelta; }

  public void setLastOffsetDelta(int lastOffsetDelta) { this.lastOffsetDelta = lastOffsetDelta; }

  public long getBaseTimestamp() { return baseTimestamp; }

  public void setBaseTimestamp(long baseTimestamp) { this.baseTimestamp = baseTimestamp; }


  public RecordBatchMetaT() {
    this.streamId = 0L;
    this.flags = 0;
    this.baseOffset = 0L;
    this.lastOffsetDelta = 0;
    this.baseTimestamp = 0L;
  }
}
