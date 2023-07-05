use fluvio_connector_common::connector;

#[connector(config)]
#[derive(Debug)]
pub(crate) struct CustomConfig {
    /// Metric namespace that you want to populate.
    pub metric_path: String,
    /// Address of the AMQP server.
    pub amqp_addr: String,
}
