use oasiscap::v1dot2::Status;
use oasiscap::Alert;
use owo_colors::OwoColorize;

pub fn printcap(alert: Alert) {
    let latest = alert.into_latest();
    if latest.status == Status::Actual {
        println!(
            "{} sent by {}",
            latest.identifier.green(),
            latest.sender.yellow()
        );
        latest.info.iter().enumerate().for_each(|(i, info)| {
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
            latest.identifier.blue().underline()
        );
        println!();
    }
}
