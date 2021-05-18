use reqwest::Client;
use std::{io, thread};
use tokio::runtime::Handle;

#[actix_web::main]
async fn main() -> io::Result<()> {
    let _tracer = opentelemetry_zipkin::new_pipeline()
        .with_service_name("tracing_actix_web_opentelemetry_zipkin")
        .with_http_client(Client::new())
        .install_batch(opentelemetry::runtime::Tokio)
        .expect("pipeline install error");

    println!("hello from thread {}", thread::current().name().unwrap());
   
    Handle::current().spawn_blocking(move || {
        opentelemetry::global::shutdown_tracer_provider();
        println!("finished shutdown_tracer_provider on thread {}", thread::current().name().unwrap());
    }).await.unwrap();
    Ok(())
}
