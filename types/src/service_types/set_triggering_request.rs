// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

use encoding::*;
#[allow(unused_imports)]
use basic_types::*;
use service_types::impls::MessageInfo;
use node_ids::ObjectId;
use service_types::impls::RequestHeader;

#[derive(Debug, Clone, PartialEq)]
pub struct SetTriggeringRequest {
    pub request_header: RequestHeader,
    pub subscription_id: u32,
    pub triggering_item_id: u32,
    pub links_to_add: Option<Vec<u32>>,
    pub links_to_remove: Option<Vec<u32>>,
}

impl MessageInfo for SetTriggeringRequest {
    fn object_id(&self) -> ObjectId {
        ObjectId::SetTriggeringRequest_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<SetTriggeringRequest> for SetTriggeringRequest {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.request_header.byte_len();
        size += self.subscription_id.byte_len();
        size += self.triggering_item_id.byte_len();
        size += byte_len_array(&self.links_to_add);
        size += byte_len_array(&self.links_to_remove);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.request_header.encode(stream)?;
        size += self.subscription_id.encode(stream)?;
        size += self.triggering_item_id.encode(stream)?;
        size += write_array(stream, &self.links_to_add)?;
        size += write_array(stream, &self.links_to_remove)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_limits: &DecodingLimits) -> EncodingResult<Self> {
        let request_header = RequestHeader::decode(stream, decoding_limits)?;
        let subscription_id = u32::decode(stream, decoding_limits)?;
        let triggering_item_id = u32::decode(stream, decoding_limits)?;
        let links_to_add: Option<Vec<u32>> = read_array(stream, decoding_limits)?;
        let links_to_remove: Option<Vec<u32>> = read_array(stream, decoding_limits)?;
        Ok(SetTriggeringRequest {
            request_header,
            subscription_id,
            triggering_item_id,
            links_to_add,
            links_to_remove,
        })
    }
}
