use std::sync::Arc;

use daft_core::schema::SchemaRef;
use daft_dsl::ExprRef;

use crate::{
    physical_plan::PhysicalPlan, sink_info::OutputFileInfo, source_info::ExternalInfo,
    PartitionSpec,
};

#[derive(Debug)]
pub struct TabularScanJson {
    pub schema: SchemaRef,
    pub external_info: ExternalInfo,
    pub partition_spec: Arc<PartitionSpec>,
    pub limit: Option<usize>,
    pub filters: Vec<ExprRef>,
}

impl TabularScanJson {
    pub(crate) fn new(
        schema: SchemaRef,
        external_info: ExternalInfo,
        partition_spec: Arc<PartitionSpec>,
        limit: Option<usize>,
        filters: Vec<ExprRef>,
    ) -> Self {
        Self {
            schema,
            external_info,
            partition_spec,
            limit,
            filters,
        }
    }
}

#[derive(Debug)]
pub struct TabularWriteJson {
    pub schema: SchemaRef,
    pub file_info: OutputFileInfo,
    // Upstream node.
    pub input: Arc<PhysicalPlan>,
}

impl TabularWriteJson {
    pub(crate) fn new(
        schema: SchemaRef,
        file_info: OutputFileInfo,
        input: Arc<PhysicalPlan>,
    ) -> Self {
        Self {
            schema,
            file_info,
            input,
        }
    }
}