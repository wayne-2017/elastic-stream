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

public class RangeIdT {
  private long streamId;
  private int rangeIndex;

  public long getStreamId() { return streamId; }

  public void setStreamId(long streamId) { this.streamId = streamId; }

  public int getRangeIndex() { return rangeIndex; }

  public void setRangeIndex(int rangeIndex) { this.rangeIndex = rangeIndex; }


  public RangeIdT() {
    this.streamId = 0L;
    this.rangeIndex = 0;
  }
}
