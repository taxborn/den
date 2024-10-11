use axum::{
    body::Bytes,
    extract::MatchedPath,
    http::{HeaderMap, Request},
    response::{Html, Response},
    routing::get,
    Router,
};
use clap::Parser;
use std::time::Duration;
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tracing::{info_span, Span};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The port to run the server on
    #[arg(short, long, default_value_t = 3000)]
    port: u16,
}

fn app() -> Router {
    Router::new()
        .route("/", get(index))
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                info_span!("http_request", method = ?request.method(), uri = tracing::field::Empty)
            })
            .on_request(|_request: &Request<_>, _span: &Span| {
                _span.record("uri", _request.uri().to_string());
                tracing::info!("request happened!!");
            })
            .on_response(|_response: &Response, _latency: Duration, _span: &Span| {
                // ...
                tracing::info!("response happened!!");
            })
            .on_body_chunk(|_chunk: &Bytes, _latency: Duration, _span: &Span| {
                // ...
            })
            .on_eos(
                |_trailers: Option<&HeaderMap>, _stream_duration: Duration, _span: &Span| {
                    // ...
                },
            )
            .on_failure(
                |_error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                    tracing::info!("idk bro {:?}", _error);
                },
            )
        )
}

#[tokio::main]
async fn main() {
    // Configure tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!(
                    "{}=debug,tower_http=debug,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Get cli arguments
    let args = Cli::parse();
    let addr = format!("0.0.0.0:{}", args.port);

    // Create a listener
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    // TODO: Should we use nginx as a reverse proxy and have SSL termination there?
    //       If so, then we just keep HTTP here, might make the app faster.
    // println!("Listening on http://{}", listener.local_addr().unwrap());
    tracing::debug!("Listening on http://{}", listener.local_addr().unwrap());

    // Serve the application
    axum::serve(listener, app()).await.unwrap();
}

async fn index() -> Html<&'static str> {
    tracing::info!("loaded template index");
    Html(std::include_str!("../site/pages/index.html"))
}
