use crate::wayland::types::{common::argument::Object, request::RequestMessage};

pub struct SessionLockManager(Object);

impl SessionLockManager {
    pub fn new(id: Object) -> Self {
        Self(id)
    }

    pub fn destroy() -> RequestMessage {
        RequestMessage

    }
}
