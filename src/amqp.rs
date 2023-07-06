//! Logic related to AMQP connections

use lapin::options::{BasicPublishOptions, QueueDeclareOptions};
use lapin::publisher_confirm::PublisherConfirm;
use lapin::types::FieldTable;
use lapin::{BasicProperties, Channel, Connection, ConnectionProperties, Queue};

use fluvio_connector_common::{tracing, Result};

use crate::graphite::GraphiteMessage;

const GRAPHITE_EXCHANGE: &str = "amq.direct";
const GRAPHITE_ROUTING_KEY: &str = "graphite";
const GRAPHITE_QUEUE: &str = "graphite_connector_data";

pub struct AmqpClient {
    conn: Connection,
    channel: Channel,
    queue: Queue,
}

impl AmqpClient {
    pub async fn connect(addr: &str) -> Result<Self> {
        let conn = Connection::connect(addr, ConnectionProperties::default()).await?;
        let channel = conn.create_channel().await?;
        let queue = channel
            .queue_declare(
                GRAPHITE_QUEUE,
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await?;

        channel
            .queue_bind(
                GRAPHITE_QUEUE,
                GRAPHITE_EXCHANGE,
                GRAPHITE_ROUTING_KEY,
                lapin::options::QueueBindOptions::default(),
                FieldTable::default(),
            )
            .await
            .expect("Error binding queue to exchange");

        tracing::info!(%addr, "Connected to AMQP server");

        Ok(Self {
            conn,
            channel,
            queue,
        })
    }

    pub async fn publish(
        &self,
        metric_path: &str,
        message: &GraphiteMessage,
    ) -> std::result::Result<PublisherConfirm, lapin::Error> {
        self.channel
            .basic_publish(
                GRAPHITE_EXCHANGE,
                metric_path,
                BasicPublishOptions::default(),
                message.to_string().as_bytes(),
                BasicProperties::default(),
            )
            .await
    }
}
