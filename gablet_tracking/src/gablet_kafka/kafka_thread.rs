use std::error::Error;
use gablet_shared_api::kafka::kafka_events::TRACKING_WEB_EVENT;

use crate::{models::tracking::NewWebView, events::tracking::save_web_view};

pub async fn dispatch_kafka_event(key: String, value: String) -> Result<(), Box<dyn Error>> {
    match key.as_str() {
        "test" => {tracing::debug!("Received test value {}", value); Ok(()) },
        TRACKING_WEB_EVENT => forward_track_web_view(value).await ,
        _ => {
            tracing::info!("Unknown kafka event {}", key.as_str());
            Ok(())
        }
    }
}

async fn forward_track_web_view(value: String) -> Result<(), Box<dyn Error>> {
    let view: NewWebView = serde_json::from_str(&value)?;

    save_web_view(&view).await?;

    Ok(())
}