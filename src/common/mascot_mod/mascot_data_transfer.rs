use crate::common::mascot_mod::mascot::Mascot;
use serde::{Deserialize, Serialize};

/// Struct for transfering Mascot data for the logged in user between client and server
#[derive(Debug, Serialize, Deserialize)]
pub struct MascotDataServerClientTransfer {
    pub selected_mascot: Mascot,
    pub owned_mascots: Vec<Mascot>,
}
