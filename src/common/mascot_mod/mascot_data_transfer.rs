use crate::common::mascot_mod::mascot::Mascot;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MascotDataServerClientTransfer {
    pub selected_mascot: Mascot,
    pub owned_mascots: Vec<Mascot>,
}
