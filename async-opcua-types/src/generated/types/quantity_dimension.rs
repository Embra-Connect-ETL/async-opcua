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
pub struct QuantityDimension {
    pub mass_exponent: i8,
    pub length_exponent: i8,
    pub time_exponent: i8,
    pub electric_current_exponent: i8,
    pub amount_of_substance_exponent: i8,
    pub luminous_intensity_exponent: i8,
    pub absolute_temperature_exponent: i8,
    pub dimensionless_exponent: i8,
}
impl opcua::types::MessageInfo for QuantityDimension {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::QuantityDimension_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::QuantityDimension_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::QuantityDimension_Encoding_DefaultXml
    }
    fn data_type_id(&self) -> opcua::types::DataTypeId {
        opcua::types::DataTypeId::QuantityDimension
    }
}
