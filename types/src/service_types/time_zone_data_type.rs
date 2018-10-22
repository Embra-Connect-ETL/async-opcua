// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

use encoding::*;
#[allow(unused_imports)]
use basic_types::*;
use service_types::impls::MessageInfo;
use node_ids::ObjectId;

#[derive(Debug, Clone, PartialEq)]
pub struct TimeZoneDataType {
    pub offset: i16,
    pub daylight_saving_in_offset: bool,
}

impl MessageInfo for TimeZoneDataType {
    fn object_id(&self) -> ObjectId {
        ObjectId::TimeZoneDataType_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<TimeZoneDataType> for TimeZoneDataType {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.offset.byte_len();
        size += self.daylight_saving_in_offset.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.offset.encode(stream)?;
        size += self.daylight_saving_in_offset.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_limits: &DecodingLimits) -> EncodingResult<Self> {
        let offset = i16::decode(stream, decoding_limits)?;
        let daylight_saving_in_offset = bool::decode(stream, decoding_limits)?;
        Ok(TimeZoneDataType {
            offset,
            daylight_saving_in_offset,
        })
    }
}
