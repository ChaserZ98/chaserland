use opentelemetry::{
    global,
    metrics::{Counter, Histogram, Meter},
};
use tower_http::trace::{MakeSpan, OnFailure, OnRequest, OnResponse};

#[derive(Debug, Clone)]
pub struct Metrics {
    pub total_requests_counter: Counter<u64>,
    pub total_errors_counter: Counter<u64>,
    pub processing_time_histogram: Histogram<u64>,
}

impl Metrics {
    pub fn new() -> Self {
        let meter = global::meter("chaserland-articles-service");
        let total_requests_counter = Self::build_total_requests_counter(&meter);
        let total_errors_counter = Self::build_total_errors_counter(&meter);
        let processing_time_histogram = Self::build_processing_time_histogram(&meter);

        Self {
            total_requests_counter,
            total_errors_counter,
            processing_time_histogram,
        }
    }

    fn build_total_requests_counter(meter: &Meter) -> Counter<u64> {
        meter
            .u64_counter("total_requests")
            .with_description("Total number of requests")
            .with_unit("requests")
            .build()
    }

    fn build_total_errors_counter(meter: &Meter) -> Counter<u64> {
        meter
            .u64_counter("total_errors")
            .with_description("Total number of errors")
            .with_unit("errors")
            .build()
    }

    fn build_processing_time_histogram(meter: &Meter) -> Histogram<u64> {
        meter
            .u64_histogram("processing_time")
            .with_description("Processing time of each successful request in milliseconds")
            .with_unit("ms")
            .build()
    }
}

#[derive(Clone)]
pub struct MetricHandler(Metrics);

impl MetricHandler {
    pub fn new(metrics: Metrics) -> Self {
        Self(metrics)
    }

    pub fn inner(&self) -> &Metrics {
        &self.0
    }
}

impl<B> MakeSpan<B> for MetricHandler {
    fn make_span(&mut self, _: &http::Request<B>) -> tracing::Span {
        tracing::trace_span!("metric")
    }
}

impl<B> OnRequest<B> for MetricHandler {
    fn on_request(&mut self, _: &http::Request<B>, _: &tracing::Span) {
        self.inner().total_requests_counter.add(1, &[]);
    }
}

impl<B> OnResponse<B> for MetricHandler {
    fn on_response(self, _: &http::Response<B>, latency: std::time::Duration, _: &tracing::Span) {
        self.inner()
            .processing_time_histogram
            .record(latency.as_millis() as u64, &[]);
    }
}

impl<B> OnFailure<B> for MetricHandler {
    fn on_failure(&mut self, _: B, _: std::time::Duration, _: &tracing::Span) {
        self.inner().total_errors_counter.add(1, &[]);
    }
}
