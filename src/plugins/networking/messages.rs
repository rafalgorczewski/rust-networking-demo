use serde::{Deserialize, Serialize};

use crate::plugins::{
    game::unit::components::Destination,
    lobby::{resources::Lobby, types::PlayerId},
};

use super::utils::{FromClientChannel, FromServerChannel};

#[derive(Serialize, Deserialize, Clone)]
pub enum MessageToClient {
    // Lobby
    /// When a player joins or leaves the lobby.
    /// Contains the current state of the lobby.
    LobbySync(Lobby),
    /// PlayerId that got assigned to the client.
    AssignedPlayerId(PlayerId),
    /// When a match got started.
    MatchStarted,
    // Game
    /// When unit destination changed.
    DestinationChanged {
        player_id: PlayerId,
        destination: Destination,
    },
}

#[derive(Serialize, Deserialize, Clone)]
pub enum MessageToServer {
    // Game
    /// When player destination change was requested.
    DestinationRequested(Destination),
}

impl MessageToClient {
    pub fn as_channel(&self) -> FromServerChannel {
        match self {
            MessageToClient::LobbySync(_) => FromServerChannel::LobbyMessages,
            MessageToClient::AssignedPlayerId(_) => FromServerChannel::LobbyMessages,
            MessageToClient::MatchStarted => FromServerChannel::LobbyMessages,
            MessageToClient::DestinationChanged {
                player_id: _,
                destination: _,
            } => FromServerChannel::Command,
        }
    }
}

impl MessageToServer {
    pub fn as_channel(&self) -> FromClientChannel {
        match self {
            MessageToServer::DestinationRequested(_) => FromClientChannel::Command,
        }
    }
}
