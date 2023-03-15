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

public class TrimStreamsRequestT {
  private int timeoutMs;
  private header.TrimmedStreamT[] trimmedStreams;

  public int getTimeoutMs() { return timeoutMs; }

  public void setTimeoutMs(int timeoutMs) { this.timeoutMs = timeoutMs; }

  public header.TrimmedStreamT[] getTrimmedStreams() { return trimmedStreams; }

  public void setTrimmedStreams(header.TrimmedStreamT[] trimmedStreams) { this.trimmedStreams = trimmedStreams; }


  public TrimStreamsRequestT() {
    this.timeoutMs = 0;
    this.trimmedStreams = null;
  }
}

