use crate::message::Message;
use std::sync::Arc;
use tokio::sync::{broadcast::Receiver, broadcast::Sender, Mutex};

pub type NwwsSender = Sender<nwws_oi::Message>;
pub type NwwsReceiver = Receiver<nwws_oi::Message>;

pub type CapSender = Sender<Message>;
pub type CapReceiver = Receiver<Message>;

// pub type MessageSender = Sender<Message>;
// pub type MessageReceiver = Receiver<Message>;

pub type SharedCapSender = Arc<Mutex<CapSender>>;

pub type CapAlert = oasiscap::v1dot2::Alert;
