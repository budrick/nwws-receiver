use axum::extract::State;
use axum::Error;
use axum::{
    response::sse::{Event, Sse},
    routing::get,
    Router,
};
use axum_extra::TypedHeader;
use futures_util::Stream;
use tokio::time::Duration;
use tokio_stream::StreamExt;
// use futures_util::stream::{self, Stream};
use std::{path::PathBuf, sync::Arc};
use tokio::sync::Mutex;
use tokio_stream::wrappers::BroadcastStream;
use tower_http::{services::ServeDir, trace::TraceLayer};

use crate::types::SharedCapSender;

pub async fn startcap(
    tx_cap: Arc<Mutex<tokio::sync::broadcast::Sender<oasiscap::v1dot2::Alert>>>,
) -> color_eyre::eyre::Result<()> {
    // build our application
    let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");
    let static_files_service = ServeDir::new(assets_dir).append_index_html_on_directories(true);

    // builcapur application with a route
    let app = Router::new()
        .fallback_service(static_files_service)
        .route("/sse", get(sse_handler))
        .layer(TraceLayer::new_for_http())
        .with_state(tx_cap);

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:13579")
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

    let stream = stream.map(move |item| {
        // println!("{:?}", item);
        Event::default().json_data(item.unwrap())
    });
    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(10))
            .text("keep-alive-text"),
    )
}
