use reqwest::Client;
use std::io;

#[actix_web::main]
async fn main() -> io::Result<()> {
    let _tracer = opentelemetry_zipkin::new_pipeline()
        .with_service_name("tracing_actix_web_opentelemetry_zipkin")
        .with_http_client(Client::new())
        .install_batch(opentelemetry::runtime::Tokio)
        .expect("pipeline install error");

    opentelemetry::global::shutdown_tracer_provider();

    Ok(())
}
