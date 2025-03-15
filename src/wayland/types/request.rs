use crate::wayland::types::common::header::Header;

enum Message {
    Callback { callback_id: u32 },
}

impl Message {
    fn to_vec(&self) -> Vec<u8> {
        match self {
            Self::Callback { callback_id } => {
                let mut v = Vec::new();
                v.extend_from_slice(&callback_id.to_le_bytes());
                v
            }
        }
    }
}

pub struct RequestMessage {
    header: Header,
    payload: Message,
}

impl RequestMessage {
    fn build(object_id: u32, opcode: u16, size: u16, message: Message) -> Self {
        let header = Header {
            object_id,
            opcode,
            size,
        };

        Self {
            header,
            payload: message,
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let header = self.header.to_vec();
        let payload = self.payload.to_vec();

        let mut request = Vec::new();
        request.extend_from_slice(&header);
        request.extend_from_slice(&payload);
        request
    }

    pub fn sync(callback_id: u32) -> Self {
        RequestMessage::build(1, 0, 12, Message::Callback { callback_id })
    }

    pub fn get_registry(callback_id: u32) -> Self {
        RequestMessage::build(1, 1, 12, Message::Callback { callback_id })
    }
}
