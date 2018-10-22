// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

use encoding::*;
#[allow(unused_imports)]
use basic_types::*;
use service_types::impls::MessageInfo;
use node_ids::ObjectId;
use service_types::impls::RequestHeader;
use service_types::enums::SecurityTokenRequestType;
use data_types::MessageSecurityMode;
use byte_string::ByteString;

/// Creates a secure channel with a server.
#[derive(Debug, Clone, PartialEq)]
pub struct OpenSecureChannelRequest {
    pub request_header: RequestHeader,
    pub client_protocol_version: u32,
    pub request_type: SecurityTokenRequestType,
    pub security_mode: MessageSecurityMode,
    pub client_nonce: ByteString,
    pub requested_lifetime: u32,
}

impl MessageInfo for OpenSecureChannelRequest {
    fn object_id(&self) -> ObjectId {
        ObjectId::OpenSecureChannelRequest_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<OpenSecureChannelRequest> for OpenSecureChannelRequest {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.request_header.byte_len();
        size += self.client_protocol_version.byte_len();
        size += self.request_type.byte_len();
        size += self.security_mode.byte_len();
        size += self.client_nonce.byte_len();
        size += self.requested_lifetime.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.request_header.encode(stream)?;
        size += self.client_protocol_version.encode(stream)?;
        size += self.request_type.encode(stream)?;
        size += self.security_mode.encode(stream)?;
        size += self.client_nonce.encode(stream)?;
        size += self.requested_lifetime.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_limits: &DecodingLimits) -> EncodingResult<Self> {
        let request_header = RequestHeader::decode(stream, decoding_limits)?;
        let client_protocol_version = u32::decode(stream, decoding_limits)?;
        let request_type = SecurityTokenRequestType::decode(stream, decoding_limits)?;
        let security_mode = MessageSecurityMode::decode(stream, decoding_limits)?;
        let client_nonce = ByteString::decode(stream, decoding_limits)?;
        let requested_lifetime = u32::decode(stream, decoding_limits)?;
        Ok(OpenSecureChannelRequest {
            request_header,
            client_protocol_version,
            request_type,
            security_mode,
            client_nonce,
            requested_lifetime,
        })
    }
}
