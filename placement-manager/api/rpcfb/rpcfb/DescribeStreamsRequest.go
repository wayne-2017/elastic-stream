// Code generated by the FlatBuffers compiler. DO NOT EDIT.

package rpcfb

import (
	flatbuffers "github.com/google/flatbuffers/go"
)

type DescribeStreamsRequestT struct {
	TimeoutMs int32 `json:"timeout_ms"`
	StreamIds []int64 `json:"stream_ids"`
}

func (t *DescribeStreamsRequestT) Pack(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	if t == nil { return 0 }
	streamIdsOffset := flatbuffers.UOffsetT(0)
	if t.StreamIds != nil {
		streamIdsLength := len(t.StreamIds)
		DescribeStreamsRequestStartStreamIdsVector(builder, streamIdsLength)
		for j := streamIdsLength - 1; j >= 0; j-- {
			builder.PrependInt64(t.StreamIds[j])
		}
		streamIdsOffset = builder.EndVector(streamIdsLength)
	}
	DescribeStreamsRequestStart(builder)
	DescribeStreamsRequestAddTimeoutMs(builder, t.TimeoutMs)
	DescribeStreamsRequestAddStreamIds(builder, streamIdsOffset)
	return DescribeStreamsRequestEnd(builder)
}

func (rcv *DescribeStreamsRequest) UnPackTo(t *DescribeStreamsRequestT) {
	t.TimeoutMs = rcv.TimeoutMs()
	streamIdsLength := rcv.StreamIdsLength()
	t.StreamIds = make([]int64, streamIdsLength)
	for j := 0; j < streamIdsLength; j++ {
		t.StreamIds[j] = rcv.StreamIds(j)
	}
}

func (rcv *DescribeStreamsRequest) UnPack() *DescribeStreamsRequestT {
	if rcv == nil { return nil }
	t := &DescribeStreamsRequestT{}
	rcv.UnPackTo(t)
	return t
}

type DescribeStreamsRequest struct {
	_tab flatbuffers.Table
}

func GetRootAsDescribeStreamsRequest(buf []byte, offset flatbuffers.UOffsetT) *DescribeStreamsRequest {
	n := flatbuffers.GetUOffsetT(buf[offset:])
	x := &DescribeStreamsRequest{}
	x.Init(buf, n+offset)
	return x
}

func GetSizePrefixedRootAsDescribeStreamsRequest(buf []byte, offset flatbuffers.UOffsetT) *DescribeStreamsRequest {
	n := flatbuffers.GetUOffsetT(buf[offset+flatbuffers.SizeUint32:])
	x := &DescribeStreamsRequest{}
	x.Init(buf, n+offset+flatbuffers.SizeUint32)
	return x
}

func (rcv *DescribeStreamsRequest) Init(buf []byte, i flatbuffers.UOffsetT) {
	rcv._tab.Bytes = buf
	rcv._tab.Pos = i
}

func (rcv *DescribeStreamsRequest) Table() flatbuffers.Table {
	return rcv._tab
}

func (rcv *DescribeStreamsRequest) TimeoutMs() int32 {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(4))
	if o != 0 {
		return rcv._tab.GetInt32(o + rcv._tab.Pos)
	}
	return 0
}

func (rcv *DescribeStreamsRequest) MutateTimeoutMs(n int32) bool {
	return rcv._tab.MutateInt32Slot(4, n)
}

func (rcv *DescribeStreamsRequest) StreamIds(j int) int64 {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(6))
	if o != 0 {
		a := rcv._tab.Vector(o)
		return rcv._tab.GetInt64(a + flatbuffers.UOffsetT(j*8))
	}
	return 0
}

func (rcv *DescribeStreamsRequest) StreamIdsLength() int {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(6))
	if o != 0 {
		return rcv._tab.VectorLen(o)
	}
	return 0
}

func (rcv *DescribeStreamsRequest) MutateStreamIds(j int, n int64) bool {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(6))
	if o != 0 {
		a := rcv._tab.Vector(o)
		return rcv._tab.MutateInt64(a+flatbuffers.UOffsetT(j*8), n)
	}
	return false
}

func DescribeStreamsRequestStart(builder *flatbuffers.Builder) {
	builder.StartObject(2)
}
func DescribeStreamsRequestAddTimeoutMs(builder *flatbuffers.Builder, timeoutMs int32) {
	builder.PrependInt32Slot(0, timeoutMs, 0)
}
func DescribeStreamsRequestAddStreamIds(builder *flatbuffers.Builder, streamIds flatbuffers.UOffsetT) {
	builder.PrependUOffsetTSlot(1, flatbuffers.UOffsetT(streamIds), 0)
}
func DescribeStreamsRequestStartStreamIdsVector(builder *flatbuffers.Builder, numElems int) flatbuffers.UOffsetT {
	return builder.StartVector(8, numElems, 8)
}
func DescribeStreamsRequestEnd(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	return builder.EndObject()
}