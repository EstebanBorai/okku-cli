use serde::{Deserialize};

use crate::entity::message::Message;
use crate::entity::profile::Profile;
use crate::entity::user::User;

#[derive(Debug, Deserialize)]
pub struct LoginResponse {
    pub(crate) token: String,
}

#[derive(Debug, Deserialize)]
pub struct MeResponse {
    pub(crate) user: User,
    pub(crate) profile: Profile,
}

#[derive(Debug, Deserialize)]
pub struct FetchChatMessagesResponse {
    pub(crate) messages: Vec<Message>,
}
