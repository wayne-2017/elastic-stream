// Code generated by the FlatBuffers compiler. DO NOT EDIT.

package rpcfb

import (
	flatbuffers "github.com/google/flatbuffers/go"
)

type ReportMetricsRequestT struct {
	DataNode *DataNodeT `json:"data_node"`
	DiskInRate int64 `json:"disk_in_rate"`
	DiskOutRate int64 `json:"disk_out_rate"`
	DiskFreeSpace int64 `json:"disk_free_space"`
	DiskUnindexedDataSize int64 `json:"disk_unindexed_data_size"`
	MemoryUsed int64 `json:"memory_used"`
	UringTaskRate int16 `json:"uring_task_rate"`
	UringInflightTaskCnt int16 `json:"uring_inflight_task_cnt"`
	UringPendingTaskCnt int32 `json:"uring_pending_task_cnt"`
	UringTaskAvgLatency int16 `json:"uring_task_avg_latency"`
	NetworkAppendRate int16 `json:"network_append_rate"`
	NetworkFetchRate int16 `json:"network_fetch_rate"`
	NetworkFailedAppendRate int16 `json:"network_failed_append_rate"`
	NetworkFailedFetchRate int16 `json:"network_failed_fetch_rate"`
	NetworkAppendAvgLatency int16 `json:"network_append_avg_latency"`
	NetworkFetchAvgLatency int16 `json:"network_fetch_avg_latency"`
	RangeMissingReplicaCnt int16 `json:"range_missing_replica_cnt"`
	RangeActiveCnt int16 `json:"range_active_cnt"`
}

func (t *ReportMetricsRequestT) Pack(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	if t == nil { return 0 }
	dataNodeOffset := t.DataNode.Pack(builder)
	ReportMetricsRequestStart(builder)
	ReportMetricsRequestAddDataNode(builder, dataNodeOffset)
	ReportMetricsRequestAddDiskInRate(builder, t.DiskInRate)
	ReportMetricsRequestAddDiskOutRate(builder, t.DiskOutRate)
	ReportMetricsRequestAddDiskFreeSpace(builder, t.DiskFreeSpace)
	ReportMetricsRequestAddDiskUnindexedDataSize(builder, t.DiskUnindexedDataSize)
	ReportMetricsRequestAddMemoryUsed(builder, t.MemoryUsed)
	ReportMetricsRequestAddUringTaskRate(builder, t.UringTaskRate)
	ReportMetricsRequestAddUringInflightTaskCnt(builder, t.UringInflightTaskCnt)
	ReportMetricsRequestAddUringPendingTaskCnt(builder, t.UringPendingTaskCnt)
	ReportMetricsRequestAddUringTaskAvgLatency(builder, t.UringTaskAvgLatency)
	ReportMetricsRequestAddNetworkAppendRate(builder, t.NetworkAppendRate)
	ReportMetricsRequestAddNetworkFetchRate(builder, t.NetworkFetchRate)
	ReportMetricsRequestAddNetworkFailedAppendRate(builder, t.NetworkFailedAppendRate)
	ReportMetricsRequestAddNetworkFailedFetchRate(builder, t.NetworkFailedFetchRate)
	ReportMetricsRequestAddNetworkAppendAvgLatency(builder, t.NetworkAppendAvgLatency)
	ReportMetricsRequestAddNetworkFetchAvgLatency(builder, t.NetworkFetchAvgLatency)
	ReportMetricsRequestAddRangeMissingReplicaCnt(builder, t.RangeMissingReplicaCnt)
	ReportMetricsRequestAddRangeActiveCnt(builder, t.RangeActiveCnt)
	return ReportMetricsRequestEnd(builder)
}

