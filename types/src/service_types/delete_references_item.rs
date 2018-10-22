// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

use encoding::*;
#[allow(unused_imports)]
use basic_types::*;
use service_types::impls::MessageInfo;
use node_ids::ObjectId;
use node_id::NodeId;
use node_id::ExpandedNodeId;

/// A request to delete a node from the server address space.
#[derive(Debug, Clone, PartialEq)]
pub struct DeleteReferencesItem {
    pub source_node_id: NodeId,
    pub reference_type_id: NodeId,
    pub is_forward: bool,
    pub target_node_id: ExpandedNodeId,
    pub delete_bidirectional: bool,
}

impl MessageInfo for DeleteReferencesItem {
    fn object_id(&self) -> ObjectId {
        ObjectId::DeleteReferencesItem_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<DeleteReferencesItem> for DeleteReferencesItem {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.source_node_id.byte_len();
        size += self.reference_type_id.byte_len();
        size += self.is_forward.byte_len();
        size += self.target_node_id.byte_len();
        size += self.delete_bidirectional.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.source_node_id.encode(stream)?;
        size += self.reference_type_id.encode(stream)?;
        size += self.is_forward.encode(stream)?;
        size += self.target_node_id.encode(stream)?;
        size += self.delete_bidirectional.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_limits: &DecodingLimits) -> EncodingResult<Self> {
        let source_node_id = NodeId::decode(stream, decoding_limits)?;
        let reference_type_id = NodeId::decode(stream, decoding_limits)?;
        let is_forward = bool::decode(stream, decoding_limits)?;
        let target_node_id = ExpandedNodeId::decode(stream, decoding_limits)?;
        let delete_bidirectional = bool::decode(stream, decoding_limits)?;
        Ok(DeleteReferencesItem {
            source_node_id,
            reference_type_id,
            is_forward,
            target_node_id,
            delete_bidirectional,
        })
    }
}
