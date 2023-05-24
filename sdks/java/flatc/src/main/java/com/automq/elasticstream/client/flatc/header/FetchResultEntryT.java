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

public class FetchResultEntryT {
  private com.automq.elasticstream.client.flatc.header.StatusT status;
  private long streamId;
  private int batchCount;

  public com.automq.elasticstream.client.flatc.header.StatusT getStatus() { return status; }

  public void setStatus(com.automq.elasticstream.client.flatc.header.StatusT status) { this.status = status; }

  public long getStreamId() { return streamId; }

  public void setStreamId(long streamId) { this.streamId = streamId; }

  public int getBatchCount() { return batchCount; }

  public void setBatchCount(int batchCount) { this.batchCount = batchCount; }


  public FetchResultEntryT() {
    this.status = null;
    this.streamId = -1L;
    this.batchCount = 0;
  }
}
