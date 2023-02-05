use chrono::{NaiveTime, Weekday};
use hcl::ser::{Block, LabeledBlock};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
pub struct Events {
    pub event: LabeledBlock<IndexMap<String, LabeledBlock<Event>>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub discord: Option<Block<Discord>>,
    pub time: Block<Time>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<Url>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Discord {
    pub location: Location,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Location {
    #[serde(rename = "stage-channel")]
    StageChannel,

    #[serde(rename = "voice-channel")]
    VoiceChannel,

    #[serde(rename = "somewhere-else")]
    SomewhereElse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Time {
    pub start: NaiveTime,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<Block<Recurrence>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Recurrence {
    #[serde(rename = "daily")]
    Daily(Vec<Weekday>),

    #[serde(rename = "weekly")]
    Weekly,

    #[serde(rename = "monthly")]
    Monthly,

    #[serde(rename = "yearly")]
    Yearly,
}
