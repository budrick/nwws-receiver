use nwws_receiver::termlog;
use std::str::FromStr;

use nwws_receiver::{config, nwwsoi};
use tokio::signal;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;
    dotenv::dotenv()?;
    let conf = config::get();
    let (tx, rx) = mpsc::channel(32);
    tokio::try_join! {
        nwwsoi::start(conf.clone(), tx.clone()),
         startprintloop(rx)
    }?;

    signal::ctrl_c().await.expect("Couldn't listen to Ctrl-C");

    println!("Received Ctrl-C");

    Ok(())
}

async fn startprintloop(
    receiver: mpsc::Receiver<nwws_oi::Message>,
) -> color_eyre::eyre::Result<()> {
    tokio::spawn(printcap(receiver));
    Ok(())
}

async fn printcap(mut receiver: mpsc::Receiver<nwws_oi::Message>) {
    while let Some(msg) = receiver.recv().await {
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
