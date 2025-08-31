use axum::extract::State;
use axum::Error;
use axum::{
    response::sse::{Event, Sse},
    routing::get,
    Router,
};
use axum_extra::TypedHeader;
use futures_util::Stream;
use tokio_stream::StreamExt;
// use futures_util::stream::{self, Stream};
use std::{path::PathBuf, sync::Arc};
use tokio::sync::Mutex;
use tokio_stream::wrappers::BroadcastStream;
use tower_http::{services::ServeDir, trace::TraceLayer};

// pub async fn start(
//     mut _rx: tokio::sync::broadcast::Receiver<nwws_oi::Message>,
// ) -> color_eyre::eyre::Result<()> {
//     // while let Ok(_message) = rx.recv().await {}

//     // build our application
//     let app = app();

//     // run it
//     let listener = tokio::net::TcpListener::bind("127.0.0.1:13579")
//         .await
//         .unwrap();
//     tracing::debug!("listening on {}", listener.local_addr().unwrap());
//     axum::serve(listener, app).await.unwrap();

//     Ok(())
// }

pub async fn startcap(
    tx_cap: Arc<Mutex<tokio::sync::broadcast::Sender<oasiscap::v1dot2::Alert>>>,
) -> color_eyre::eyre::Result<()> {
    // build our application
    let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");
    let static_files_service = ServeDir::new(assets_dir).append_index_html_on_directories(true);

    // build our application with a route
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

// fn app() -> Router {
//     let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");
//     let static_files_service = ServeDir::new(assets_dir).append_index_html_on_directories(true);
//     // build our application with a route
//     Router::new()
//         .fallback_service(static_files_service)
//         .route("/sse", get(sse_handler))
//         .layer(TraceLayer::new_for_http())
// }

async fn sse_handler(
    State(sender): State<Arc<Mutex<tokio::sync::broadcast::Sender<oasiscap::v1dot2::Alert>>>>,
    TypedHeader(user_agent): TypedHeader<headers::UserAgent>,
) -> Sse<impl Stream<Item = Result<Event, Error>>> {
    println!("`{}` connected", user_agent.as_str());

    let guard = sender.lock().await;
    let rx_message = (*guard).subscribe();
    let stream = BroadcastStream::new(rx_message);

    // A `Stream` that repeats an event every second
    //
    // You can also create streams from tokio channels using the wrappers in
    // https://docs.rs/tokio-stream
    // let stream = stream::repeat_with(|| Event::default().data("hi!"))
    //     .map(Ok)
    //     .throttle(Duration::from_secs(1));
    let stream = stream.map(move |item| Event::default().json_data(item.unwrap()));
    Sse::new(stream)
}
