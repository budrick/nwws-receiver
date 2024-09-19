extern crate dotenv;
use dotenv::dotenv;
use futures::StreamExt;
use nwws_oi::StreamEvent;
use nwws_oi::Server;
use nwws_oi::Config;
use nwws_oi::Channel;
use uuid;
use chrono::{DateTime, Utc};
use rusqlite::{named_params, Connection};
use anyhow::Result;
// use rusqlite::{named_params, Connection, Result};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::builder()
        .filter(None, log::LevelFilter::Info)
        .filter_module("nwws_oi", log::LevelFilter::Info)
        .parse_default_env()
        .init();

    let mut path = std::env::current_dir()?;
    path.push("bulletins");
    path.set_extension("sqlite");

    println!("Database path: {}", path.display());

    let username = std::env::var("NWWS_OI_USERNAME").expect("NWWS_OI_USERNAME must be set");
    let password = std::env::var("NWWS_OI_PASSWORD").expect("NWWS_OI_PASSWORD must be set");

    let conf = Config {
        username,
        password,
        server: Server::Primary,
        resource: format!("uuid/{}", uuid::Uuid::new_v4()),
        channel: Channel::Default
    };
    let mut stream = nwws_oi::Stream::new(conf);
    
    // Create an SQLite database, so that we might shove bulletins into it.
    let dbconn = Connection::open(path)?;
    // We want to store the timestamp, type (ttaa) and full text of bulletins
    dbconn.execute("CREATE TABLE IF NOT EXISTS bulletins (time_rfc3339 text, type text, bulletin text)", [])?;
    dbconn.execute("CREATE INDEX IF NOT EXISTS time_idx ON bulletins (time_rfc3339)", [])?;
    dbconn.execute("CREATE INDEX IF NOT EXISTS type_ids ON bulletins (type)", [])?;
    dbconn.execute("CREATE INDEX IF NOT EXISTS time_type ON bulletins (time_rfc3339, type)", [])?;
    let mut stmt = dbconn.prepare("INSERT INTO bulletins VALUES (:time, :type, :text)")?;
    
    // Process messages when we get them.
    while let Some(event) = stream.next().await {
        match event {
            StreamEvent::ConnectionState(_state) => {}
            StreamEvent::Error(error) => log::error!("error: {}", error),
            StreamEvent::Message(message) => {
                let ttaa = &message.ttaaii[..4];
                let now: DateTime<Utc> = Utc::now();
                match ttaa {
                    // Tornado warning
                    "WFUS" => {
                        println!("{}: Tornado warning issued by {}", now.to_rfc3339(), message.cccc);
                        let _res = stmt.execute(named_params!{":time": now.to_rfc3339(), ":type": ttaa, ":text": message.message });
                    },
                    // Severe thunderstorm warning
                    "WUUS" => {
                        // Ignore SPC updates. Unsure why they have the same WMO ttaa as severe thunderstorm warnings.
                        if message.cccc != "KWNS" {
                            println!("{}: Severe thunderstorm warning issued by {}", now.to_rfc3339(), message.cccc);
                            let _res = stmt.execute(named_params!{":time": now.to_rfc3339(), ":type": ttaa, ":text": message.message });
                        }
                    },
                    // Severe weather statement - may update/supercede WFUS or WUUS bulletins
                    "WWUS" => {
                        let _res = stmt.execute(named_params!{":time": now.to_rfc3339(), ":type": ttaa, ":text": message.message });
                    },
                    // Fall through
                    _ => ()
                }
                // log::info!("{}", format!("{:#?}", message));
            }
        }
    }
    Ok(())
}
