// Copyright 2022 TiKV Project Authors.
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

package endpoint

import (
	"go.uber.org/zap"

	"github.com/AutoMQ/placement-manager/pkg/server/storage/kv"
)

// Endpoint is the base underlying storage endpoint for all other upper
// specific storage backends. It should define some common storage interfaces and operations,
// which provides the default implementations for all kinds of storages.
type Endpoint struct {
	kv.KV

	lg *zap.Logger
}

// NewEndpoint creates a new base storage endpoint with the given KV.
// It should be embedded inside a storage backend.
func NewEndpoint(kv kv.KV, logger *zap.Logger) *Endpoint {
	return &Endpoint{
		KV: kv,
		lg: logger,
	}
}
