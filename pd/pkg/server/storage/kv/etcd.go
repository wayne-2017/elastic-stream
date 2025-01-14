// Copyright 2016 TiKV Project Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

package kv

import (
	"bytes"
	"context"
	"fmt"

	"github.com/pkg/errors"
	etcdrpc "go.etcd.io/etcd/api/v3/v3rpc/rpctypes"
	clientv3 "go.etcd.io/etcd/client/v3"
	"go.etcd.io/etcd/server/v3/embed"
	"go.uber.org/zap"

	"github.com/AutoMQ/pd/pkg/util/etcdutil"
	"github.com/AutoMQ/pd/pkg/util/traceutil"
)

const (
	// A buffer in order to reduce times of context switch.
	_watchChanCap = 128
)

var (
	// ErrTxnFailed is the error when etcd transaction failed.
	ErrTxnFailed = errors.New("etcd transaction failed")
	// ErrTooManyTxnOps is the error when the number of operations in a transaction exceeds the limit.
	ErrTooManyTxnOps = errors.New("too many txn operations")
	// ErrCompacted is the error when the requested revision has been compacted.
	ErrCompacted = errors.New("requested revision has been compacted")
)

// Etcd is a kv based on etcd.
type Etcd struct {
	rootPath   []byte
	newTxnFunc func(ctx context.Context) clientv3.Txn // WARNING: do not call `If` on the returned txn.
	maxTxnOps  uint

	watcher clientv3.Watcher

	lg *zap.Logger
}

type Client interface {
	clientv3.KV
	clientv3.Watcher
}

// EtcdParam is used to create a new etcd kv.
type EtcdParam struct {
	Client Client
	// rootPath is the prefix of all keys in etcd.
	RootPath string
	// cmpFunc is used to create a transaction. If cmpFunc is nil, the transaction will not have any condition.
	CmpFunc func() clientv3.Cmp
	// maxTxnOps is the max number of operations in a transaction. It is an etcd server configuration.
	// If maxTxnOps is 0, it will use the default value (128).
	MaxTxnOps uint
}

// NewEtcd creates a new etcd kv.
func NewEtcd(param EtcdParam, lg *zap.Logger) *Etcd {
	logger := lg.With(zap.String("root-path", param.RootPath))
	e := &Etcd{
		rootPath:  []byte(param.RootPath),
		maxTxnOps: param.MaxTxnOps,
		lg:        logger,
	}

	if e.maxTxnOps == 0 {
		e.maxTxnOps = embed.DefaultMaxTxnOps
	}

	if param.CmpFunc != nil {
		e.newTxnFunc = func(ctx context.Context) clientv3.Txn {
			// cmpFunc should be evaluated lazily.
			return etcdutil.NewTxn(ctx, param.Client, logger.With(traceutil.TraceLogField(ctx))).If(param.CmpFunc())
		}
	} else {
		e.newTxnFunc = func(ctx context.Context) clientv3.Txn {
			return etcdutil.NewTxn(ctx, param.Client, logger.With(traceutil.TraceLogField(ctx)))
		}
	}

	e.watcher = param.Client

	return e
}

// Get returns ErrTxnFailed if EtcdParam.CmpFunc evaluates to false.
func (e *Etcd) Get(ctx context.Context, k []byte) ([]byte, error) {
	if len(k) == 0 {
		return nil, nil
	}

	kvs, err := e.BatchGet(ctx, [][]byte{k}, false)
	if err != nil {
		return nil, errors.Wrap(err, "kv get")
	}

	for _, kv := range kvs {
		if bytes.Equal(kv.Key, k) {
			return kv.Value, nil
		}
	}
	return nil, nil
}

// BatchGet returns ErrTxnFailed if EtcdParam.CmpFunc evaluates to false.
// If inTxn is true, BatchGet returns ErrTooManyTxnOps if the number of keys exceeds EtcdParam.MaxTxnOps.
func (e *Etcd) BatchGet(ctx context.Context, keys [][]byte, inTxn bool) ([]KeyValue, error) {
	if len(keys) == 0 {
		return nil, nil
	}
	batchSize := int(e.maxTxnOps)
	if inTxn && len(keys) > batchSize {
		return nil, errors.Wrap(ErrTooManyTxnOps, "kv batch get")
	}

	kvs := make([]KeyValue, 0, len(keys))

	for i := 0; i < len(keys); i += batchSize {
		end := i + batchSize
		if end > len(keys) {
			end = len(keys)
		}
		batchKeys := keys[i:end]

		ops := make([]clientv3.Op, 0, len(batchKeys))
		for _, k := range batchKeys {
			if len(k) == 0 {
				continue
			}
			key := e.addPrefix(k)
			ops = append(ops, clientv3.OpGet(string(key)))
		}

		txn := e.newTxnFunc(ctx).Then(ops...)
		resp, err := txn.Commit()
		if err != nil {
			return nil, errors.Wrap(err, "kv batch get")
		}
		if !resp.Succeeded {
			return nil, errors.Wrap(ErrTxnFailed, "kv batch get")
		}

		for _, resp := range resp.Responses {
			rangeResp := resp.GetResponseRange()
			if rangeResp == nil {
				continue
			}
			for _, kv := range rangeResp.Kvs {
				if !e.hasPrefix(kv.Key) {
					continue
				}
				kvs = append(kvs, KeyValue{
					Key:   e.trimPrefix(kv.Key),
					Value: kv.Value,
				})
			}
		}
	}

	return kvs, nil
}

