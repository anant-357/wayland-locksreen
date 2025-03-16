use crate::wayland::types::{common::argument::Object, request::RequestMessage};


#[derive(Debug, Clone, Copy)]
pub struct SessionLockManager(Object);

impl SessionLockManager {
    pub fn new(id: Object) -> Self {
        Self(id)
    }

    pub fn destroy() -> RequestMessage {

    }
}
