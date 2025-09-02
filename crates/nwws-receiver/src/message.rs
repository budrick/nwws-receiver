use crate::types::CapAlert;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub enum Message {
    Alert(Box<Alert>),
    State(State),
    Dummy,
    Empty,
}

impl From<CapAlert> for Message {
    fn from(value: oasiscap::v1dot2::Alert) -> Self {
        Self::Alert(Box::new(Alert::from(value)))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    pub id: oasiscap::id::Id,
    pub headline: Option<String>,
    pub info: Vec<oasiscap::v1dot2::Info>,
    pub status: oasiscap::v1dot2::Status,
    pub sender: oasiscap::id::Id, // pub cap: CapAlert,
}

impl From<CapAlert> for Alert {
    fn from(value: CapAlert) -> Self {
        Self {
            id: value.identifier,
            headline: value.info[0].headline.clone(),
            info: value.info.clone(),
            status: value.status,
            sender: value.sender,
        }
    }
}

type State = Vec<Alert>;
