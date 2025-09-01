use oasiscap::v1dot2::Alert;

#[derive(Debug, Clone)]
pub enum Message {
    Alert(Box<Alert>),
    State(State),
    Dummy,
}

// #[derive(Debug)]
// pub struct Alert {
//     headline: String,
//     severity: String,
//     description: String,
// }

type State = Vec<Alert>;
