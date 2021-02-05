use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entity::user::User;

#[derive(Debug, Deserialize, Serialize)]
pub struct Profile {
    pub id: Uuid,
    pub first_name: Option<String>,
    pub email: Option<String>,
    // This field is not updated in regards to okku-server
    pub avatar: Option<Uuid>,
    pub surname: Option<String>,
    pub birthday: Option<DateTime<Utc>>,
    pub contacts: Option<Vec<User>>,
    pub bio: Option<String>,
}
