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

public class HeartbeatRequestT {
  private String clientId;
  private byte clientRole;
  private header.DataNodeT dataNode;

  public String getClientId() { return clientId; }

  public void setClientId(String clientId) { this.clientId = clientId; }

  public byte getClientRole() { return clientRole; }

  public void setClientRole(byte clientRole) { this.clientRole = clientRole; }

  public header.DataNodeT getDataNode() { return dataNode; }

  public void setDataNode(header.DataNodeT dataNode) { this.dataNode = dataNode; }


  public HeartbeatRequestT() {
    this.clientId = null;
    this.clientRole = 0;
    this.dataNode = null;
  }
}
