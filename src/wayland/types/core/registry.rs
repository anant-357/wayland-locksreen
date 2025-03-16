use crate::wayland::types::{
    common::argument::{NewId, Object},
    request::{Message, RequestMessage},
};

#[derive(Debug, Clone, Copy)]
pub struct WlRegistry(Object);

impl WlRegistry {
    pub fn new(id: Object) -> Self {
        Self(id)
    }
    pub fn bind(&self, name: u32, id: NewId) -> RequestMessage {
        RequestMessage::build(
            self.0,
            0,
            16 + id.len() as u16,
            Message::Bind {
                name: Object::new(name),
                id,
            },
        )
    }
}
