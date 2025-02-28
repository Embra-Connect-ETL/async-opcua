// This file was autogenerated from schemas/1.05/Opc.Ua.Types.bsd by async-opcua-codegen
//
// DO NOT EDIT THIS FILE

// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
#[allow(unused)]
mod opcua {
    pub use crate as types;
}
#[opcua::types::ua_encodable]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ReadProcessedDetails {
    pub start_time: opcua::types::date_time::DateTime,
    pub end_time: opcua::types::date_time::DateTime,
    pub processing_interval: f64,
    pub aggregate_type: Option<Vec<opcua::types::node_id::NodeId>>,
    pub aggregate_configuration: super::aggregate_configuration::AggregateConfiguration,
}
impl opcua::types::MessageInfo for ReadProcessedDetails {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::ReadProcessedDetails_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::ReadProcessedDetails_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::ReadProcessedDetails_Encoding_DefaultXml
    }
    fn data_type_id(&self) -> opcua::types::DataTypeId {
        opcua::types::DataTypeId::ReadProcessedDetails
    }
}
