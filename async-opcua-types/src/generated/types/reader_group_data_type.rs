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
pub struct ReaderGroupDataType {
    pub name: opcua::types::string::UAString,
    pub enabled: bool,
    pub security_mode: super::enums::MessageSecurityMode,
    pub security_group_id: opcua::types::string::UAString,
    pub security_key_services: Option<Vec<super::endpoint_description::EndpointDescription>>,
    pub max_network_message_size: u32,
    pub group_properties: Option<Vec<super::key_value_pair::KeyValuePair>>,
    pub transport_settings: opcua::types::extension_object::ExtensionObject,
    pub message_settings: opcua::types::extension_object::ExtensionObject,
    pub data_set_readers: Option<Vec<super::data_set_reader_data_type::DataSetReaderDataType>>,
}
impl opcua::types::MessageInfo for ReaderGroupDataType {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::ReaderGroupDataType_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::ReaderGroupDataType_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::ReaderGroupDataType_Encoding_DefaultXml
    }
    fn data_type_id(&self) -> opcua::types::DataTypeId {
        opcua::types::DataTypeId::ReaderGroupDataType
    }
}
