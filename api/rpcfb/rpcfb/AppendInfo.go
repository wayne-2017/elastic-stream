// Code generated by the FlatBuffers compiler. DO NOT EDIT.

package rpcfb

import (
	flatbuffers "github.com/google/flatbuffers/go"
)

type AppendInfoT struct {
	Range *RangeT `json:"range"`
	RequestIndex int32 `json:"request_index"`
	BatchLength int32 `json:"batch_length"`
}

func (t *AppendInfoT) Pack(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	if t == nil { return 0 }
	range_Offset := t.Range.Pack(builder)
	AppendInfoStart(builder)
	AppendInfoAddRange(builder, range_Offset)
	AppendInfoAddRequestIndex(builder, t.RequestIndex)
	AppendInfoAddBatchLength(builder, t.BatchLength)
	return AppendInfoEnd(builder)
}

func (rcv *AppendInfo) UnPackTo(t *AppendInfoT) {
	t.Range = rcv.Range(nil).UnPack()
	t.RequestIndex = rcv.RequestIndex()
	t.BatchLength = rcv.BatchLength()
}

func (rcv *AppendInfo) UnPack() *AppendInfoT {
	if rcv == nil { return nil }
	t := &AppendInfoT{}
	rcv.UnPackTo(t)
	return t
}

type AppendInfo struct {
	_tab flatbuffers.Table
}

func GetRootAsAppendInfo(buf []byte, offset flatbuffers.UOffsetT) *AppendInfo {
	n := flatbuffers.GetUOffsetT(buf[offset:])
	x := &AppendInfo{}
	x.Init(buf, n+offset)
	return x
}

func GetSizePrefixedRootAsAppendInfo(buf []byte, offset flatbuffers.UOffsetT) *AppendInfo {
	n := flatbuffers.GetUOffsetT(buf[offset+flatbuffers.SizeUint32:])
	x := &AppendInfo{}
	x.Init(buf, n+offset+flatbuffers.SizeUint32)
	return x
}

func (rcv *AppendInfo) Init(buf []byte, i flatbuffers.UOffsetT) {
	rcv._tab.Bytes = buf
	rcv._tab.Pos = i
}

func (rcv *AppendInfo) Table() flatbuffers.Table {
	return rcv._tab
}

func (rcv *AppendInfo) Range(obj *Range) *Range {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(4))
	if o != 0 {
		x := rcv._tab.Indirect(o + rcv._tab.Pos)
		if obj == nil {
			obj = new(Range)
		}
		obj.Init(rcv._tab.Bytes, x)
		return obj
	}
	return nil
}

func (rcv *AppendInfo) RequestIndex() int32 {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(6))
	if o != 0 {
		return rcv._tab.GetInt32(o + rcv._tab.Pos)
	}
	return 0
}

func (rcv *AppendInfo) MutateRequestIndex(n int32) bool {
	return rcv._tab.MutateInt32Slot(6, n)
}

func (rcv *AppendInfo) BatchLength() int32 {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(8))
	if o != 0 {
		return rcv._tab.GetInt32(o + rcv._tab.Pos)
	}
	return 0
}

func (rcv *AppendInfo) MutateBatchLength(n int32) bool {
	return rcv._tab.MutateInt32Slot(8, n)
}

func AppendInfoStart(builder *flatbuffers.Builder) {
	builder.StartObject(3)
}
func AppendInfoAddRange(builder *flatbuffers.Builder, range_ flatbuffers.UOffsetT) {
	builder.PrependUOffsetTSlot(0, flatbuffers.UOffsetT(range_), 0)
}
func AppendInfoAddRequestIndex(builder *flatbuffers.Builder, requestIndex int32) {
	builder.PrependInt32Slot(1, requestIndex, 0)
}
func AppendInfoAddBatchLength(builder *flatbuffers.Builder, batchLength int32) {
	builder.PrependInt32Slot(2, batchLength, 0)
}
func AppendInfoEnd(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	return builder.EndObject()
}
