#[derive(Debug)]
pub enum Message {
    Alert(Alert),
    State(State),
}

pub struct Alert {
    headline: String,
    severity: String,
    description: String,
}

type State = Vec<Alert>;
