use crate::types::CapAlert;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub enum Message {
    Alert(Box<CapAlert>),
    State(State),
    Dummy,
}
#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct Alert {
    headline: String,
}

impl Alert {
    pub fn from_capalert(alert: CapAlert) -> Alert {
        let headline = &alert.info[0].headline.clone();
        Self {
            headline: headline.clone().unwrap_or_else(|| "No Headline".to_owned()),
        }
    }
}

// #[derive(Debug)]
// pub struct Alert {
//     headline: String,
//     severity: String,
//     description: String,
// }

type State = Vec<Alert>;
