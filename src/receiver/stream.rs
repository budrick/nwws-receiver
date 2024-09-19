use nwws_oi::StreamEvent;
use nwws_oi::Server;
use nwws_oi::Config;
use nwws_oi::Channel;

use chrono::{DateTime, Utc};

pub fn create<C: Into<Config>>(config: C) {
    let mut stream = nwws_oi::Stream::new(config);
    while let Some(event) = stream.next().await {
        handle(event);
    }
}

fn handle(event: StreamEvent) {
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