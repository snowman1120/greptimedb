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

use arrow::datatypes::{DataType as ArrowDataType, Field};
use serde::{Deserialize, Serialize};

use crate::prelude::*;
use crate::value::ListValue;
use crate::vectors::{ListVectorBuilder, MutableVector};

/// Used to represent the List datatype.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListType {
    /// The type of List's inner data.
    inner: Box<ConcreteDataType>,
}

impl Default for ListType {
    fn default() -> Self {
        ListType::new(ConcreteDataType::null_datatype())
    }
}

impl ListType {
    pub fn new(datatype: ConcreteDataType) -> Self {
        ListType {
            inner: Box::new(datatype),
        }
    }
}

impl DataType for ListType {
    fn name(&self) -> &str {
        "List"
    }

    fn logical_type_id(&self) -> LogicalTypeId {
        LogicalTypeId::List
    }

    fn default_value(&self) -> Value {
        Value::List(ListValue::new(None, *self.inner.clone()))
    }

    fn as_arrow_type(&self) -> ArrowDataType {
        let field = Box::new(Field::new("item", self.inner.as_arrow_type(), true));
        ArrowDataType::List(field)
    }

    fn create_mutable_vector(&self, capacity: usize) -> Box<dyn MutableVector> {
        Box::new(ListVectorBuilder::with_type_capacity(
            *self.inner.clone(),
            capacity,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value::ListValue;

    #[test]
    fn test_list_type() {
        let t = ListType::new(ConcreteDataType::boolean_datatype());
        assert_eq!("List", t.name());
        assert_eq!(LogicalTypeId::List, t.logical_type_id());
        assert_eq!(
            Value::List(ListValue::new(None, ConcreteDataType::boolean_datatype())),
            t.default_value()
        );
        assert_eq!(
            ArrowDataType::List(Box::new(Field::new("item", ArrowDataType::Boolean, true))),
            t.as_arrow_type()
        );
    }
}
