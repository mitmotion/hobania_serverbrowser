use prometheus::{register_histogram_vec, register_int_counter_vec, HistogramVec, IntCounterVec};

/// Prometheus Metrics to be exposed on /metrics
#[derive(Clone)]
pub struct Metrics {
    pub request: IntCounterVec,
    pub request_duration: HistogramVec,
}
impl Metrics {
    pub fn new() -> Self {
        let request_duration = register_histogram_vec!(
            "serverbrowser_duration_seconds",
            "The duration of request to complete in seconds",
            &["route"],
            vec![0.001, 0.01, 0.05, 0.1, 0.5, 15.]
        )
        .unwrap();

        Metrics {
            request: register_int_counter_vec!("request_total", "reconciliations", &["route"])
                .unwrap(),
            request_duration,
        }
    }
}
