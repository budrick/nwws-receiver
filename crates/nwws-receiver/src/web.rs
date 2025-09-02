use crate::message::Message;
use axum::extract::State;
use axum::Error;
use axum::{
    response::sse::{Event, Sse},
    routing::get,
    Router,
};
use axum_extra::TypedHeader;
use futures_util::Stream;
use std::path::PathBuf;
use tokio::time::Duration;
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::StreamExt;
use tower_http::{services::ServeDir, trace::TraceLayer};

use crate::types::SharedCapSender;

pub async fn startcap(
    config: crate::config::Config,
    tx_cap: SharedCapSender,
) -> color_eyre::eyre::Result<()> {
    // build our application
    let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");
    let static_files_service = ServeDir::new(assets_dir).append_index_html_on_directories(true);

    // build application with a route
    let app = Router::new()
        .fallback_service(static_files_service)
        .route("/sse", get(sse_handler))
        .layer(TraceLayer::new_for_http())
        .with_state(tx_cap);

    // run it
    let listener = tokio::net::TcpListener::bind(config.sse.addr)
        .await
        .unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn sse_handler(
    State(sender): State<SharedCapSender>,
    TypedHeader(user_agent): TypedHeader<headers::UserAgent>,
) -> Sse<impl Stream<Item = Result<Event, Error>>> {
    println!("`{}` connected", user_agent.as_str());

    let guard = sender.lock().await;
    let rx_message = (*guard).subscribe();
    let stream = BroadcastStream::new(rx_message);
    let dstream = tokio_stream::iter(vec![Ok(Box::new(Message::Dummy))]);
    let cstream = stream.merge(dstream);

    let stream = cstream.map(move |item| {
        let a = item.unwrap();
        let r = match *a {
            Message::Alert(alert) => return Event::default().json_data(alert),
            _ => Event::default().data("Messages may take a while to arrive..."),
        };
        Ok(r)
    });
    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(10))
            .text("keep-alive-text"),
    )
}
