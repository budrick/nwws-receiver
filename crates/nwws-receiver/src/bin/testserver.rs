use std::str::FromStr;

use nwws_receiver::{config, nwwsoi};
use oasiscap::v1dot2::Status;
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
                let latest = alert.into_latest();
                if latest.status == Status::Actual {
                    println!("Bulletin: {} sent by {}", latest.identifier, latest.sender);
                    latest.info.iter().enumerate().for_each(|(i, info)| {
                        println!(
                            "[{}] {}",
                            i,
                            info.headline
                                .clone()
                                .unwrap_or_else(|| String::from("No headline"))
                        );
                    });
                    // for (index, info) in latest.info.iter().enumerate() {
                    //     println!("[{}] {}", index, "");
                    // }
                    println!("{:?}", &latest.info[0]);
                    println!("{}", "");
                }
            } else {
                println!("Failed to parse: {}", x);
            }
            // println!("{}", msg.message);
        }
        // if let Some(ref awid) = msg.awips_id {
        //     if let Some(id) = NWSProduct::from_str(&awid[..3]) {
        //         println!("AWIPS: {}; Product: {}", awid, id.description());
        //         if id == NWSProduct::HML {
        //             println!("{}", msg.message);
        //         }
        //     } else {
        //         println!("ttaaii: {};", msg.ttaaii);
        //         if &msg.ttaaii[..4] == "NTXX" || &msg.ttaaii[..2] == "XO" {
        //             println!("{}", msg.message);
        //         }
        //     }
        // }
    }
}

fn extractxml(message: &str) -> &str {
    let startpos = message.find('<').unwrap_or(0);
    let endpos = message.rfind('>').unwrap_or(0);
    &message[startpos..=endpos]
}
