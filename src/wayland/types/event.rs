use crate::wayland::types::common::{
    argument::{Argument, Object},
    header::Header,
    parse_utils::{Error, WaylandResult},
};

use std::io::Cursor;

#[derive(Debug)]
pub enum Event {
    Error {
        object_id: u32,
        code: u32,
        message: String,
    },
    DeleteId {
        id: u32,
    },
    Interface {
        id: u32,
        name: String,
        version: u32,
    },
    Callback {
        callback_data: u32,
    },
    Other,
}

impl Event {
    fn parse(header: Header, payload: &mut Cursor<&[u8]>) -> WaylandResult<Self> {
        match header.object_id.inner() {
            1 => match header.opcode {
                0 => {
                    let object_id = u32::decode(payload)?;
                    let code = u32::decode(payload)?;
                    let message = String::decode(payload)?;
                    Ok(Self::Error {
                        object_id,
                        code,
                        message,
                    })
                }
                1 => {
                    let id = u32::decode(payload)?;
                    Ok(Self::DeleteId { id })
                }
                _ => {
                    tracing::warn!(
                        "Unhandleable event!: id {}, opcode {}",
                        header.object_id.inner(),
                        header.opcode
                    );
                    Ok(Self::Other)
                }
            },
            2 => match header.opcode {
                0 => {
                    if payload.get_ref().len() < 8 {
                        Err(Error::UnexpectedEndOfBuffer)
                    } else {
                        let id = u32::decode(payload)?;
                        let name = String::decode(payload)?;
                        let version = u32::decode(payload)?;
                        Ok(Event::Interface { id, name, version })
                    }
                }
                _ => {
                    tracing::warn!(
                        "Unhandleable event!: id {}, opcode {}",
                        header.object_id.inner(),
                        header.opcode
                    );
                    Ok(Self::Other)
                }
            },
            3 => match header.opcode {
                0 => {
                    let callback_data = u32::decode(payload)?;
                    let event = Self::Callback { callback_data };
                    Ok(event)
                }
                _ => {
                    tracing::warn!(
                        "Unhandleable event!: id {}, opcode {}",
                        header.object_id.inner(),
                        header.opcode
                    );
                    Ok(Self::Other)
                }
            },
            _ => {
                tracing::warn!(
                    "Unhandleable event!: id {}, opcode {}",
                    header.object_id.inner(),
                    header.opcode
                );
                Ok(Self::Other)
            }
        }
    }

    pub fn get_global(&self) -> Option<(u32, String, u32)> {
        match self {
            Self::Interface { id, name, version } => Some((*id, name.to_string(), *version)),
            _ => None,
        }
    }
}

pub struct EventMessage {
    header: Header,
    pub payload: Event,
}

impl EventMessage {
    pub fn parse_messages(buffer: &[u8]) -> Vec<Self> {
        let mut messages = Vec::new();
        let mut buf = Cursor::new(buffer);
        let mut offset = 0;

        while offset + 8 < buf.get_ref().len() {
            let message = Self::parse(&mut buf).unwrap();
            offset += message.header.size as usize;
            messages.push(message);
        }

        messages
    }

    fn parse(buffer: &mut Cursor<&[u8]>) -> WaylandResult<Self> {
        let object_id = Object::decode(buffer)?;
        let opcode = u16::decode(buffer)?;
        let message_size = u16::decode(buffer)?;
        let header = Header::new(object_id, opcode, message_size);
        if message_size > buffer.get_ref().len() as u16 {
            Err(Error::UnexpectedEndOfBuffer)
        } else {
            let payload = Event::parse(header.clone(), buffer)?;
            tracing::info!("{:?}", payload);
            Ok(Self { header, payload })
        }
    }

    pub fn is_global(&self) -> bool {
        match self.payload {
            Event::Interface {
                id: _,
                name: _,
                version: _,
            } => true,
            _ => false,
        }
    }

    pub fn is_callback_done(&self) -> bool {
        match self.payload {
            Event::Callback { callback_data: _ } => true,
            _ => false,
        }
    }
}
