// Code generated by the FlatBuffers compiler. DO NOT EDIT.

package rpcfb

import (
	flatbuffers "github.com/google/flatbuffers/go"
)

type UpdateStreamsResponseT struct {
	ThrottleTimeMs int32 `json:"throttle_time_ms"`
	UpdateResponses []*UpdateStreamResultT `json:"update_responses"`
	ErrorCode ErrorCode `json:"error_code"`
	ErrorMessage string `json:"error_message"`
}

func (t *UpdateStreamsResponseT) Pack(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	if t == nil { return 0 }
	updateResponsesOffset := flatbuffers.UOffsetT(0)
	if t.UpdateResponses != nil {
		updateResponsesLength := len(t.UpdateResponses)
		updateResponsesOffsets := make([]flatbuffers.UOffsetT, updateResponsesLength)
		for j := 0; j < updateResponsesLength; j++ {
			updateResponsesOffsets[j] = t.UpdateResponses[j].Pack(builder)
		}
		UpdateStreamsResponseStartUpdateResponsesVector(builder, updateResponsesLength)
		for j := updateResponsesLength - 1; j >= 0; j-- {
			builder.PrependUOffsetT(updateResponsesOffsets[j])
		}
		updateResponsesOffset = builder.EndVector(updateResponsesLength)
	}
	errorMessageOffset := flatbuffers.UOffsetT(0)
	if t.ErrorMessage != "" {
		errorMessageOffset = builder.CreateString(t.ErrorMessage)
	}
	UpdateStreamsResponseStart(builder)
	UpdateStreamsResponseAddThrottleTimeMs(builder, t.ThrottleTimeMs)
	UpdateStreamsResponseAddUpdateResponses(builder, updateResponsesOffset)
	UpdateStreamsResponseAddErrorCode(builder, t.ErrorCode)
	UpdateStreamsResponseAddErrorMessage(builder, errorMessageOffset)
	return UpdateStreamsResponseEnd(builder)
}

func (rcv *UpdateStreamsResponse) UnPackTo(t *UpdateStreamsResponseT) {
	t.ThrottleTimeMs = rcv.ThrottleTimeMs()
	updateResponsesLength := rcv.UpdateResponsesLength()
	t.UpdateResponses = make([]*UpdateStreamResultT, updateResponsesLength)
	for j := 0; j < updateResponsesLength; j++ {
		x := UpdateStreamResult{}
		rcv.UpdateResponses(&x, j)
		t.UpdateResponses[j] = x.UnPack()
	}
	t.ErrorCode = rcv.ErrorCode()
	t.ErrorMessage = string(rcv.ErrorMessage())
}

func (rcv *UpdateStreamsResponse) UnPack() *UpdateStreamsResponseT {
	if rcv == nil { return nil }
	t := &UpdateStreamsResponseT{}
	rcv.UnPackTo(t)
	return t
}

type UpdateStreamsResponse struct {
	_tab flatbuffers.Table
}

func GetRootAsUpdateStreamsResponse(buf []byte, offset flatbuffers.UOffsetT) *UpdateStreamsResponse {
	n := flatbuffers.GetUOffsetT(buf[offset:])
	x := &UpdateStreamsResponse{}
	x.Init(buf, n+offset)
	return x
}

func GetSizePrefixedRootAsUpdateStreamsResponse(buf []byte, offset flatbuffers.UOffsetT) *UpdateStreamsResponse {
	n := flatbuffers.GetUOffsetT(buf[offset+flatbuffers.SizeUint32:])
	x := &UpdateStreamsResponse{}
	x.Init(buf, n+offset+flatbuffers.SizeUint32)
	return x
}

func (rcv *UpdateStreamsResponse) Init(buf []byte, i flatbuffers.UOffsetT) {
	rcv._tab.Bytes = buf
	rcv._tab.Pos = i
}

func (rcv *UpdateStreamsResponse) Table() flatbuffers.Table {
	return rcv._tab
}

func (rcv *UpdateStreamsResponse) ThrottleTimeMs() int32 {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(4))
	if o != 0 {
		return rcv._tab.GetInt32(o + rcv._tab.Pos)
	}
	return 0
}

func (rcv *UpdateStreamsResponse) MutateThrottleTimeMs(n int32) bool {
	return rcv._tab.MutateInt32Slot(4, n)
}

func (rcv *UpdateStreamsResponse) UpdateResponses(obj *UpdateStreamResult, j int) bool {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(6))
	if o != 0 {
		x := rcv._tab.Vector(o)
		x += flatbuffers.UOffsetT(j) * 4
		x = rcv._tab.Indirect(x)
		obj.Init(rcv._tab.Bytes, x)
		return true
	}
	return false
}

func (rcv *UpdateStreamsResponse) UpdateResponsesLength() int {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(6))
	if o != 0 {
		return rcv._tab.VectorLen(o)
	}
	return 0
}

func (rcv *UpdateStreamsResponse) ErrorCode() ErrorCode {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(8))
	if o != 0 {
		return ErrorCode(rcv._tab.GetInt16(o + rcv._tab.Pos))
	}
	return 0
}

func (rcv *UpdateStreamsResponse) MutateErrorCode(n ErrorCode) bool {
	return rcv._tab.MutateInt16Slot(8, int16(n))
}

func (rcv *UpdateStreamsResponse) ErrorMessage() []byte {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(10))
	if o != 0 {
		return rcv._tab.ByteVector(o + rcv._tab.Pos)
	}
	return nil
}

func UpdateStreamsResponseStart(builder *flatbuffers.Builder) {
	builder.StartObject(4)
}
func UpdateStreamsResponseAddThrottleTimeMs(builder *flatbuffers.Builder, throttleTimeMs int32) {
	builder.PrependInt32Slot(0, throttleTimeMs, 0)
}
func UpdateStreamsResponseAddUpdateResponses(builder *flatbuffers.Builder, updateResponses flatbuffers.UOffsetT) {
	builder.PrependUOffsetTSlot(1, flatbuffers.UOffsetT(updateResponses), 0)
}
func UpdateStreamsResponseStartUpdateResponsesVector(builder *flatbuffers.Builder, numElems int) flatbuffers.UOffsetT {
	return builder.StartVector(4, numElems, 4)
}
func UpdateStreamsResponseAddErrorCode(builder *flatbuffers.Builder, errorCode ErrorCode) {
	builder.PrependInt16Slot(2, int16(errorCode), 0)
}
func UpdateStreamsResponseAddErrorMessage(builder *flatbuffers.Builder, errorMessage flatbuffers.UOffsetT) {
	builder.PrependUOffsetTSlot(3, flatbuffers.UOffsetT(errorMessage), 0)
}
func UpdateStreamsResponseEnd(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	return builder.EndObject()
}
