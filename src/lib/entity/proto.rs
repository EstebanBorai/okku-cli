use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entity::user::User;

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
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Chat {
    pub id: Uuid,
    pub messages: Vec<Message>,
    pub participants_ids: Vec<Uuid>,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Message {
    pub id: Uuid,
    pub body: String,
    pub chat: Chat,
    pub author: User,
    pub created_at: DateTime<Utc>,
}
