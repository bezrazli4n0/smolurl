use clap::Parser;
use smolurl::{app, args};
use std::process;
use tracing_subscriber::prelude::*;

#[actix_web::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let args = args::Args::parse();

    tracing::info!("Server started at - {}:{}", args.host, args.port);
    if let Err(error) = app::run(args.host, args.port, args.db_url, args.private_key_path).await {
        eprintln!("Can't start server - {error}");
        process::exit(1);
    }
}
