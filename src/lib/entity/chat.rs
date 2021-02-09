use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::message::Message;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Chat {
    pub id: Uuid,
    pub messages: Vec<Message>,
    pub participants_ids: Vec<Uuid>,
}