// GetByRange returns ErrTxnFailed if EtcdParam.CmpFunc evaluates to false.
// GetByRange returns ErrCompacted if the requested revision has been compacted.
func (e *Etcd) GetByRange(ctx context.Context, r Range, rev int64, limit int64, desc bool) ([]KeyValue, int64, bool, error) {
	if len(r.StartKey) == 0 {
		return nil, 0, false, nil
	}

	startKey := e.addPrefix(r.StartKey)
	endKey := e.addPrefix(r.EndKey)

	opts := []clientv3.OpOption{clientv3.WithRange(string(endKey))}
	if rev > 0 {
		opts = append(opts, clientv3.WithRev(rev))
	}
	if limit > 0 {
		opts = append(opts, clientv3.WithLimit(limit))
	}
	if desc {
		opts = append(opts, clientv3.WithSort(clientv3.SortByKey, clientv3.SortDescend))
	}

	resp, err := e.newTxnFunc(ctx).Then(clientv3.OpGet(string(startKey), opts...)).Commit()
	if err != nil {
		if err == etcdrpc.ErrCompacted {
			return nil, 0, false, errors.Wrapf(ErrCompacted, "kv get by range, revision %d", rev)
		}
		return nil, 0, false, errors.Wrap(err, "kv get by range")
	}
	if !resp.Succeeded {
		return nil, 0, false, errors.Wrap(ErrTxnFailed, "kv get by range")
	}

	// when the transaction succeeds, the number of responses is always 1 and is always a range response.
	rangeResp := resp.Responses[0].GetResponseRange()

	kvs := make([]KeyValue, 0, len(rangeResp.Kvs))
	for _, kv := range rangeResp.Kvs {
		if !e.hasPrefix(kv.Key) {
			continue
		}
		kvs = append(kvs, KeyValue{
			Key:   e.trimPrefix(kv.Key),
			Value: kv.Value,
		})
	}

	returnedRV := rev
	if returnedRV <= 0 {
		returnedRV = rangeResp.Header.Revision
	}

	return kvs, returnedRV, rangeResp.More, nil
}

func (e *Etcd) Watch(ctx context.Context, prefix []byte, rev int64, filter Filter) Watcher {
	ctx = clientv3.WithRequireLeader(ctx)
	key := e.addPrefix(prefix)
	opts := []clientv3.OpOption{clientv3.WithPrevKV(), clientv3.WithPrefix()}
	if rev > 0 {
		opts = append(opts, clientv3.WithRev(rev))
	}

	wch := e.watcher.Watch(ctx, string(key), opts...)
	logger := e.lg.With(zap.ByteString("prefix", prefix), zap.Int64("revision", rev), traceutil.TraceLogField(ctx))
	watcher := e.newWatchChan(ctx, wch, filter, logger)
	go watcher.run()

	return watcher
}

// Put returns ErrTxnFailed if EtcdParam.CmpFunc evaluates to false.
func (e *Etcd) Put(ctx context.Context, k, v []byte, prevKV bool) ([]byte, error) {
	if len(k) == 0 {
		return nil, nil
	}

	prevKVs, err := e.BatchPut(ctx, []KeyValue{{Key: k, Value: v}}, prevKV, false)
	if err != nil {
		return nil, errors.Wrap(err, "kv put")
	}

	if !prevKV {
		return nil, nil
	}

	for _, kv := range prevKVs {
		if bytes.Equal(kv.Key, k) {
			return kv.Value, nil
		}
	}
	return nil, nil
}

