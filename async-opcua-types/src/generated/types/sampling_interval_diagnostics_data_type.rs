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
pub struct SamplingIntervalDiagnosticsDataType {
    pub sampling_interval: f64,
    pub monitored_item_count: u32,
    pub max_monitored_item_count: u32,
    pub disabled_monitored_item_count: u32,
}
impl opcua::types::MessageInfo for SamplingIntervalDiagnosticsDataType {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::SamplingIntervalDiagnosticsDataType_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::SamplingIntervalDiagnosticsDataType_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::SamplingIntervalDiagnosticsDataType_Encoding_DefaultXml
    }
    fn data_type_id(&self) -> opcua::types::DataTypeId {
        opcua::types::DataTypeId::SamplingIntervalDiagnosticsDataType
    }
}
