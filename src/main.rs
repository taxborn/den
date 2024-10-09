mod router;

use clap::{Parser, Subcommand};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use std::net::SocketAddr;
use std::path::PathBuf;
use tokio::net::TcpListener;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The port to run the server on
    #[arg(short, long, default_value_t = 3000)]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let args = Cli::parse();
    let addr = SocketAddr::from(([0, 0, 0, 0], args.port));
    let listener = TcpListener::bind(addr).await?;

    println!("Listening on http://{addr}");

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(router::route))
                .await
            {
                eprintln!("Error serving connection: {err:?}");
            }
        });
    }
}
