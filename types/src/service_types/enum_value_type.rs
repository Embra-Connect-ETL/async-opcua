// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

use encoding::*;
#[allow(unused_imports)]
use basic_types::*;
use service_types::impls::MessageInfo;
use node_ids::ObjectId;
use basic_types::LocalizedText;

/// A mapping between a value of an enumerated type and a name and description.
#[derive(Debug, Clone, PartialEq)]
pub struct EnumValueType {
    pub value: i64,
    pub display_name: LocalizedText,
    pub description: LocalizedText,
}

impl MessageInfo for EnumValueType {
    fn object_id(&self) -> ObjectId {
        ObjectId::EnumValueType_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<EnumValueType> for EnumValueType {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.value.byte_len();
        size += self.display_name.byte_len();
        size += self.description.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.value.encode(stream)?;
        size += self.display_name.encode(stream)?;
        size += self.description.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_limits: &DecodingLimits) -> EncodingResult<Self> {
        let value = i64::decode(stream, decoding_limits)?;
        let display_name = LocalizedText::decode(stream, decoding_limits)?;
        let description = LocalizedText::decode(stream, decoding_limits)?;
        Ok(EnumValueType {
            value,
            display_name,
            description,
        })
    }
}
