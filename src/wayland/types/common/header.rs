use crate::wayland::types::common::{
    argument::{Argument, Object},
    parse_utils::ParseResult,
};

#[derive(Debug, Clone)]
pub struct Header {
    pub opcode: u16,
    pub object_id: Object,
    pub size: u16,
}

impl Header {
    pub fn new(object_id: Object, opcode: u16, size: u16) -> Header {
        Header {
            opcode,
            object_id,
            size,
        }
    }

    pub fn to_vec(&self) -> ParseResult<Vec<u8>> {
        self.size.encode_extend(
            self.opcode
                .encode_extend(self.object_id.encode_extend(Vec::new())?)?,
        )
    }
}
