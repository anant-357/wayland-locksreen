use crate::wayland::types::{
    common::argument::Object,
    request::{Message, RequestMessage},
};

#[derive(Debug, Clone, Copy)]
pub struct SessionLockManager(Object);

impl SessionLockManager {
    pub fn new(id: Object) -> Self {
        Self(id)
    }

    pub fn destroy(&self) -> RequestMessage {
        RequestMessage::build(self.0, 0, 8, Message::Empty)
    }

    pub fn lock(&self, ext_session_lock_v1: u32) -> RequestMessage {
        RequestMessage::build(
            self.0,
            1,
            12,
            Message::Lock {
                ext_session_lock_v1: Object::new(ext_session_lock_v1),
            },
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SessionLock(Object);

impl SessionLock {
    pub fn new(id: Object) -> Self {
        Self(id)
    }

    pub fn destroy(&self) -> RequestMessage {
        RequestMessage::build(self.0, 0, 8, Message::Empty)
    }

    pub fn get_lock_surface(&self) {}

    pub fn unlock_and_destroy(&self) -> RequestMessage {
        RequestMessage::build(self.0, 2, 8, Message::Empty)
    }
}
