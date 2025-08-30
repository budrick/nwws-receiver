pub async fn start(
    mut rx: tokio::sync::broadcast::Receiver<nwws_oi::Message>,
) -> color_eyre::eyre::Result<()> {
    while let Ok(_message) = rx.recv().await {}
    Ok(())
}
