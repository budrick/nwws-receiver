use crate::types::CapAlert;
use serde::{Deserialize, Serialize};
pub type Map = oasiscap::map::Map<crate::map::Entry>;

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
pub struct Info {
    /// The language of this `Info` section.
    pub language: oasiscap::language::Language,

    /// Zero or more categories describing the subject event.
    pub categories: Vec<oasiscap::v1dot2::Category>,

    /// Text describing the subject event.
    pub event: String,

    /// The recommended type of action for the target audience.
    pub response_type: Vec<oasiscap::v1dot2::ResponseType>,

    /// The time available to prepare for the subject event.
    pub urgency: oasiscap::v1dot2::Urgency,

    /// The intensity of impact of the subject event.
    pub severity: oasiscap::v1dot2::Severity,

    /// The confidence in the observation or prediction.
    pub certainty: oasiscap::v1dot2::Certainty,

    /// The target audience of the alert message.
    pub audience: Option<String>,

    /// System-specific codes identifying the event type of the alert message
    pub event_codes: oasiscap::map::Map<crate::map::Entry>,

    /// The effective time of the information of the alert message
    ///
    /// If this item is not included, the effective time SHALL be assumed to be the same as in
    /// `sent`.
    pub effective: Option<oasiscap::DateTime>,

    /// The expected time of the beginning of the subject event of the alert message.
    pub onset: Option<oasiscap::DateTime>,

    /// The expiry time of the information of the alert message.
    ///
    /// If this item is not provided, each recipient is free to set its own policy as to when the
    /// message is no longer in effect.
    pub expires: Option<oasiscap::DateTime>,

    /// The human-readable name of the agency or authority issuing this alert.
    pub sender_name: Option<String>,

    /// A brief human-readable headline.
    ///
    /// Note that some displays (for example, short messaging service devices) may only present this
    /// headline; it SHOULD be made as direct and actionable as possible while remaining short. 160
    /// characters MAY be a useful target limit for headline length.
    pub headline: Option<String>,

    /// An extended human readable description of the hazard or event that occasioned this message.
    pub description: Option<String>,

    /// An extended human readable instruction to targeted recipients. If different instructions are
    /// intended for different recipients, they should be represented by use of multiple `Info`
    /// blocks.
    pub instruction: Option<String>,

    /// A full, absolute URI for an HTML page or other text resource with additional or reference
    /// information regarding this alert.
    pub web: Option<oasiscap::Url>,

    /// The text describing the contact for follow-up and confirmation of the alert message
    pub contact: Option<String>,

    /// System-specific additional parameters associated with the alert message
    pub parameters: oasiscap::map::Map<crate::map::Entry>,

    pub resources: Vec<oasiscap::v1dot2::Resource>,

    /// Geographical (and usually also geospatial) information describing the expected or actual
    /// location of the event.
    pub areas: Vec<oasiscap::v1dot2::Area>,
}
impl From<oasiscap::v1dot2::Info> for Info {
    fn from(value: oasiscap::v1dot2::Info) -> Self {
        let event_codes: oasiscap::map::Map<crate::map::Entry> = value.event_codes.iter().collect();
        let parameters: oasiscap::map::Map<crate::map::Entry> = value.parameters.iter().collect();
        // let resources: Vec<Resource> = value.resources.iter().collect();
        Self {
            language: value.language,
            categories: value.categories,
            event: value.event,
            response_type: value.response_type,
            urgency: value.urgency,
            severity: value.severity,
            certainty: value.certainty,
            audience: value.audience,
            event_codes,
            effective: value.effective,
            onset: value.onset,
            expires: value.expires,
            sender_name: value.sender_name,
            headline: value.headline,
            description: value.description,
            instruction: value.instruction,
            web: value.web,
            contact: value.contact,
            parameters,
            resources: value.resources,
            areas: value.areas,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    pub id: oasiscap::id::Id,
    pub headline: Option<String>,
    pub sent: oasiscap::DateTime,
    pub info: Vec<Info>,
    pub status: oasiscap::v1dot2::Status,
    pub sender: oasiscap::id::Id, // pub cap: CapAlert,
}

impl From<CapAlert> for Alert {
    fn from(value: CapAlert) -> Self {
        let info = value.info.iter().map(|i| Info::from(i.clone())).collect();
        Self {
            id: value.identifier,
            headline: value.info[0].headline.clone(),
            info,
            sent: value.sent,
            status: value.status,
            sender: value.sender,
        }
    }
}

type State = Vec<Alert>;
