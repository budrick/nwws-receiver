use nws_product_list::NWSProduct;
use nwws_receiver::{config, nwws};
use tokio::signal;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;
    dotenv::dotenv()?;
    let conf = config::get();
    let (tx, rx) = mpsc::channel(32);
    tokio::try_join! {
        nwws::start(conf.clone(), tx.clone()),
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
        if let Some(ref awid) = msg.awips_id {
            if let Some(id) = NWSProduct::from_str(&awid[..3]) {
                println!("AWIPS: {}; Product: {}", awid, id.description());
                if id == NWSProduct::HML {
                    println!("{}", msg.message);
                }
            } else {
                println!("ttaaii: {};", msg.ttaaii);
                if &msg.ttaaii[..4] == "NTXX" || &msg.ttaaii[..2] == "XO" {
                    println!("{}", msg.message);
                }
            }
        }
    }
}
