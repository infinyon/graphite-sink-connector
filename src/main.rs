mod amqp;
mod config;
mod graphite;

use config::CustomConfig;
use fluvio_connector_common::{connector, consumer::ConsumerStream, Result};

use self::graphite::GraphiteMessage;
use amqp::AmqpClient;

#[connector(sink)]
async fn start(config: CustomConfig, mut stream: impl ConsumerStream) -> Result<()> {
    println!("Starting graphite-connector sink connector with {config:?}");

    let amqp_client = AmqpClient::connect(&config.amqp_addr).await?;

    while let Some(Ok(record)) = stream.next().await {
        let value_bytes = record.value();
        let value_utf8 = String::from_utf8_lossy(value_bytes).to_string();
        let message = GraphiteMessage::new(config.metric_path.clone(), value_utf8);

        // println!("{}",record.value().as_ut8_lossy_string());
    }

    Ok(())
}
