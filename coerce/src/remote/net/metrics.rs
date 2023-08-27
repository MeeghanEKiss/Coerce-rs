pub const METRIC_NETWORK_BYTES_RECV: &str = "coerce_network_bytes_recv";
pub const METRIC_NETWORK_BYTES_SENT: &str = "coerce_network_bytes_sent";

pub const LABEL_SRC_ADDR: &str = "src_addr";

pub struct NetworkMetrics;

impl NetworkMetrics {
    #[inline]
    pub fn incr_bytes_received(len: u64, src_addr: &str) {
        #[cfg(feature = "metrics")]
        increment_counter!(METRIC_NETWORK_BYTES_RECV,
            LABEL_SRC_ADDR => len.to_string()
        );
    }

    #[inline]
    pub fn incr_bytes_sent(len: u64, dest_addr: &str) {
        #[cfg(feature = "metrics")]
        increment_counter!(METRIC_NETWORK_BYTES_SENT,
            LABEL_SRC_ADDR => len.to_string()
        );
    }
}
