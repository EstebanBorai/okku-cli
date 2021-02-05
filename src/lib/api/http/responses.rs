use serde::{Deserialize, Serialize};

use crate::entity::profile::Profile;
use crate::entity::user::User;

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginResponse {
    pub(crate) token: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MeResponse {
    pub(crate) user: User,
    pub(crate) profile: Profile,
}
