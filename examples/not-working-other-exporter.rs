use std::io;

#[actix_web::main]
async fn main() -> io::Result<()> {
    let instrumentationkey = std::env::var("INSTRUMENTATION_KEY").expect("INSTRUMENTATION_KEY");
    let _tracer = opentelemetry_application_insights::new_pipeline(instrumentationkey)
    .with_client(reqwest::Client::new())
    .install_batch(opentelemetry::runtime::Tokio);

    opentelemetry::global::shutdown_tracer_provider();

    Ok(())
}
