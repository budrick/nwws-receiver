use crate::config::Config;
use crate::message::Message;
use crate::types::CapSender;
use crate::util::extractxml;
use color_eyre::eyre::Result;
use nwws_oi::StreamEvent;
use std::str::FromStr;
use tokio_stream::StreamExt;

pub async fn startstream(conf: Config, tx: CapSender) -> Result<()> {
    let stream = nwws_oi::Stream::new(conf.nwwsoi);
    tokio::spawn(streamloop(stream, tx));

    Ok(())
}

async fn streamloop(stream: nwws_oi::Stream, tx: CapSender) {
    // async fn streamloop(stream: nwws_oi::Stream) {
    let mut stream = StreamExt::filter_map(stream, |event| match event {
        StreamEvent::Message(message) => {
            if &message.ttaaii[..1] == "X" {
                let x = extractxml(&message.message);
                if let Ok(alert) = oasiscap::Alert::from_str(x) {
                    Some(Message::from(alert.into_latest()))
                } else {
                    println!("Failed to parse: {}", x);
                    None
                }
            } else {
                None
            }
        }
        _ => None,
    });

    while let Some(message) = stream.next().await {
        // Ignoring the result is an antipattern, but go with it for now.
        if let Err(e) = tx.send(Box::new(message)) {
            println!("nwwsoi: Error broadcasting, {}", e);
        }
    }
}
