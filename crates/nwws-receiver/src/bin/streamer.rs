extern crate dotenv;
use chrono::{DateTime, Utc};
use dotenv::dotenv;
use futures::StreamExt;
use nwws_oi::Channel;
use nwws_oi::Config;
use nwws_oi::Server;
use nwws_oi::StreamEvent;
use uuid;
// use rusqlite::{named_params, Connection};
// use rusqlite::{named_params, Connection, Result};

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::builder()
        .filter(None, log::LevelFilter::Info)
        .filter_module("nwws_oi", log::LevelFilter::Info)
        .parse_default_env()
        .init();

    let username = std::env::var("NWWS_OI_USERNAME").expect("NWWS_OI_USERNAME must be set");
    let password = std::env::var("NWWS_OI_PASSWORD").expect("NWWS_OI_PASSWORD must be set");

    let conf = Config {
        username,
        password,
        server: Server::Primary,
        resource: format!("uuid/{}", uuid::Uuid::new_v4()),
        channel: Channel::Default,
    };
    let mut stream = nwws_oi::Stream::new(conf);

    // Process messages when we get them.
    while let Some(event) = stream.next().await {
        match event {
            StreamEvent::ConnectionState(_state) => {}
            StreamEvent::Error(error) => log::error!("error: {}", error),
            StreamEvent::Message(message) => {
                let ttaa = &message.ttaaii[..4];
                let now: DateTime<Utc> = Utc::now();
                println!("{:?}", message);
            }
        }
    }
}
