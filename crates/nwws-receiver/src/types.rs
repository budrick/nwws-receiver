use std::sync::Arc;
use tokio::sync::Mutex;

pub type NwwsSender = tokio::sync::broadcast::Sender<nwws_oi::Message>;
pub type NwwsReceiver = tokio::sync::broadcast::Receiver<nwws_oi::Message>;

pub type CapSender = tokio::sync::broadcast::Sender<oasiscap::v1dot2::Alert>;
pub type CapReceiver = tokio::sync::broadcast::Receiver<oasiscap::v1dot2::Alert>;

pub type SharedCapSender = Arc<Mutex<CapSender>>;
