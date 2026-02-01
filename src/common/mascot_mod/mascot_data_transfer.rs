use serde::{Deserialize, Serialize};
use crate::common::mascot_mod::mascot::Mascot;

#[derive(Debug, Serialize, Deserialize)]
pub struct MascotDataServerClientTransfer {
    pub selected_mascot: Mascot,
    pub owned_mascots: Vec<Mascot>
}