mod amqp;
mod config;
mod graphite;

use fluvio_connector_common::{Result, connector};
use fluvio_connector_common::consumer::ConsumerStream;
use fluvio_connector_common::tracing;

use self::amqp::AmqpClient;
use self::config::CustomConfig;
use self::graphite::GraphiteMessage;

#[connector(sink)]
async fn start(config: CustomConfig, mut stream: impl ConsumerStream) -> Result<()> {
    println!("Starting graphite-connector sink connector with {config:?}");

    let amqp_client = AmqpClient::connect(&config.amqp_addr).await?;

    while let Some(Ok(record)) = stream.next().await {
        let value_bytes = record.value();
        let value_utf8 = String::from_utf8_lossy(value_bytes).to_string();
        let message = GraphiteMessage::new(config.metric_path.clone(), value_utf8);

        if let Err(err) = amqp_client.publish(&config.metric_path, &message).await {
            tracing::error!(%err, %message, "Error publishing message to AMQP server");
        } else {
            tracing::debug!(%message, "Published message to AMQP server");
        }
    }

    Ok(())
}
