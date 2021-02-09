use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::chat::Chat;
use super::message::Message;
use super::user::User;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", content = "inner")]
pub enum Parcel {
    #[serde(rename = "message")]
    LocalMessage(Message),
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Proto {
    Input(InputParcel),
    Output(OutputParcel),
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct OutputParcelPayload {
    pub author_id: Uuid,
    pub chat_id: Uuid,
    pub body: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct OutputParcel {
    pub inner: OutputParcelPayload,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InputParcel {
    pub id: Uuid,
    pub body: String,
    pub chat: Chat,
    pub author: User,
    pub created_at: DateTime<Utc>,
}
