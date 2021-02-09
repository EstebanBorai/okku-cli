use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::chat::Chat;
use super::user::User;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Message {
    pub id: Uuid,
    pub body: String,
    pub chat: Chat,
    pub author: User,
    pub created_at: DateTime<Utc>,
}
