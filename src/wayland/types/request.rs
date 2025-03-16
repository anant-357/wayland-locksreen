use crate::wayland::types::common::{
    argument::{Argument, NewId, Object},
    header::Header,
    parse_utils::ParseResult,
};

#[derive(Debug)]
pub enum Message {
    GetRegistry { registry: Object },
    Sync { callback: u32 },
    Bind { name: Object, id: NewId },
}

impl Message {
    fn to_vec(&self) -> ParseResult<Vec<u8>> {
        match self {
            Self::Sync { callback } => callback.encode(),
            Self::Bind { name, id } => {
                let buffer = name.encode_extend(Vec::new())?;
                id.encode_extend(buffer)
            }
            Self::GetRegistry { registry } => registry.encode(),
        }
    }
}

#[derive(Debug)]
pub struct RequestMessage {
    header: Header,
    payload: Message,
}

impl RequestMessage {
    pub fn build(object_id: Object, opcode: u16, size: u16, message: Message) -> Self {
        let header = Header::new(object_id, opcode, size);
        Self {
            header,
            payload: message,
        }
    }

    pub fn to_vec(&self) -> ParseResult<Vec<u8>> {
        let mut request = self.header.to_vec()?;
        request.extend(self.payload.to_vec()?);
        Ok(request)
    }

   }
