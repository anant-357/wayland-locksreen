use crate::wayland::types::common::{
    argument::{Argument, NewId, Object},
    header::Header,
    parse_utils::WaylandResult,
};

#[derive(Debug)]
pub enum Message {
    GetRegistry { registry: Object },
    Sync { callback: u32 },
    Bind { name: Object, id: NewId },
    Lock { ext_session_lock_v1: Object },
    Empty,
}

impl Message {
    fn to_vec(&self) -> WaylandResult<Vec<u8>> {
        match self {
            Self::Sync { callback } => callback.encode(),
            Self::Bind { name, id } => {
                let buffer = name.encode_extend(Vec::new())?;
                id.encode_extend(buffer)
            }
            Self::GetRegistry { registry } => registry.encode(),
            Self::Lock {
                ext_session_lock_v1,
            } => ext_session_lock_v1.encode(),
            Self::Empty => Ok(Vec::new()),
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

    pub fn to_vec(&self) -> WaylandResult<Vec<u8>> {
        let mut request = self.header.to_vec()?;
        request.extend(self.payload.to_vec()?);
        Ok(request)
    }
}
