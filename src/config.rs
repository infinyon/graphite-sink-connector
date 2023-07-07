use fluvio_connector_common::connector;

#[connector(config)]
#[derive(Debug)]
pub(crate) struct CustomConfig {
    /// Metric namespace that you want to populate.
    pub metric_path: String,
}
