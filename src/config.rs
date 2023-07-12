use fluvio_connector_common::connector;

#[connector(config, name = "graphite")]
#[derive(Debug)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct CustomConfig {
    /// Metric namespace that you want to populate.
    pub metric_path: String,
    /// The address of the plaintext TCP interface.
    pub addr: String,
}
