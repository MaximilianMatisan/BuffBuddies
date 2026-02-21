use serde::{Deserialize, Serialize};

/// Primarily used to send new Friendship data from client to server
#[derive(Debug, Serialize, Deserialize)]
pub struct FriendRequest {
    /// The Person you want to add/delete as a friend
    pub username: String,
}
