use std::{
    sync::{
        Arc,
        atomic::{AtomicI64, Ordering},
    },
    time::Duration,
};

use tokio::time::interval;

use crate::router::http::ws_v2::types::StreamConnection;

pub struct HealthMonitor {
    check_interval: Duration,
    connection_timeout: Duration,
    last_activity: Arc<AtomicI64>,
}

impl HealthMonitor {
    pub fn new(check_interval: Duration, connection_timeout: Duration) -> Self {
        Self {
            check_interval,
            connection_timeout,
            last_activity: Arc::new(AtomicI64::new(chrono::Utc::now().timestamp_micros())),
        }
    }

    pub async fn start_monitoring(&self, mut connection: Box<dyn StreamConnection>) {
        let mut interval = interval(self.check_interval);
        let last_activity = self.last_activity.clone();
        let timeout = self.connection_timeout;

        tokio::spawn(async move {
            loop {
                interval.tick().await;

                let last =
                    chrono::Utc::now().timestamp_micros() - last_activity.load(Ordering::SeqCst);

                if Duration::from_micros(last as u64) > timeout {
                    log::warn!("Connection timeout detected");
                    if let Err(e) = connection.reconnect().await {
                        log::error!("Reconnection failed: {}", e);
                    }
                }
            }
        });
    }

    pub fn update_activity(&self) {
        self.last_activity
            .store(chrono::Utc::now().timestamp_micros(), Ordering::SeqCst);
    }
}
