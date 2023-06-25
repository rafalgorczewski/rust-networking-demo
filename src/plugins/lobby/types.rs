use std::collections::HashMap;

use bimap::BiBTreeMap;
use serde::{Deserialize, Serialize};

use crate::plugins::networking::types::ClientId;

/// ID used as an abstraction to the ClientId.
pub type PlayerId = usize;

pub type PlayerMap = BiBTreeMap<ClientId, PlayerId>;

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct ClientInfo {
    pub nickname: String,
}
pub type Clients = HashMap<ClientId, ClientInfo>;

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct PlayerInfo {}
pub type Players = HashMap<PlayerId, PlayerInfo>;
