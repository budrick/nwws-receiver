use crate::message::Alert;
use crate::message::Message;
use oasiscap::v1dot2::Alert as CapAlert;
use oasiscap::v1dot2::Status;
use owo_colors::OwoColorize;

use crate::types::CapReceiver;

fn printcap(alert: CapAlert) {
    // let latest = alert.into_latest();
    if alert.status == Status::Actual {
        println!(
            "{} sent by {}",
            alert.identifier.green(),
            alert.sender.yellow()
        );
        alert.info.iter().enumerate().for_each(|(i, info)| {
            println!(
                "[{}] {}",
                i.green(),
                info.headline
                    .clone()
                    .unwrap_or_else(|| String::from("No headline"))
            );
        });
        println!(
            "{}{}",
            "https://api.weather.gov/alerts/".blue().underline(),
            alert.identifier.blue().underline()
        );
        println!();
    }
}

fn printalert(alert: Alert) {
    // let latest = alert.into_latest();
    if alert.status == Status::Actual {
        println!("{} sent by {}", alert.id.green(), alert.sender.yellow());
        alert.info.iter().enumerate().for_each(|(i, info)| {
            println!(
                "[{}] {}",
                i.green(),
                info.headline
                    .clone()
                    .unwrap_or_else(|| String::from("No headline"))
            );
        });
        println!(
            "{}{}",
            "https://api.weather.gov/alerts/".blue().underline(),
            alert.id.blue().underline()
        );
        println!();
    }
}

pub async fn startcap(mut rx: CapReceiver) -> color_eyre::eyre::Result<()> {
    while let Ok(message) = rx.recv().await {
        if let Message::Alert(alert) = *message {
            printalert(*alert);
        }
    }
    Ok(())
}