// BatchPut returns ErrTxnFailed if EtcdParam.CmpFunc evaluates to false.
// If inTxn is true, BatchPut returns ErrTooManyTxnOps if the number of kvs exceeds EtcdParam.MaxTxnOps.
func (e *Etcd) BatchPut(ctx context.Context, kvs []KeyValue, prevKV bool, inTxn bool) ([]KeyValue, error) {
	if len(kvs) == 0 {
		return nil, nil
	}
	batchSize := int(e.maxTxnOps)
	if inTxn && len(kvs) > batchSize {
		return nil, errors.Wrap(ErrTooManyTxnOps, "kv batch put")
	}

	var prevKVs []KeyValue
	if prevKV {
		prevKVs = make([]KeyValue, 0, len(kvs))
	}

	for i := 0; i < len(kvs); i += batchSize {
		end := i + batchSize
		if end > len(kvs) {
			end = len(kvs)
		}
		batchKVs := kvs[i:end]

		ops := make([]clientv3.Op, 0, len(batchKVs))
		var opts []clientv3.OpOption
		if prevKV {
			opts = append(opts, clientv3.WithPrevKV())
		}
		for _, kv := range batchKVs {
			if len(kv.Key) == 0 {
				continue
			}
			key := e.addPrefix(kv.Key)
			ops = append(ops, clientv3.OpPut(string(key), string(kv.Value), opts...))
		}

		txn := e.newTxnFunc(ctx).Then(ops...)
		resp, err := txn.Commit()
		if err != nil {
			return nil, errors.Wrap(err, "kv batch put")
		}
		if !resp.Succeeded {
			return nil, errors.Wrap(ErrTxnFailed, "kv batch put")
		}

		if !prevKV {
			continue
		}
		for _, resp := range resp.Responses {
			putResp := resp.GetResponsePut()
			if putResp.PrevKv == nil {
				continue
			}
			if !e.hasPrefix(putResp.PrevKv.Key) {
				continue
			}
			prevKVs = append(prevKVs, KeyValue{
				Key:   e.trimPrefix(putResp.PrevKv.Key),
				Value: putResp.PrevKv.Value,
			})
		}
	}

	return prevKVs, nil
}

// Delete returns ErrTxnFailed if EtcdParam.CmpFunc evaluates to false.
func (e *Etcd) Delete(ctx context.Context, k []byte, prevKV bool) ([]byte, error) {
	if len(k) == 0 {
		return nil, nil
	}

	prevKVs, err := e.BatchDelete(ctx, [][]byte{k}, prevKV, false)
	if err != nil {
		return nil, errors.Wrap(err, "kv delete")
	}

	if !prevKV {
		return nil, nil
	}
	for _, kv := range prevKVs {
		if bytes.Equal(kv.Key, k) {
			return kv.Value, nil
		}
	}
	return nil, nil
}

// BatchDelete returns ErrTxnFailed if EtcdParam.CmpFunc evaluates to false.
// If inTxn is true, BatchDelete returns ErrTooManyTxnOps if the number of keys exceeds EtcdParam.MaxTxnOps.
func (e *Etcd) BatchDelete(ctx context.Context, keys [][]byte, prevKV bool, inTxn bool) ([]KeyValue, error) {
	if len(keys) == 0 {
		return nil, nil
	}
	batchSize := int(e.maxTxnOps)
	if inTxn && len(keys) > batchSize {
		return nil, errors.Wrap(ErrTooManyTxnOps, "kv batch delete")
	}

	var prevKVs []KeyValue
	if prevKV {
		prevKVs = make([]KeyValue, 0, len(keys))
	}

	for i := 0; i < len(keys); i += batchSize {
		end := i + batchSize
		if end > len(keys) {
			end = len(keys)
		}
		batchKeys := keys[i:end]

		ops := make([]clientv3.Op, 0, len(batchKeys))
		var opts []clientv3.OpOption
		if prevKV {
			opts = append(opts, clientv3.WithPrevKV())
		}
		for _, k := range batchKeys {
			if len(k) == 0 {
				continue
			}
			key := e.addPrefix(k)
			ops = append(ops, clientv3.OpDelete(string(key), opts...))
		}

		txn := e.newTxnFunc(ctx).Then(ops...)
		resp, err := txn.Commit()
		if err != nil {
			return nil, errors.Wrap(err, "kv batch delete")
		}
		if !resp.Succeeded {
			return nil, errors.Wrap(ErrTxnFailed, "kv batch delete")
		}

		if !prevKV {
			continue
		}
		for _, resp := range resp.Responses {
			deleteResp := resp.GetResponseDeleteRange()
			for _, kv := range deleteResp.PrevKvs {
				if !e.hasPrefix(kv.Key) {
					continue
				}
				prevKVs = append(prevKVs, KeyValue{
					Key:   e.trimPrefix(kv.Key),
					Value: kv.Value,
				})
			}
		}
	}

	return prevKVs, nil
}

func (e *Etcd) GetPrefixRangeEnd(p []byte) []byte {
	prefix := e.addPrefix(p)
	end := []byte(clientv3.GetPrefixRangeEnd(string(prefix)))
	return e.trimPrefix(end)
}

func (e *Etcd) Logger() *zap.Logger {
	return e.lg
}

