use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::plugins::networking::types::ClientId;

use super::{
    constants::HOST_PLAYER_ID,
    types::{ClientInfo, Clients, PlayerId, PlayerInfo, PlayerMap, Players},
};

/// PlayerId assigned to Player in current lobby.
#[derive(Resource)]
pub struct CurrentPlayerId(pub PlayerId);

#[derive(Resource, Serialize, Deserialize, Clone)]
pub struct Lobby {
    player_map: PlayerMap,
    clients: Clients,
    players: Players,
}

impl Default for Lobby {
    fn default() -> Self {
        let mut player_map = PlayerMap::default();
        player_map.insert(ClientId::Host, HOST_PLAYER_ID);

        Self {
            player_map,
            clients: Default::default(),
            players: Default::default(),
        }
    }
}

impl Lobby {
    /// Adds a new player to the lobby assigning him a free PlayerId.
    pub fn add_player(&mut self, client_id: ClientId) {
        if let Some(player_id) = self.get_unused_player_id() {
            self.insert_player(client_id, player_id);
        }
    }
    /// Inserts a new player to the lobby with overwrite capabilities.
    pub fn insert_player(&mut self, client_id: ClientId, player_id: PlayerId) {
        let did_overwrite = self.player_map.insert(client_id, player_id).did_overwrite();
        info!(
            "Inserted a player to the lobby. ClientId: {}, PlayerId: {}, Overwritten: {}",
            client_id, player_id, did_overwrite
        );
    }
    pub fn remove_player_by_client_id(&mut self, client_id: ClientId) {
        if let Some((old_client_id, old_player_id)) = self.player_map.remove_by_left(&client_id) {
            info!(
                "Removed a player from the lobby. ClientId: {}, PlayerId: {}",
                old_client_id, old_player_id
            );
        } else {
            info!(
                "Couldn't remove a player from the lobby! No player with ClientId: {}",
                client_id
            );
        }
    }
    pub fn remove_player_by_player_id(&mut self, player_id: PlayerId) {
        if let Some((old_client_id, old_player_id)) = self.player_map.remove_by_right(&player_id) {
            info!(
                "Removed a player from the lobby. ClientId: {}, PlayerId: {}",
                old_client_id, old_player_id
            );
        } else {
            info!(
                "Couldn't remove a player from the lobby! No player with PlayerId: {}",
                player_id
            );
        }
    }

    pub fn get_players(&self) -> Vec<PlayerId> {
        self.player_map.right_values().copied().collect()
    }

    pub fn get_client_id(&self, player_id: PlayerId) -> Option<ClientId> {
        self.player_map.get_by_right(&player_id).copied()
    }
    pub fn get_player_id(&self, client_id: ClientId) -> Option<PlayerId> {
        self.player_map.get_by_left(&client_id).copied()
    }
    pub fn get_client_info(&self, client_id: ClientId) -> Option<&ClientInfo> {
        self.clients.get(&client_id)
    }
    pub fn get_player_info(&self, player_id: PlayerId) -> Option<&PlayerInfo> {
        self.players.get(&player_id)
    }

    pub fn set_client_info(&mut self, client_id: ClientId, info: ClientInfo) {
        self.clients.insert(client_id, info);
    }
    pub fn set_player_info(&mut self, player_id: PlayerId, info: PlayerInfo) {
        self.players.insert(player_id, info);
    }

    fn get_unused_player_id(&self) -> Option<PlayerId> {
        (PlayerId::MIN..PlayerId::MAX).find(|&i| !self.player_map.contains_right(&i))
    }
}
