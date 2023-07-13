// Code generated by the FlatBuffers compiler. DO NOT EDIT.

package rpcfb

import (
	flatbuffers "github.com/google/flatbuffers/go"
)

type OffloadOwnerT struct {
	ServerId int32 `json:"server_id"`
	Epoch int64 `json:"epoch"`
}

func (t *OffloadOwnerT) Pack(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	if t == nil { return 0 }
	OffloadOwnerStart(builder)
	OffloadOwnerAddServerId(builder, t.ServerId)
	OffloadOwnerAddEpoch(builder, t.Epoch)
	return OffloadOwnerEnd(builder)
}

func (rcv *OffloadOwner) UnPackTo(t *OffloadOwnerT) {
	t.ServerId = rcv.ServerId()
	t.Epoch = rcv.Epoch()
}

func (rcv *OffloadOwner) UnPack() *OffloadOwnerT {
	if rcv == nil { return nil }
	t := &OffloadOwnerT{}
	rcv.UnPackTo(t)
	return t
}

type OffloadOwner struct {
	_tab flatbuffers.Table
}

func GetRootAsOffloadOwner(buf []byte, offset flatbuffers.UOffsetT) *OffloadOwner {
	n := flatbuffers.GetUOffsetT(buf[offset:])
	x := &OffloadOwner{}
	x.Init(buf, n+offset)
	return x
}

func GetSizePrefixedRootAsOffloadOwner(buf []byte, offset flatbuffers.UOffsetT) *OffloadOwner {
	n := flatbuffers.GetUOffsetT(buf[offset+flatbuffers.SizeUint32:])
	x := &OffloadOwner{}
	x.Init(buf, n+offset+flatbuffers.SizeUint32)
	return x
}

func (rcv *OffloadOwner) Init(buf []byte, i flatbuffers.UOffsetT) {
	rcv._tab.Bytes = buf
	rcv._tab.Pos = i
}

func (rcv *OffloadOwner) Table() flatbuffers.Table {
	return rcv._tab
}

func (rcv *OffloadOwner) ServerId() int32 {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(4))
	if o != 0 {
		return rcv._tab.GetInt32(o + rcv._tab.Pos)
	}
	return -1
}

func (rcv *OffloadOwner) MutateServerId(n int32) bool {
	return rcv._tab.MutateInt32Slot(4, n)
}

func (rcv *OffloadOwner) Epoch() int64 {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(6))
	if o != 0 {
		return rcv._tab.GetInt64(o + rcv._tab.Pos)
	}
	return -1
}

func (rcv *OffloadOwner) MutateEpoch(n int64) bool {
	return rcv._tab.MutateInt64Slot(6, n)
}

func OffloadOwnerStart(builder *flatbuffers.Builder) {
	builder.StartObject(2)
}
func OffloadOwnerAddServerId(builder *flatbuffers.Builder, serverId int32) {
	builder.PrependInt32Slot(0, serverId, -1)
}
func OffloadOwnerAddEpoch(builder *flatbuffers.Builder, epoch int64) {
	builder.PrependInt64Slot(1, epoch, -1)
}
func OffloadOwnerEnd(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	return builder.EndObject()
}