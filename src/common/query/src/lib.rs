// Copyright 2022 Greptime Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use common_recordbatch::{RecordBatches, SendableRecordBatchStream};

pub mod columnar_value;
pub mod error;
mod function;
pub mod logical_plan;
pub mod physical_plan;
pub mod prelude;
mod signature;

// sql output
pub enum Output {
    AffectedRows(usize),
    RecordBatches(RecordBatches),
    Stream(SendableRecordBatchStream),
}

pub use datafusion::physical_plan::ExecutionPlan as DfPhysicalPlan;
