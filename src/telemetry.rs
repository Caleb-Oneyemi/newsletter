use tracing::{subscriber::set_global_default, Subscriber};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

pub fn get_tracing_subscriber(name: String, log_level: String) -> impl Subscriber + Send + Sync {
    let env_filter_layer = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new(log_level));
    let formatting_layer = BunyanFormattingLayer::new(name, std::io::stdout);

    Registry::default()
        .with(env_filter_layer)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

pub fn init_tracing_subscriber(subscriber: impl Subscriber + Send + Sync) {
    // redirect all events from logs to tracing subscriber
    LogTracer::init().expect("failed to set up logger");

    // register subscriber for processing spans.
    set_global_default(subscriber).expect("failed to setup log subscriber");
}
