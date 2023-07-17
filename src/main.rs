mod config;
mod graphite;

use std::io::Write;
use std::net::TcpStream;

use fluvio_connector_common::consumer::ConsumerStream;
use fluvio_connector_common::{connector, Result};

use self::config::CustomConfig;
use self::graphite::GraphiteMessage;

#[connector(sink)]
async fn start(config: CustomConfig, mut stream: impl ConsumerStream) -> Result<()> {
    let mut graphite_tcp = TcpStream::connect(config.addr)?;

    while let Some(Ok(record)) = stream.next().await {
        let value_bytes = record.value();
        let value_utf8 = std::str::from_utf8(value_bytes)?;
        let message = GraphiteMessage::new(&config.metric_path, value_utf8);

        graphite_tcp.write_all(message.to_string().as_bytes())?;
    }

    Ok(())
}
