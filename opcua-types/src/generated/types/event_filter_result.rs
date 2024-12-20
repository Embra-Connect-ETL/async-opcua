// This file was autogenerated from schemas/1.05/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE

// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
#[allow(unused)]
mod opcua {
    pub use crate as types;
}
#[derive(Debug, Clone, PartialEq, opcua::types::BinaryEncodable, opcua::types::BinaryDecodable)]
#[cfg_attr(
    feature = "json",
    derive(opcua::types::JsonEncodable, opcua::types::JsonDecodable)
)]
#[cfg_attr(feature = "xml", derive(opcua::types::FromXml))]
#[derive(Default)]
pub struct EventFilterResult {
    pub select_clause_results: Option<Vec<opcua::types::status_code::StatusCode>>,
    pub select_clause_diagnostic_infos: Option<Vec<opcua::types::diagnostic_info::DiagnosticInfo>>,
    pub where_clause_result: super::content_filter_result::ContentFilterResult,
}
impl opcua::types::MessageInfo for EventFilterResult {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::EventFilterResult_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::EventFilterResult_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::EventFilterResult_Encoding_DefaultXml
    }
    fn data_type_id(&self) -> opcua::types::DataTypeId {
        opcua::types::DataTypeId::EventFilterResult
    }
}
