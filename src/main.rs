mod config;

use config::CustomConfig;
use fluvio_connector_common::{connector, consumer::ConsumerStream, Result};

#[connector(sink)]
async fn start(config: CustomConfig, mut stream: impl ConsumerStream) -> Result<()> {
    println!("Starting graphite-connector sink connector with {config:?}");
    while let Some(Ok(record)) = stream.next().await {
        println!("{}",record.value().as_ut8_lossy_string());
    }
    Ok(())
}
