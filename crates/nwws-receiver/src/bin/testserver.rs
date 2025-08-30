use nwws_receiver::termlog;
use std::str::FromStr;

use nwws_receiver::{config, nwwsoi, web};
use tokio::signal;
use tokio::sync::broadcast;

#[tokio::main]
async fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;
    dotenv::dotenv()?;
    let conf = config::get();
    let (tx, _) = broadcast::channel(32);
    tokio::try_join! {
        nwwsoi::start(conf.clone(), tx.clone()),
        startprintloop(tx.subscribe()),
        web::start(tx.subscribe()),
    }?;

    signal::ctrl_c().await.expect("Couldn't listen to Ctrl-C");

    println!("Received Ctrl-C");

    Ok(())
}

async fn startprintloop(
    receiver: broadcast::Receiver<nwws_oi::Message>,
) -> color_eyre::eyre::Result<()> {
    tokio::spawn(printcap(receiver));
    Ok(())
}

async fn printcap(mut receiver: broadcast::Receiver<nwws_oi::Message>) {
    while let Ok(msg) = receiver.recv().await {
        // println!("ttaaii: {};", msg.ttaaii);
        if &msg.ttaaii[..1] == "X" {
            let x = extractxml(&msg.message);
            if let Ok(alert) = oasiscap::Alert::from_str(x) {
                termlog::printcap(alert);
            } else {
                println!("Failed to parse: {}", x);
            }
        }
    }
}

fn extractxml(message: &str) -> &str {
    let startpos = message.find('<').unwrap_or(0);
    let endpos = message.rfind('>').unwrap_or(0);
    &message[startpos..=endpos]
}
