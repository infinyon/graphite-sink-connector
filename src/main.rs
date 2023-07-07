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
    println!("Starting graphite-connector sink connector with {config:?}");

    let mut graphite_tcp = TcpStream::connect("127.0.0.1:2003")?;

    while let Some(Ok(record)) = stream.next().await {
        let value_bytes = record.value();
        let value_utf8 = String::from_utf8_lossy(value_bytes).to_string();
        let message = GraphiteMessage::new(config.metric_path.clone(), value_utf8);

        graphite_tcp.write(message.to_string().as_bytes())?;
    }

    Ok(())
}
