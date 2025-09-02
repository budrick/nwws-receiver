use nwws_receiver::types::SharedCapSender;
use nwws_receiver::{config, nwwsoi, termlog, web};
use std::sync::Arc;
use tokio::signal;
use tokio::sync::broadcast;
use tokio::sync::Mutex;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;
    dotenv::dotenv()?;

    init_tracing();

    let conf = config::get();
    let (tx, _) = broadcast::channel(32);
    let tx_cap: SharedCapSender = Arc::new(Mutex::new(tx.clone()));

    tokio::try_join! {
        nwwsoi::startstream(conf.clone(), tx.clone()), // Get NWWS messages, send successful CAP decodes to broadcast channel
        web::startcap(conf.clone(), tx_cap), // Receive CAP messages and broadcast to SSE clients
        termlog::startcap(tx.subscribe()), // Receive CAP messages and print
        // nwwsoi::startcap(tx.subscribe(), captx), // Receive NWWS messages, emit CAP. Here because move restrictions.
    }?;

    signal::ctrl_c().await.expect("Couldn't listen to Ctrl-C");

    println!("Received Ctrl-C");

    Ok(())
}

fn init_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}
