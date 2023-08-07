use std::{
    error::Error,
    fmt::Display,
    future::Future,
    sync::{
        atomic::{AtomicI32, Ordering},
        Arc,
    },
    thread::sleep,
    time::Duration,
};

use dashmap::DashMap;
use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};

use crate::{cancellation_token::CancellationToken, credentials::Credentials};

#[derive(Debug)]
pub struct KafkaPollError {
    pub started: bool,
    pub inner: Option<Box<dyn Error>>,
}

impl Display for KafkaPollError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error during kafka polling.")?;

        if let Some(err) = &self.inner {
            write!(f, " Inner error: {}", err)
        } else {
            Ok(())
        }
    }
}

impl Error for KafkaPollError {}

const MAX_FAILS: i32 = 3;

pub fn kafka_thread<Fut>(
    cancellation_token: CancellationToken,
    handle_event: fn(String, String) -> Fut,
) where
    Fut: Future<Output = Result<(), Box<dyn Error>>> + Send + 'static,
{
    loop {
        if cancellation_token.is_cancellation_requested() {
            tracing::info!("Kafka thread cancelled");
        }
        tracing::info!("Kafka thread starting...");
        let result = kafka_worker(cancellation_token.clone(), handle_event);
        match result {
            Err(err) => {
                tracing::error!("Kafka worker thread failed: {}\n\nRestarting...", err);
            }
            Ok(stopped) => {
                if stopped {
                    return;
                }
            }
        }
    }
}

fn kafka_worker<Fut>(cancellation_token: CancellationToken, handle_event: fn(String, String) -> Fut) -> Result<bool, KafkaPollError>
where
    Fut: Future<Output = Result<(), Box<dyn Error>>> + Send + 'static,
{
    let fails: Arc<DashMap<String, AtomicI32>> = Arc::new(DashMap::new());

    let pool = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .map_err(|err| KafkaPollError {
            started: false,
            inner: Some(Box::new(err)),
        })?;

    let creds = Credentials::new("./config/credentials.toml")
        .map_err(|err| KafkaPollError {
            started: false,
            inner: Some(Box::new(err)),
        })?
        .kafka
        .ok_or_else(|| KafkaPollError {
            started: false,
            inner: None,
        })?;

    let mut consumer_builder = Consumer::from_hosts(creds.hosts)
        .with_group("playground".into())
        .with_fallback_offset(FetchOffset::Earliest)
        .with_offset_storage(GroupOffsetStorage::Kafka);

    for topic in creds.topics {
        consumer_builder = consumer_builder.with_topic(topic);
    }

    let mut con = consumer_builder.create().map_err(|err| KafkaPollError {
        started: false,
        inner: Some(Box::new(err)),
    })?;

    loop {
        if cancellation_token.is_cancellation_requested() {
            return Ok(false);
        }

        let mss = con.poll().map_err(|err| KafkaPollError {
            started: true,
            inner: Some(Box::new(err)),
        })?;

        if mss.is_empty() {
            sleep(Duration::from_secs(if cfg!(debug_assertions) {
                1
            } else {
                10
            }));
            continue;
        }

        for ms in mss.iter() {
            for m in ms.messages() {
                let key = std::str::from_utf8(m.key)
                    .map_err(|err| KafkaPollError {
                        started: true,
                        inner: Some(Box::new(err)),
                    })?
                    .to_owned();

                let value = std::str::from_utf8(m.value)
                    .map_err(|err| KafkaPollError {
                        started: true,
                        inner: Some(Box::new(err)),
                    })?
                    .to_owned();

                let send_fails = fails.clone();

                match key.as_str() {
                    "CLEAR_KAFKA_FAILS" => {
                        tracing::info!("Clearing kafka fails");
                        fails.clear()
                    }
                    "STOP_KAFKA_THREAD" => {
                        tracing::info!("Received kafka event to stop the kafka thread");
                        return Ok(true)
                    },
                    _ => {
                        tracing::trace!(
                            "Executing kafka event from topic {} with key {} and value {}",
                            ms.topic(),
                            key,
                            value
                        );
                        pool.spawn(dispatch_kafka_event(key, value, send_fails, handle_event));
                    }
                }
            }
            con.consume_messageset(ms).map_err(|err| KafkaPollError {
                started: true,
                inner: Some(Box::new(err)),
            })?;
        }
        con.commit_consumed().map_err(|err| KafkaPollError {
            started: true,
            inner: Some(Box::new(err)),
        })?;
    }
}

async fn dispatch_kafka_event<Fut>(
    key: String,
    value: String,
    fails: Arc<DashMap<String, AtomicI32>>,
    handle_event: fn(String, String) -> Fut,
) where
    Fut: Future<Output = Result<(), Box<dyn Error>>>,
{
    let total_fails = fails
        .get(&key)
        .or_else(|| {
            fails.insert(key.clone(), AtomicI32::new(0));
            fails.get(&key)
        })
        .and_then(|fail_count| Some(fail_count.load(Ordering::Relaxed)));

    if let Some(total_fails) = total_fails {
        if total_fails >= MAX_FAILS {
            return;
        }
    }

    let result = handle_event(key.clone(), value).await;

    if result.is_err() {
        tracing::error!("Failed to complete kafka event {}", key);

        fails.get(&key).and_then(|fail_count| {
            fail_count.fetch_add(1, Ordering::Relaxed);
            Some(())
        });
    }
}
