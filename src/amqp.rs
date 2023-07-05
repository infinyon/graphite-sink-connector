//! Logic related to AMQP connections

use std::ops::Deref;

use lapin::{Connection, ConnectionProperties};

use fluvio_connector_common::{tracing, Result};

pub struct AmqpClient(Connection);

impl AmqpClient {
    pub async fn connect(addr: &str) -> Result<Self> {
        let conn = Connection::connect(addr, ConnectionProperties::default()).await?;

        tracing::info!(%addr, "Connected to AMQP server");

        Ok(Self(conn))
    }
}

impl Deref for AmqpClient {
    type Target = Connection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
