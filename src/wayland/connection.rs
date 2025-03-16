use crate::wayland::types::{
    common::argument::{NewId, Object},
    core::{display::WlDisplay, registry::WlRegistry},
    event::EventMessage,
};
use mio::{Events, Interest, Poll, Token, net::UnixStream};
use std::{
    collections::HashMap,
    env,
    io::{Read, Result, Write},
    path::PathBuf,
};

const WAYLAND_SOCKET: Token = Token(0);

pub struct Wayland {
    stream: UnixStream,
    display: WlDisplay,
    registry: Option<WlRegistry>,
    poll: Poll,
    next_id: u32,
    interface_map: HashMap<u32, (String, u32)>,
}

impl Wayland {
    pub fn connect() -> Result<Self> {
        let runtime_dir = env::var("XDG_RUNTIME_DIR").expect("XDG_RUNTIME_DIR not set");
        let display_name = env::var("WAYLAND_DISPLAY").expect("WAYLAND_DISPLAY not set");
        let socket_path = PathBuf::from(runtime_dir).join(display_name);
        println!("Connecting to wayland socket at path: {:?}", socket_path);

        let poll = Poll::new()?;
        let mut stream = UnixStream::connect(&socket_path)?;

        poll.registry().register(
            &mut stream,
            WAYLAND_SOCKET,
            Interest::READABLE | Interest::WRITABLE,
        )?;

        Ok(Self {
            stream,
            display: WlDisplay::new(Object::new(1)),
            registry: None,
            poll,
            next_id: 2,
            interface_map: HashMap::new(),
        })
    }

    pub fn poll_events(&mut self) -> Result<()> {
        let mut events = Events::with_capacity(128);
        self.poll.poll(&mut events, None)?;
        for event in events.iter() {
            if event.token() == WAYLAND_SOCKET {
                if event.is_readable() {
                    loop {
                        match self.handle_readable()? {
                            true => continue,
                            false => break,
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn handle_readable(&mut self) -> Result<bool> {
        match self.read_messages()? {
            Some(messages) => {
                for message in messages {
                    if message.is_global() {
                        let interface = message
                            .payload
                            .get_global()
                            .expect("Global event should have interface");
                        self.interface_map
                            .insert(interface.0, (interface.1, interface.2));
                    } else {
                        if message.is_callback_done() {
                            self.bind(7);
                        }
                    }
                }
                Ok(true)
            }
            None => Ok(false),
        }
    }

    pub fn read_messages(&mut self) -> Result<Option<Vec<EventMessage>>> {
        let mut buf = [0u8; 8192];
        match self.stream.read(&mut buf) {
            Ok(0) => Ok(None),
            Ok(n) => {
                tracing::trace!("Read {} bytes from socket", n);
                let messages = EventMessage::parse_messages(&buf[..n]);
                if messages.is_empty() {
                    Ok(None)
                } else {
                    Ok(Some(messages))
                }
            }
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => Ok(None),
            Err(e) => Err(e),
        }
    }

    pub fn bind(&mut self, name: u32) -> Result<()> {
        let id = self.next_id;
        self.next_id += 1;

        if let Some((interface, version)) = self.interface_map.get(&name) {
            tracing::info!("Binding interface {} version {}", interface, version);
            let request = self
                .registry
                .expect("WlRegistry not setup yet, called get_registry first?")
                .bind(
                    name,
                    NewId::new((interface.to_string(), *version), Object::new(id)),
                );
            let request_bytes = request.to_vec().unwrap();
            tracing::info!("Sending: {:?}", request);
            tracing::info!("Sending: {:?}, len: {}", request_bytes, request_bytes.len());
            self.stream.write_all(&request_bytes)?;
            self.stream.flush()?;
            tracing::trace!("Sent bind request");
            Ok(())
        } else {
            tracing::error!("Interface with name {} not supported by compositor", name);
            Ok(())
        }
    }

    pub fn sync(&mut self) -> Result<()> {
        let id = self.next_id;
        self.next_id += 1;

        self.stream
            .write_all(&self.display.sync(id).to_vec().unwrap())?;
        self.stream.flush()?;
        tracing::trace!("Sent sync request");
        Ok(())
    }

    pub fn get_registry(&mut self) -> Result<()> {
        let id = self.next_id;
        self.next_id += 1;

        self.stream
            .write_all(&self.display.get_registry(id).to_vec().unwrap())?;
        self.stream.flush()?;
        self.registry = Some(WlRegistry::new(Object::new(id)));
        tracing::trace!("Created: {:?}", self.registry);
        Ok(())
    }
}
