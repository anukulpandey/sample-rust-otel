use anyhow::{Context, Result};
use log::Level;
use opentelemetry::global;
use opentelemetry_appender_log::OpenTelemetryLogBridge;

use opentelemetry_otlp::WithExportConfig;
use opentelemetry_resource_detectors::{OsResourceDetector, ProcessResourceDetector};
use opentelemetry_sdk::{
    propagation::TraceContextPropagator,
    resource::{
        EnvResourceDetector, ResourceDetector, SdkProvidedResourceDetector,
        TelemetryResourceDetector,
    },
    runtime, Resource,
};
use tonic::metadata::{MetadataMap, MetadataValue};

use std::{collections::HashMap, time::Duration};
use std::{env, str::FromStr};

// get_resource returns a Resource containing information about the environment
// The Resource is used to provide context to Traces, Metrics and Logs
// It is created by merging the results of multiple ResourceDetectors
// The ResourceDetectors are responsible for detecting information about the environment
fn get_resource() -> Resource {
    let os_resource = OsResourceDetector.detect(Duration::from_secs(0));
    let process_resource = ProcessResourceDetector.detect(Duration::from_secs(0));
    let sdk_resource = SdkProvidedResourceDetector.detect(Duration::from_secs(0));
    let env_resource = EnvResourceDetector::new().detect(Duration::from_secs(0));
    let telemetry_resource = TelemetryResourceDetector.detect(Duration::from_secs(0));

    os_resource
        .merge(&process_resource)
        .merge(&sdk_resource)
        .merge(&env_resource)
        .merge(&telemetry_resource)
}

// A Tracer Provider is a factory for Tracers
// A Tracer creates spans containing more information about what is happening for a given operation,
// such as a request in a service.
fn init_tracer() {
    let signoz_access_token = "<INGESTION_KEY>";
    let mut metadata = MetadataMap::new();
    metadata.insert(
        "signoz-ingestion-key",
        MetadataValue::from_str(&signoz_access_token).unwrap(),
    );
    global::set_text_map_propagator(TraceContextPropagator::new());

    let tracer_provider = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(opentelemetry_otlp::new_exporter().tonic()
        .with_metadata(metadata)
        .with_endpoint("https://ingest.in.signoz.cloud:443"))
        .install_batch(runtime::Tokio)
        .expect("Failed to initialise tracing provider");

    global::set_tracer_provider(tracer_provider);
}

// A Logger Provider is a factory for Loggers
// The init_logger_provider function initialises a Logger Provider
// And sets up a Log Appender for the log crate, bridging logs to the OpenTelemetry Logger.
fn init_logger_provider() {
    let signoz_access_token = "<INGESTION_KEY>";
    let mut metadata = MetadataMap::new();
    metadata.insert(
        "signoz-ingestion-key",
        MetadataValue::from_str(&signoz_access_token).unwrap(),
    );

    // let logger_provider = opentelemetry_otlp::new_pipeline()
    //     .logging()
    //     .with_exporter(
    //         opentelemetry_otlp::new_exporter()
    //         .tonic()
    //         .with_metadata(metadata)
    //         .with_endpoint("https://ingest.in.signoz.cloud:443"))
    //     .with_resource(get_resource())
    //     .install_batch(runtime::Tokio)
    //     .expect("Failed to initialise logger provider");

    let mut headers = HashMap::new();
headers.insert(
    "signoz-ingestion-key".to_string(), // Convert to String
    signoz_access_token.to_string(),    // Convert to String
);

    let logger_provider = opentelemetry_otlp::new_pipeline()
        .logging()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
            .http()
            .with_headers(headers)
            .with_endpoint("https://ingest.in.signoz.cloud:443"))
        .with_resource(get_resource())
        .install_batch(runtime::Tokio)
        .expect("Failed to initialise logger provider");


       
    let otel_log_appender = OpenTelemetryLogBridge::new(&logger_provider);
    log::set_boxed_logger(Box::new(otel_log_appender)).unwrap();

    let max_level = env::var("LOG_LEVEL")
        .ok()
        .and_then(|l| Level::from_str(l.to_lowercase().as_str()).ok())
        .unwrap_or(Level::Info);
    log::set_max_level(max_level.to_level_filter());
}

pub fn init_otel() -> Result<()> {
    // init_logger_provider();
    init_tracer();
    // init_meter_provider().with_context(|| "initialising meter provider")?;
    Ok(())
}
