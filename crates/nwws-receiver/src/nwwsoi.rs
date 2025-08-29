use crate::config::Config;
use chrono::DateTime;
use chrono::Utc;
use futures::StreamExt;
use nwws_oi::StreamEvent;

pub async fn start(
    conf: Config,
    sender: tokio::sync::mpsc::Sender<nwws_oi::Message>,
) -> color_eyre::eyre::Result<()> {
    let stream = nwws_oi::Stream::new(conf.nwwsoi);
    tokio::spawn(mainloop(stream, sender));
    Ok(())
}

async fn mainloop(
    mut stream: nwws_oi::Stream,
    sender: tokio::sync::mpsc::Sender<nwws_oi::Message>,
) {
    // Process messages when we get them.
    while let Some(event) = stream.next().await {
        match event {
            StreamEvent::ConnectionState(_state) => {}
            StreamEvent::Error(error) => log::error!("error: {}", error),
            StreamEvent::Message(message) => {
                let _ttaa = &message.ttaaii[..4];
                let _now: DateTime<Utc> = Utc::now();
                sender.send(message).await.unwrap();
            }
        }
    }
}
