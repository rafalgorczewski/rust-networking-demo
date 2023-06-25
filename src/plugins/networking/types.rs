use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize, PartialOrd, Ord)]
/// ID used to denote a connection from a client.
pub enum ClientId {
    Host,
    Id(u64),
}

impl Display for ClientId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClientId::Host => write!(f, "Host"),
            ClientId::Id(id) => write!(f, "{}", id),
        }
    }
}
