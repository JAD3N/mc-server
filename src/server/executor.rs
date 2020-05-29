use tokio::time::{Duration, Instant};
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use tokio::sync::Mutex;
use futures::future;
use super::Server;
use crate::world::level::Level;

lazy_static! {
    static ref TICK_RATE: AtomicU32 = AtomicU32::new(20);
    static ref TICK_WARNING_THRESHOLD: AtomicU32 = AtomicU32::new(15_000);
}

pub struct ServerExecutor {
    server: Arc<Mutex<Server>>,
    next_tick_time: Instant,
    last_warning_time: Option<Instant>,
}

impl ServerExecutor {
    pub fn new(server: Arc<Mutex<Server>>) -> Self {
        Self {
            server,
            next_tick_time: Instant::now(),
            last_warning_time: None,
        }
    }

    pub async fn execute(&mut self) -> anyhow::Result<()> {
        let now = Instant::now();
        let delta = now - self.next_tick_time;
        let delta_millis = delta.as_millis() as u32;

        let single_tick = 1000 / TICK_RATE.load(Ordering::Relaxed);
        let single_tick_dur = Duration::from_millis(single_tick as u64);

        if delta_millis > 2000 && {
            if let Some(last_warning_time) = self.last_warning_time {
                (self.next_tick_time - last_warning_time).as_millis() as u32 >= TICK_WARNING_THRESHOLD.load(Ordering::Relaxed)
            } else {
                true
            }
        } {
            let missed_ticks = delta_millis / single_tick;

            warn!("Can't keep up! Is the server overloaded? Running {}ms or {} ticks behind", delta_millis, missed_ticks);

            self.next_tick_time += single_tick_dur * missed_ticks;
            self.last_warning_time = Some(self.next_tick_time.clone());
        }

        self.next_tick_time += single_tick_dur;
        self.tick().await?;

        Ok(())
    }

    pub async fn wait(&self) {
        tokio::time::delay_until(
            self.next_tick_time
        ).await;
    }

    async fn tick(&self) -> anyhow::Result<()> {
        let levels;

        {
            let mut server = self.server.lock().await;

            levels = server.levels.values().cloned()
                .collect::<Vec<Arc<Mutex<Level>>>>();

            server.tick().await?;
        }

        let mut handles = vec![];

        for level in levels.into_iter() {
            handles.push(tokio::spawn(async move {
                let mut level = level.lock().await;

                if let Err(e) = level.tick().await {
                    error!("Failed to tick level '{}': {}", level.name, e);
                }
            }));
        }

        // wait until all handles are done
        future::join_all(handles).await;

        Ok(())
    }
}