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
pub struct SubscriptionAcknowledgement {
    pub subscription_id: u32,
    pub sequence_number: u32,
}
impl opcua::types::MessageInfo for SubscriptionAcknowledgement {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::SubscriptionAcknowledgement_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::SubscriptionAcknowledgement_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::SubscriptionAcknowledgement_Encoding_DefaultXml
    }
    fn data_type_id(&self) -> opcua::types::DataTypeId {
        opcua::types::DataTypeId::SubscriptionAcknowledgement
    }
}
