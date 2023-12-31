//! Graphite's resources definitions

use std::fmt::Display;
use std::time::{SystemTime, UNIX_EPOCH};

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
pub struct GraphiteMessage<'a> {
    /// Metric namespace that you want to populate
    metric_path: &'a str,
    /// Value to assign to the metric at this time
    value: &'a str,
    /// is the number of seconds since unix epoch time. Carbon-cache will use the time of arrival if the timestamp is set to -1.
    timestamp: u64,
}

impl<'a> GraphiteMessage<'a> {
    /// Create a new [`GraphiteMessage`] instance with system's current timestamp
    pub fn new(metric_path: &'a str, value: &'a str) -> Self {
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

impl Display for GraphiteMessage<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // <metric path> <metric value> <metric timestamp>
        writeln!(f, "{} {} {}", self.metric_path, self.value, self.timestamp)
    }
}
