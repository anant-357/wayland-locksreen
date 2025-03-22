mod wayland;
use std::io::Result;
use tracing_subscriber::FmtSubscriber;

use crate::wayland::connection::Wayland;

fn setup_logs(level: tracing::Level) {
    let subscriber = FmtSubscriber::builder()
        .without_time()
        .with_max_level(level)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set logger");
}

fn main() -> Result<()> {
    setup_logs(tracing::Level::INFO);
    let mut conn = Wayland::connect()?;
    tracing::info!("Wayland Connection Established");
    conn.setup()?;
    tracing::info!("Received all globals!");
    conn.bind("ext_session_lock_manager_v1".to_string())?;
    conn.lock()?;
    std::thread::sleep(std::time::Duration::from_secs(5));
    conn.unlock()?;

    loop {
        conn.poll_events()?;
    }
}
