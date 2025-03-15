mod wayland;
use std::io::Result;
use tracing_subscriber::FmtSubscriber;

use crate::wayland::connection::Wayland;

fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder().without_time().with_max_level(tracing::Level::TRACE).finish();
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set logger");
    let mut conn = Wayland::connect()?;
    tracing::info!("Wayland Connection Established");
    conn.get_registry()?;
    conn.sync()?;
    
    loop {
        conn.poll_events()?;
    }
}
