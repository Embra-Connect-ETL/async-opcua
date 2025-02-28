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
pub struct ReadEventDetails {
    pub num_values_per_node: u32,
    pub start_time: opcua::types::date_time::DateTime,
    pub end_time: opcua::types::date_time::DateTime,
    pub filter: super::event_filter::EventFilter,
}
impl opcua::types::MessageInfo for ReadEventDetails {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::ReadEventDetails_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::ReadEventDetails_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::ReadEventDetails_Encoding_DefaultXml
    }
    fn data_type_id(&self) -> opcua::types::DataTypeId {
        opcua::types::DataTypeId::ReadEventDetails
    }
}
