use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FriendRequest {
    /// The Person you want to add as a friend
    pub username: String,
}
