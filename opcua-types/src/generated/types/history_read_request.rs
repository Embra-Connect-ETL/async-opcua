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
pub struct HistoryReadRequest {
    pub request_header: opcua::types::request_header::RequestHeader,
    pub history_read_details: opcua::types::extension_object::ExtensionObject,
    pub timestamps_to_return: super::enums::TimestampsToReturn,
    pub release_continuation_points: bool,
    pub nodes_to_read: Option<Vec<super::history_read_value_id::HistoryReadValueId>>,
}
impl opcua::types::MessageInfo for HistoryReadRequest {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::HistoryReadRequest_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::HistoryReadRequest_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::HistoryReadRequest_Encoding_DefaultXml
    }
    fn data_type_id(&self) -> opcua::types::DataTypeId {
        opcua::types::DataTypeId::HistoryReadRequest
    }
}