func (e *Etcd) addPrefix(k []byte) []byte {
	return bytes.Join([][]byte{e.rootPath, k}, []byte(KeySeparator))
}

func (e *Etcd) trimPrefix(k []byte) []byte {
	return k[len(e.rootPath)+len(KeySeparator):]
}

func (e *Etcd) hasPrefix(k []byte) bool {
	return len(k) >= len(e.rootPath)+len(KeySeparator) &&
		bytes.Equal(k[:len(e.rootPath)], e.rootPath) &&
		string(k[len(e.rootPath):len(e.rootPath)+len(KeySeparator)]) == KeySeparator
}

type watchChan struct {
	e *Etcd

	ctx    context.Context
	cancel context.CancelFunc

	ch         clientv3.WatchChan
	filter     Filter
	resultChan chan Events

	lg *zap.Logger
}

func (e *Etcd) newWatchChan(ctx context.Context, ch clientv3.WatchChan, filter Filter, logger *zap.Logger) *watchChan {
	ctx, cancel := context.WithCancel(ctx)
	return &watchChan{
		e:          e,
		ctx:        ctx,
		cancel:     cancel,
		ch:         ch,
		filter:     filter,
		resultChan: make(chan Events, _watchChanCap),
		lg:         logger,
	}
}

func (w *watchChan) Close() {
	w.cancel()
}

func (w *watchChan) EventChan() <-chan Events {
	return w.resultChan
}

func (w *watchChan) run() {
	defer func() {
		close(w.resultChan)
	}()

	for {
		select {
		case <-w.ctx.Done():
			return
		case resp, ok := <-w.ch:
			if !ok {
				return
			}
			if err := resp.Err(); err != nil {
				w.sendError(err)
				return
			}
			if len(resp.Events) == 0 {
				continue
			}
			events := make([]Event, 0, len(resp.Events))
			for _, e := range resp.Events {
				parsed, err := w.parseEvent(e)
				if err != nil {
					w.sendError(err)
					return
				}
				if w.filter != nil && !w.filter(parsed) {
					continue
				}

				events = append(events, parsed)
			}
			if len(events) == 0 {
				continue
			}
			w.sendEvents(Events{Events: events, Revision: resp.Header.Revision})
		}
	}
}

func (w *watchChan) sendError(err error) {
	logger := w.lg.With(zap.Error(err))
	if errors.Is(err, etcdrpc.ErrCompacted) {
		err = ErrCompacted
		logger.Warn("watch chan error: compaction")
	} else {
		logger.Error("watch chan error")
	}

	select {
	case w.resultChan <- Events{Error: err}:
	case <-w.ctx.Done():
		logger.Warn("error is dropped due to context done")
	}
}

func (w *watchChan) sendEvents(e Events) {
	logger := w.lg.With(zap.Int("events-cnt", len(e.Events)), zap.Int64("events-revision", e.Revision))
	if len(w.resultChan) == cap(w.resultChan) {
		logger.Warn("watch chan is full", zap.Int("cap", cap(w.resultChan)))
	}
	select {
	case w.resultChan <- e:
		if logger.Core().Enabled(zap.DebugLevel) {
			fields := make([]zap.Field, 0, len(e.Events)*3)
			for i, ev := range e.Events {
				fields = append(fields,
					zap.String(fmt.Sprintf("events-%d-type", i), string(ev.Type)),
					zap.ByteString(fmt.Sprintf("events-%d-key", i), ev.Key),
					zap.ByteString(fmt.Sprintf("events-%d-value", i), ev.Value),
				)
			}
			logger.Debug("send events to watch chan", fields...)
		}
	case <-w.ctx.Done():
		logger.Warn("events are dropped due to context done")
	}
}

func (w *watchChan) parseEvent(e *clientv3.Event) (Event, error) {
	var ret Event
	switch {
	case e.Type == clientv3.EventTypeDelete:
		if e.PrevKv == nil {
			// The previous value has been compacted.
			return Event{}, errors.Errorf("etcd event has no previous key-value pair. key: %q, modRevision: %d, type: %s", e.Kv.Key, e.Kv.ModRevision, e.Type)
		}
		ret = Event{
			Type:  Deleted,
			Key:   w.e.trimPrefix(e.Kv.Key),
			Value: e.PrevKv.Value,
		}
	case e.IsCreate():
		ret = Event{
			Type:  Added,
			Key:   w.e.trimPrefix(e.Kv.Key),
			Value: e.Kv.Value,
		}
	case e.IsModify():
		ret = Event{
			Type:  Modified,
			Key:   w.e.trimPrefix(e.Kv.Key),
			Value: e.Kv.Value,
		}
	default:
		// Should never happen as etcd clientv3.Event has no other types.
	}
	return ret, nil
}