func (rcv *ReportMetricsRequest) UnPackTo(t *ReportMetricsRequestT) {
	t.DataNode = rcv.DataNode(nil).UnPack()
	t.DiskInRate = rcv.DiskInRate()
	t.DiskOutRate = rcv.DiskOutRate()
	t.DiskFreeSpace = rcv.DiskFreeSpace()
	t.DiskUnindexedDataSize = rcv.DiskUnindexedDataSize()
	t.MemoryUsed = rcv.MemoryUsed()
	t.UringTaskRate = rcv.UringTaskRate()
	t.UringInflightTaskCnt = rcv.UringInflightTaskCnt()
	t.UringPendingTaskCnt = rcv.UringPendingTaskCnt()
	t.UringTaskAvgLatency = rcv.UringTaskAvgLatency()
	t.NetworkAppendRate = rcv.NetworkAppendRate()
	t.NetworkFetchRate = rcv.NetworkFetchRate()
	t.NetworkFailedAppendRate = rcv.NetworkFailedAppendRate()
	t.NetworkFailedFetchRate = rcv.NetworkFailedFetchRate()
	t.NetworkAppendAvgLatency = rcv.NetworkAppendAvgLatency()
	t.NetworkFetchAvgLatency = rcv.NetworkFetchAvgLatency()
	t.RangeMissingReplicaCnt = rcv.RangeMissingReplicaCnt()
	t.RangeActiveCnt = rcv.RangeActiveCnt()
}

func (rcv *ReportMetricsRequest) UnPack() *ReportMetricsRequestT {
	if rcv == nil { return nil }
	t := &ReportMetricsRequestT{}
	rcv.UnPackTo(t)
	return t
}

type ReportMetricsRequest struct {
	_tab flatbuffers.Table
}

func GetRootAsReportMetricsRequest(buf []byte, offset flatbuffers.UOffsetT) *ReportMetricsRequest {
	n := flatbuffers.GetUOffsetT(buf[offset:])
	x := &ReportMetricsRequest{}
	x.Init(buf, n+offset)
	return x
}

func GetSizePrefixedRootAsReportMetricsRequest(buf []byte, offset flatbuffers.UOffsetT) *ReportMetricsRequest {
	n := flatbuffers.GetUOffsetT(buf[offset+flatbuffers.SizeUint32:])
	x := &ReportMetricsRequest{}
	x.Init(buf, n+offset+flatbuffers.SizeUint32)
	return x
}

func (rcv *ReportMetricsRequest) Init(buf []byte, i flatbuffers.UOffsetT) {
	rcv._tab.Bytes = buf
	rcv._tab.Pos = i
}

func (rcv *ReportMetricsRequest) Table() flatbuffers.Table {
	return rcv._tab
}

func (rcv *ReportMetricsRequest) DataNode(obj *DataNode) *DataNode {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(4))
	if o != 0 {
		x := rcv._tab.Indirect(o + rcv._tab.Pos)
		if obj == nil {
			obj = new(DataNode)
		}
		obj.Init(rcv._tab.Bytes, x)
		return obj
	}
	return nil
}

func (rcv *ReportMetricsRequest) DiskInRate() int64 {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(6))
	if o != 0 {
		return rcv._tab.GetInt64(o + rcv._tab.Pos)
	}
	return 0
}

func (rcv *ReportMetricsRequest) MutateDiskInRate(n int64) bool {
	return rcv._tab.MutateInt64Slot(6, n)
}

func (rcv *ReportMetricsRequest) DiskOutRate() int64 {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(8))
	if o != 0 {
		return rcv._tab.GetInt64(o + rcv._tab.Pos)
	}
	return 0
}

func (rcv *ReportMetricsRequest) MutateDiskOutRate(n int64) bool {
	return rcv._tab.MutateInt64Slot(8, n)
}

func (rcv *ReportMetricsRequest) DiskFreeSpace() int64 {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(10))
	if o != 0 {
		return rcv._tab.GetInt64(o + rcv._tab.Pos)
	}
	return 0
}

func (rcv *ReportMetricsRequest) MutateDiskFreeSpace(n int64) bool {
	return rcv._tab.MutateInt64Slot(10, n)
}

func (rcv *ReportMetricsRequest) DiskUnindexedDataSize() int64 {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(12))
	if o != 0 {
		return rcv._tab.GetInt64(o + rcv._tab.Pos)
	}
	return 0
}

func (rcv *ReportMetricsRequest) MutateDiskUnindexedDataSize(n int64) bool {
	return rcv._tab.MutateInt64Slot(12, n)
}

func (rcv *ReportMetricsRequest) MemoryUsed() int64 {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(14))
	if o != 0 {
		return rcv._tab.GetInt64(o + rcv._tab.Pos)
	}
	return 0
}

func (rcv *ReportMetricsRequest) MutateMemoryUsed(n int64) bool {
	return rcv._tab.MutateInt64Slot(14, n)
}

