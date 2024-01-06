extern crate dotenv;
mod html_templates;
use html_templates::{generate_index_page, generate_random_page};
use opentelemetry::global::shutdown_tracer_provider;
use opentelemetry::sdk::Resource;
use opentelemetry::trace::TraceError;
use opentelemetry::{
    global, sdk::trace as sdktrace,
    trace::{TraceContextExt, Tracer},
    Context, Key, KeyValue,
};
use opentelemetry_otlp::WithExportConfig;
use rand::Rng;
use std::error::Error;
use tonic::metadata::{MetadataMap, MetadataValue};
use std::net::SocketAddr;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use dotenv::dotenv; 
use std::env;

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let tracer = global::tracer("global_tracer");
    let _cx = Context::new();
    let user_agent = extract_user_agent(&req);

    match req.uri().path() {
        "/" => { 
            let html_response = generate_index_page();

            tracer.in_span("operation", |cx| {
                let span = cx.span();
                span.set_attribute(Key::new("User Agent").string(user_agent.clone()));

                span.add_event(
                    format!("Operations"),
                    vec![
                        Key::new("Visited Route").string("/"),
                    ],
                );
            });

            Ok(Response::new(Body::from(html_response)))
        }
        "/generate_random" => {
            let random_number = generate_random_number();
            tracer.in_span("operation", |cx| {
                let span = cx.span();
                span.set_attribute(Key::new("User Agent").string(user_agent.clone()));
                
                span.add_event(
                    format!("Operations"),
                    vec![
                        Key::new("Visited Route").string("/generate_random"),
                        Key::new("Random No.").string(random_number.to_string()),
                    ],
                );
                tracer.in_span("Sub operation", |cx| {
                    let span = cx.span();
                    span.set_attribute(Key::new("Random No.").string(random_number.to_string()));
                    span.add_event("Sub span event", vec![]);
                });
            });
            let random_num_response = generate_random_page(random_number);
            Ok(Response::new(Body::from(random_num_response)))
        }
        _ => Ok(Response::builder()
            .status(404)
            .body(Body::from("Not Found"))
            .unwrap()),
    }
}

fn generate_random_number() -> i64 {
    rand::thread_rng().gen_range(1..100)
}

// if you want to pass env variables via command
// fn init_tracer() -> Result<sdktrace::Tracer, TraceError> {
//     opentelemetry_otlp::new_pipeline()
//         .tracing()
//         .with_exporter(opentelemetry_otlp::new_exporter().tonic().with_env())
//         .with_trace_config(
//             sdktrace::config().with_resource(Resource::default()),
//         )
//         .install_batch(opentelemetry::runtime::Tokio)
// }

// if you want to pass env variables via .env file
fn init_tracer() -> Result<sdktrace::Tracer, TraceError> {
    let signoz_access_token = std::env::var("SIGNOZ_ACCESS_TOKEN").expect("SIGNOZ_ACCESS_TOKEN not set");
    let mut metadata = MetadataMap::new();
    metadata.insert(
        "signoz-access-token",
        MetadataValue::from_str(&signoz_access_token).unwrap(),
    );
    opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_metadata(metadata)
                .with_endpoint(std::env::var("SIGNOZ_ENDPOINT").expect("SIGNOZ_ENDPOINT not set")),
        )
        .with_trace_config(
            sdktrace::config().with_resource(Resource::new(vec![
                KeyValue::new(
                    opentelemetry_semantic_conventions::resource::SERVICE_NAME,
                    std::env::var("APP_NAME").expect("APP_NAME not set"),
                ),
            ])),
        )
        .install_batch(opentelemetry::runtime::Tokio)
}

fn extract_user_agent(req: &Request<Body>) -> String {
    req.headers()
        .get("user-agent")
        .and_then(|value| value.to_str().ok())
        .map_or_else(|| String::from("unknown"), |ua| ua.to_string())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    dotenv().ok(); //comment this if you want to pass env vars via command
    let _ = init_tracer()?;

    let port = std::env::var("PORT").expect("PORT not set").parse().expect("PORT must be an Integer");
    let addr: SocketAddr = ([127, 0, 0, 1], port).into();

    let make_svc = make_service_fn(|_conn| {
        async { Ok::<_, Infallible>(service_fn(handle_request)) }
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Server running on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }

    shutdown_tracer_provider();
    Ok(())
}
