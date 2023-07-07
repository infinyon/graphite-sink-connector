mod config;
mod graphite;

use fluvio_connector_common::consumer::ConsumerStream;
use fluvio_connector_common::{connector, Result};

use self::config::CustomConfig;
use self::graphite::GraphiteMessage;

#[connector(sink)]
async fn start(config: CustomConfig, mut stream: impl ConsumerStream) -> Result<()> {
    println!("Starting graphite-connector sink connector with {config:?}");

    while let Some(Ok(record)) = stream.next().await {
        let value_bytes = record.value();
        let value_utf8 = String::from_utf8_lossy(value_bytes).to_string();
        let message = GraphiteMessage::new(config.metric_path.clone(), value_utf8);
    }

    Ok(())
}
