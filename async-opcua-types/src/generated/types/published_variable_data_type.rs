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
pub struct PublishedVariableDataType {
    pub published_variable: opcua::types::node_id::NodeId,
    pub attribute_id: u32,
    pub sampling_interval_hint: f64,
    pub deadband_type: u32,
    pub deadband_value: f64,
    pub index_range: opcua::types::string::UAString,
    pub substitute_value: opcua::types::variant::Variant,
    pub meta_data_properties: Option<Vec<opcua::types::qualified_name::QualifiedName>>,
}
impl opcua::types::MessageInfo for PublishedVariableDataType {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::PublishedVariableDataType_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::PublishedVariableDataType_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::PublishedVariableDataType_Encoding_DefaultXml
    }
    fn data_type_id(&self) -> opcua::types::DataTypeId {
        opcua::types::DataTypeId::PublishedVariableDataType
    }
}
