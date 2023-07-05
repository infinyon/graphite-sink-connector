mod config;

use std::borrow::Cow;
use std::time::{SystemTime, UNIX_EPOCH};

use config::CustomConfig;
use fluvio_connector_common::{connector, consumer::ConsumerStream, Result};

/// Graphite Message
///
/// Graphite understands messages with this format:
///
/// ```ignore
/// metric_path value timestamp\n
/// ```
///
/// A simple example of doing this from the unix terminal would look like this:
///
/// ```bash
/// echo "test.bash.stats 42 `date +%s`" | nc graphite.example.com 2003
/// ```
///
/// # Reference
///
/// You can read more on the Graphite message format [here][1].
///
/// [1]: https://graphite.readthedocs.io/en/latest/feeding-carbon.html#step-3-understanding-the-graphite-message-format
struct GraphiteMessage {
    /// Metric namespace that you want to populate
    metric_path: String,
    /// Value to assign to the metric at this time
    value: String,
    /// is the number of seconds since unix epoch time. Carbon-cache will use the time of arrival if the timestamp is set to -1.
    timestamp: u64,
}

impl GraphiteMessage {
    pub fn new(metric_path: String, value: String) -> Self {
        Self {
            metric_path,
            value,
            timestamp: Self::timestamp(),
        }
    }

    fn timestamp() -> u64 {
        let now = SystemTime::now();
        let duration = now.duration_since(UNIX_EPOCH).expect("Time went backwards");

        duration.as_secs()
    }
}

#[connector(sink)]
async fn start(config: CustomConfig, mut stream: impl ConsumerStream) -> Result<()> {
    println!("Starting graphite-connector sink connector with {config:?}");

    while let Some(Ok(record)) = stream.next().await {
        let value_bytes = record.value();
        let value_utf8 = String::from_utf8_lossy(value_bytes).to_string();
        let message: GraphiteMessage = GraphiteMessage::new(config.metric_path.clone(), value_utf8);

        // println!("{}",record.value().as_ut8_lossy_string());
    }

    Ok(())
}
