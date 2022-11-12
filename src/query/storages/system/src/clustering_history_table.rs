// Copyright 2022 Datafuse Labs.
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

use common_exception::Result;
use common_expression::ColumnBuilder;
use common_expression::DataField;
use common_expression::DataSchemaRefExt;
use common_expression::NumberDataType;
use common_expression::NumberScalar;
use common_expression::Scalar;
use common_expression::SchemaDataType;

use crate::SystemLogElement;
use crate::SystemLogQueue;
use crate::SystemLogTable;

#[derive(Clone)]
pub struct ClusteringHistoryLogElement {
    pub start_time: i64,
    pub end_time: i64,
    pub database: String,
    pub table: String,
    pub reclustered_bytes: u64,
    pub reclustered_rows: u64,
}

impl SystemLogElement for ClusteringHistoryLogElement {
    const TABLE_NAME: &'static str = "clustering_history";

    fn schema() -> DataSchemaRef {
        DataSchemaRefExt::create(vec![
            DataField::new("start_time", SchemaDataType::Timestamp),
            DataField::new("end_time", SchemaDataType::Timestamp),
            DataField::new("database", SchemaDataType::String),
            DataField::new("table", SchemaDataType::String),
            DataField::new(
                "reclustered_bytes",
                SchemaDataType::Number(NumberDataType::UInt64),
            ),
            DataField::new(
                "reclustered_rows",
                SchemaDataType::Number(NumberDataType::UInt64),
            ),
        ])
    }

    fn fill_to_data_block(&self, columns: &mut Vec<ColumnBuilder>) -> Result<()> {
        let mut columns = columns.iter_mut();
        columns
            .next()
            .unwrap()
            .push(Scalar::Timestamp(self.start_time).as_ref());
        columns
            .next()
            .unwrap()
            .push(Scalar::Timestamp(self.end_time).as_ref());
        columns
            .next()
            .unwrap()
            .push(Scalar::String(self.database.as_bytes().to_vec()).as_ref());
        columns
            .next()
            .unwrap()
            .push(Scalar::String(self.table.as_bytes().to_vec()).as_ref());
        columns
            .next()
            .unwrap()
            .push(Scalar::Number(NumberScalar::UInt64(self.reclustered_bytes)).as_ref());
        columns
            .next()
            .unwrap()
            .push(Scalar::Number(NumberScalar::UInt64(self.reclustered_rows)).as_ref());
    }
}

pub type ClusteringHistoryQueue = SystemLogQueue<ClusteringHistoryLogElement>;
pub type ClusteringHistoryTable = SystemLogTable<ClusteringHistoryLogElement>;
