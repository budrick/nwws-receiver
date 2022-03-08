extern crate dotenv;
use dotenv::dotenv;
use futures::StreamExt;
use nwws_oi::StreamEvent;
use chrono::{DateTime, Utc};
// use sqlite;

// fn get_pwd() -> std::path::PathBuf {
//     let p = std::env::current_dir();
//     match p {
//         Ok(p) => p,
//         Err(_e) => std::path::PathBuf::new()
//     }
// }

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::builder()
        .filter(None, log::LevelFilter::Info)
        .filter_module("nwws_oi", log::LevelFilter::Info)
        .parse_default_env()
        .init();

    let mut path = std::env::current_dir().unwrap();
    path.push("bulletins");
    path.set_extension("sqlite");

    println!("Path {}", path.display());

    let username = std::env::var("NWWS_OI_USERNAME").expect("NWWS_OI_USERNAME must be set");
    let password = std::env::var("NWWS_OI_PASSWORD").expect("NWWS_OI_PASSWORD must be set");

    let mut stream = nwws_oi::Stream::new((username, password));

    while let Some(event) = stream.next().await {
        match event {
            StreamEvent::ConnectionState(_state) => {}
            StreamEvent::Error(error) => log::error!("error: {}", error),
            StreamEvent::Message(message) => {
                let ttaa = &message.ttaaii[..4];
                let now: DateTime<Utc> = Utc::now();
                match ttaa {
                    // Tornado warning
                    "WFUS" => println!("{}: Tornado warning issued by {}", now.to_rfc3339(), message.cccc),
                    // Severe thunderstorm warning
                    "WUUS" => println!("{}: Severe thunderstorm warning issued by {}", now.to_rfc3339(), message.cccc),
                    // Severe weather statement - may update/supercede WFUS or WUUS bulletins
                    "WWUS" => (),
                    // Fall through
                    _ => ()
                }
                // log::info!("{}", format!("{:#?}", message));
            }
        }
    }
}
