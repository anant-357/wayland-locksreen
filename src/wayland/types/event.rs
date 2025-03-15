use crate::wayland::types::common::{
    argument::Argument,
    header::Header,
    parse_utils::{ParseError, ParseResult},
};

use std::io::Cursor;

#[derive(Debug)]
pub enum Event {
    DeleteId { id: u32 },
    Interface { id: u32, name: String, version: u32 },
    Callback { callback_data: u32 },
    Other,
}

impl Event {
    fn parse(header: Header, payload: &mut Cursor<&[u8]>) -> ParseResult<Self> {
        match header.object_id {
            1 => match header.opcode {
                1 => {
                    let id = Argument::parse_wayland_u32(payload)?;
                    let event = Self::DeleteId { id };
                    Ok(event)
                }
                _ => {
                    tracing::warn!(
                        "Unhandleable event!: id {}, opcode {}",
                        header.object_id,
                        header.opcode
                    );
                    Ok(Self::Other)
                }
            },
            2 => match header.opcode {
                0 => {
                    if payload.get_ref().len() < 8 {
                        Err(ParseError::UnexpectedEndOfBuffer)
                    } else {
                        let id = Argument::parse_wayland_u32(payload)?;
                        let name = Argument::parse_wayland_string(payload)?
                            .expect("Failed to parse wayland string");
                        let version = Argument::parse_wayland_u32(payload)?;
                        Ok(Event::Interface { id, name, version })
                    }
                }
                _ => {
                    tracing::warn!(
                        "Unhandleable event!: id {}, opcode {}",
                        header.object_id,
                        header.opcode
                    );
                    Ok(Self::Other)
                }
            },
            3 => match header.opcode {
                0 => {
                    let callback_data = Argument::parse_wayland_u32(payload)?;
                    let event = Self::Callback { callback_data };
                    Ok(event)
                }
                _ => {
                    tracing::warn!(
                        "Unhandleable event!: id {}, opcode {}",
                        header.object_id,
                        header.opcode
                    );
                    Ok(Self::Other)
                }
            },
            _ => {
                tracing::warn!(
                    "Unhandleable event!: id {}, opcode {}",
                    header.object_id,
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

    fn parse(buffer: &mut Cursor<&[u8]>) -> ParseResult<Self> {
        let object_id = Argument::parse_wayland_u32(buffer)?;
        let opcode = Argument::parse_wayland_u16(buffer)?;
        let message_size = Argument::parse_wayland_u16(buffer)?;
        let header = Header::new(object_id, opcode, message_size);
        if message_size > buffer.get_ref().len() as u16 {
            Err(ParseError::UnexpectedEndOfBuffer)
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
