use crate::cap::extractxml;
use crate::config::Config;
use chrono::DateTime;
use chrono::Utc;
use futures::StreamExt;
use nwws_oi::StreamEvent;
use std::str::FromStr;

pub async fn start(
    conf: Config,
    sender: tokio::sync::broadcast::Sender<nwws_oi::Message>,
) -> color_eyre::eyre::Result<()> {
    let stream = nwws_oi::Stream::new(conf.nwwsoi);
    tokio::spawn(mainloop(stream, sender));
    Ok(())
}

async fn mainloop(
    mut stream: nwws_oi::Stream,
    sender: tokio::sync::broadcast::Sender<nwws_oi::Message>,
) {
    // Process messages when we get them.
    while let Some(event) = stream.next().await {
        match event {
            StreamEvent::ConnectionState(_state) => {}
            StreamEvent::Error(error) => log::error!("error: {}", error),
            StreamEvent::Message(message) => {
                let _ttaa = &message.ttaaii[..4];
                let _now: DateTime<Utc> = Utc::now();
                sender.send(message).unwrap();
            }
        }
    }
}

pub async fn startcap(
    receiver: tokio::sync::broadcast::Receiver<nwws_oi::Message>,
    sender: tokio::sync::broadcast::Sender<oasiscap::v1dot2::Alert>,
) -> color_eyre::eyre::Result<()> {
    // let stream = nwws_oi::Stream::new(conf.nwwsoi);
    tokio::spawn(caploop(receiver, sender));
    Ok(())
}
async fn caploop(
    mut receiver: tokio::sync::broadcast::Receiver<nwws_oi::Message>,
    sender: tokio::sync::broadcast::Sender<oasiscap::v1dot2::Alert>,
) {
    // Process messages when we get them.
    while let Ok(msg) = receiver.recv().await {
        if &msg.ttaaii[..1] == "X" {
            let x = extractxml(&msg.message);
            if let Ok(alert) = oasiscap::Alert::from_str(x) {
                if sender.send(alert.into_latest()).is_ok() {}
            } else {
                println!("Failed to parse: {}", x);
            }
        }
    }
}
