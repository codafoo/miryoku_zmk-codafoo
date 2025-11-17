use anyhow::Result;
use std::sync::Arc;
use tokio::sync::Mutex;

mod protocol;
mod ui;
mod monitor;

use monitor::ZmkMonitor;
use ui::App;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .init();

    // Create monitor
    let monitor = Arc::new(Mutex::new(ZmkMonitor::new()?));

    // Start monitoring in background
    let monitor_clone = monitor.clone();
    tokio::spawn(async move {
        if let Err(e) = monitor_clone.lock().await.start_monitoring().await {
            tracing::error!("Monitoring error: {}", e);
        }
    });

    // Run UI
    let mut app = App::new(monitor);
    app.run().await?;

    Ok(())
}