func (rcv *ReportMetricsRequest) UringTaskRate() int16 {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(16))
	if o != 0 {
		return rcv._tab.GetInt16(o + rcv._tab.Pos)
	}
	return 0
}

func (rcv *ReportMetricsRequest) MutateUringTaskRate(n int16) bool {
	return rcv._tab.MutateInt16Slot(16, n)
}

func (rcv *ReportMetricsRequest) UringInflightTaskCnt() int16 {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(18))
	if o != 0 {
		return rcv._tab.GetInt16(o + rcv._tab.Pos)
	}
	return 0
}

func (rcv *ReportMetricsRequest) MutateUringInflightTaskCnt(n int16) bool {
	return rcv._tab.MutateInt16Slot(18, n)
}

func (rcv *ReportMetricsRequest) UringPendingTaskCnt() int32 {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(20))
	if o != 0 {
		return rcv._tab.GetInt32(o + rcv._tab.Pos)
	}
	return 0
}

func (rcv *ReportMetricsRequest) MutateUringPendingTaskCnt(n int32) bool {
	return rcv._tab.MutateInt32Slot(20, n)
}

func (rcv *ReportMetricsRequest) UringTaskAvgLatency() int16 {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(22))
	if o != 0 {
		return rcv._tab.GetInt16(o + rcv._tab.Pos)
	}
	return 0
}

func (rcv *ReportMetricsRequest) MutateUringTaskAvgLatency(n int16) bool {
	return rcv._tab.MutateInt16Slot(22, n)
}

func (rcv *ReportMetricsRequest) NetworkAppendRate() int16 {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(24))
	if o != 0 {
		return rcv._tab.GetInt16(o + rcv._tab.Pos)
	}
	return 0
}

func (rcv *ReportMetricsRequest) MutateNetworkAppendRate(n int16) bool {
	return rcv._tab.MutateInt16Slot(24, n)
}

func (rcv *ReportMetricsRequest) NetworkFetchRate() int16 {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(26))
	if o != 0 {
		return rcv._tab.GetInt16(o + rcv._tab.Pos)
	}
	return 0
}

func (rcv *ReportMetricsRequest) MutateNetworkFetchRate(n int16) bool {
	return rcv._tab.MutateInt16Slot(26, n)
}

func (rcv *ReportMetricsRequest) NetworkFailedAppendRate() int16 {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(28))
	if o != 0 {
		return rcv._tab.GetInt16(o + rcv._tab.Pos)
	}
	return 0
}

func (rcv *ReportMetricsRequest) MutateNetworkFailedAppendRate(n int16) bool {
	return rcv._tab.MutateInt16Slot(28, n)
}

func (rcv *ReportMetricsRequest) NetworkFailedFetchRate() int16 {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(30))
	if o != 0 {
		return rcv._tab.GetInt16(o + rcv._tab.Pos)
	}
	return 0
}

func (rcv *ReportMetricsRequest) MutateNetworkFailedFetchRate(n int16) bool {
	return rcv._tab.MutateInt16Slot(30, n)
}

func (rcv *ReportMetricsRequest) NetworkAppendAvgLatency() int16 {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(32))
	if o != 0 {
		return rcv._tab.GetInt16(o + rcv._tab.Pos)
	}
	return 0
}

func (rcv *ReportMetricsRequest) MutateNetworkAppendAvgLatency(n int16) bool {
	return rcv._tab.MutateInt16Slot(32, n)
}

func (rcv *ReportMetricsRequest) NetworkFetchAvgLatency() int16 {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(34))
	if o != 0 {
		return rcv._tab.GetInt16(o + rcv._tab.Pos)
	}
	return 0
}

func (rcv *ReportMetricsRequest) MutateNetworkFetchAvgLatency(n int16) bool {
	return rcv._tab.MutateInt16Slot(34, n)
}

func (rcv *ReportMetricsRequest) RangeMissingReplicaCnt() int16 {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(36))
	if o != 0 {
		return rcv._tab.GetInt16(o + rcv._tab.Pos)
	}
	return 0
}

func (rcv *ReportMetricsRequest) MutateRangeMissingReplicaCnt(n int16) bool {
	return rcv._tab.MutateInt16Slot(36, n)
}

