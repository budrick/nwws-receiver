use crate::types::CapAlert;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub enum Message {
    Alert(Box<Alert>),
    State(State),
    Dummy,
    Empty,
}
#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct Alert {
    pub headline: Option<String>,
    pub cap: CapAlert,
}

// impl Alert {
//     pub fn from_capalert(alert: CapAlert) -> Alert {
//         let headline = &alert.info[0].headline.clone();
//         Self {
//             headline: headline.clone(),
//             cap: alert,
//         }
//     }
// }
impl From<CapAlert> for Alert {
    fn from(value: CapAlert) -> Self {
        Self {
            headline: value.info[0].headline.clone(),
            cap: value,
        }
    }
}
impl From<CapAlert> for Message {
    fn from(value: oasiscap::v1dot2::Alert) -> Self {
        Self::Alert(Box::new(Alert::from(value)))
    }
}

// #[derive(Debug)]
// pub struct Alert {
//     headline: String,
//     severity: String,
//     description: String,
// }

type State = Vec<Alert>;
