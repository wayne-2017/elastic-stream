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

public class TrimStreamRequestT {
  private int timeoutMs;
  private long streamId;
  private long minOffset;

  public int getTimeoutMs() { return timeoutMs; }

  public void setTimeoutMs(int timeoutMs) { this.timeoutMs = timeoutMs; }

  public long getStreamId() { return streamId; }

  public void setStreamId(long streamId) { this.streamId = streamId; }

  public long getMinOffset() { return minOffset; }

  public void setMinOffset(long minOffset) { this.minOffset = minOffset; }


  public TrimStreamRequestT() {
    this.timeoutMs = 0;
    this.streamId = -1L;
    this.minOffset = 0L;
  }
}