func (rcv *ReportMetricsRequest) RangeActiveCnt() int16 {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(38))
	if o != 0 {
		return rcv._tab.GetInt16(o + rcv._tab.Pos)
	}
	return 0
}

func (rcv *ReportMetricsRequest) MutateRangeActiveCnt(n int16) bool {
	return rcv._tab.MutateInt16Slot(38, n)
}

func ReportMetricsRequestStart(builder *flatbuffers.Builder) {
	builder.StartObject(18)
}
func ReportMetricsRequestAddDataNode(builder *flatbuffers.Builder, dataNode flatbuffers.UOffsetT) {
	builder.PrependUOffsetTSlot(0, flatbuffers.UOffsetT(dataNode), 0)
}
func ReportMetricsRequestAddDiskInRate(builder *flatbuffers.Builder, diskInRate int64) {
	builder.PrependInt64Slot(1, diskInRate, 0)
}
func ReportMetricsRequestAddDiskOutRate(builder *flatbuffers.Builder, diskOutRate int64) {
	builder.PrependInt64Slot(2, diskOutRate, 0)
}
func ReportMetricsRequestAddDiskFreeSpace(builder *flatbuffers.Builder, diskFreeSpace int64) {
	builder.PrependInt64Slot(3, diskFreeSpace, 0)
}
func ReportMetricsRequestAddDiskUnindexedDataSize(builder *flatbuffers.Builder, diskUnindexedDataSize int64) {
	builder.PrependInt64Slot(4, diskUnindexedDataSize, 0)
}
func ReportMetricsRequestAddMemoryUsed(builder *flatbuffers.Builder, memoryUsed int64) {
	builder.PrependInt64Slot(5, memoryUsed, 0)
}
func ReportMetricsRequestAddUringTaskRate(builder *flatbuffers.Builder, uringTaskRate int16) {
	builder.PrependInt16Slot(6, uringTaskRate, 0)
}
func ReportMetricsRequestAddUringInflightTaskCnt(builder *flatbuffers.Builder, uringInflightTaskCnt int16) {
	builder.PrependInt16Slot(7, uringInflightTaskCnt, 0)
}
func ReportMetricsRequestAddUringPendingTaskCnt(builder *flatbuffers.Builder, uringPendingTaskCnt int32) {
	builder.PrependInt32Slot(8, uringPendingTaskCnt, 0)
}
func ReportMetricsRequestAddUringTaskAvgLatency(builder *flatbuffers.Builder, uringTaskAvgLatency int16) {
	builder.PrependInt16Slot(9, uringTaskAvgLatency, 0)
}
func ReportMetricsRequestAddNetworkAppendRate(builder *flatbuffers.Builder, networkAppendRate int16) {
	builder.PrependInt16Slot(10, networkAppendRate, 0)
}
func ReportMetricsRequestAddNetworkFetchRate(builder *flatbuffers.Builder, networkFetchRate int16) {
	builder.PrependInt16Slot(11, networkFetchRate, 0)
}
func ReportMetricsRequestAddNetworkFailedAppendRate(builder *flatbuffers.Builder, networkFailedAppendRate int16) {
	builder.PrependInt16Slot(12, networkFailedAppendRate, 0)
}
func ReportMetricsRequestAddNetworkFailedFetchRate(builder *flatbuffers.Builder, networkFailedFetchRate int16) {
	builder.PrependInt16Slot(13, networkFailedFetchRate, 0)
}
func ReportMetricsRequestAddNetworkAppendAvgLatency(builder *flatbuffers.Builder, networkAppendAvgLatency int16) {
	builder.PrependInt16Slot(14, networkAppendAvgLatency, 0)
}
func ReportMetricsRequestAddNetworkFetchAvgLatency(builder *flatbuffers.Builder, networkFetchAvgLatency int16) {
	builder.PrependInt16Slot(15, networkFetchAvgLatency, 0)
}
func ReportMetricsRequestAddRangeMissingReplicaCnt(builder *flatbuffers.Builder, rangeMissingReplicaCnt int16) {
	builder.PrependInt16Slot(16, rangeMissingReplicaCnt, 0)
}
func ReportMetricsRequestAddRangeActiveCnt(builder *flatbuffers.Builder, rangeActiveCnt int16) {
	builder.PrependInt16Slot(17, rangeActiveCnt, 0)
}
func ReportMetricsRequestEnd(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	return builder.EndObject()
}
