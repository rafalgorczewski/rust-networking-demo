use bevy::prelude::*;
use bevy_renet::renet::{RenetClient, RenetServer, ServerEvent};

use crate::{
    plugins::{
        lobby::{
            constants::HOST_PLAYER_ID,
            resources::{CurrentPlayerId, Lobby},
        },
        mainmenu::events::{ConnectRequestedEvent, GameStartRequestedEvent, HostRequestedEvent},
        networking::types::ClientId,
    },
    states::{GameState, NetworkingState},
};

use super::{
    events::{SendMessageToClientEvent, SendMessageToServerEvent},
    messages::MessageToClient,
    resources::ConnectionRetryTimer,
    utils::{
        construct_new_client, construct_new_server, send_message_to_client, send_message_to_server,
    },
};

pub(super) fn on_send_message_to_client(
    mut event_reader: EventReader<SendMessageToClientEvent>,
    mut server: ResMut<RenetServer>,
) {
    for event in event_reader.iter() {
        // Always broadcast to allow replication.
        send_message_to_client(None, event.0.clone(), &mut server)
    }
}

pub(super) fn on_send_message_to_server(
    mut event_reader: EventReader<SendMessageToServerEvent>,
    mut client: ResMut<RenetClient>,
) {
    for event in event_reader.iter() {
        send_message_to_server(event.0.clone(), &mut client)
    }
}

pub(super) fn on_host_requested(
    mut event_reader: EventReader<HostRequestedEvent>,
    mut commands: Commands,
    mut networking_state: ResMut<NextState<NetworkingState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if event_reader.iter().next().is_some() {
        let result = construct_new_server();
        match result {
            Ok(server) => {
                info!("Started server hosting. Address: {}", server.addr());

                commands.insert_resource(server);

                commands.insert_resource(CurrentPlayerId(HOST_PLAYER_ID));

                networking_state.set(NetworkingState::Hosting);
                game_state.set(GameState::Lobby);
            }
            Err(error) => {
                error!("Couldn't start server hosting! {}", error);
            }
        }
    }
    event_reader.clear();
}

pub(super) fn on_connect_requested(
    mut event_reader: EventReader<ConnectRequestedEvent>,
    mut commands: Commands,
    mut networking_state: ResMut<NextState<NetworkingState>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut connection_timer: ResMut<ConnectionRetryTimer>,
) {
    if let Some(event) = event_reader.iter().next() {
        let result = construct_new_client(event.0);
        match result {
            Ok(client) => {
                info!("Client created. Client ID: {}", client.client_id());

                if client.is_connected() {
                    info!("Client is now connected!");

                    networking_state.set(NetworkingState::Connected);
                    game_state.set(GameState::Lobby);
                } else if client.is_connecting() {
                    info!("Trying to connect. Starting the timer.");

                    networking_state.set(NetworkingState::Connecting);

                    connection_timer.timer.reset();
                    connection_timer.timer.unpause();
                    connection_timer.stopwatch.reset();
                    connection_timer.stopwatch.unpause();
                } else {
                    info!("Client couldn't connect to the server!");
                }
                commands.insert_resource(client);
            }
            Err(error) => {
                error!("Couldn't create the client! Error: {}", error);
            }
        }
    }
    event_reader.clear();
}

pub(super) fn on_server_event(
    mut event_reader: EventReader<ServerEvent>,
    mut lobby: ResMut<Lobby>,
    mut server: ResMut<RenetServer>,
) {
    for event in event_reader.iter() {
        match event {
            ServerEvent::ClientConnected(client_id, _) => {
                info!("Client connected. ClientId: {}", client_id);

                let client_id = ClientId::Id(*client_id);

                lobby.add_player(client_id);

                if let Some(player_id) = lobby.get_player_id(client_id) {
                    send_message_to_client(
                        Some(client_id),
                        MessageToClient::AssignedPlayerId(player_id),
                        &mut server,
                    );
                } else {
                    error!(
                        "No PlayerId found for requested ClientId! ClientId: {}",
                        client_id
                    );
                }
            }
            ServerEvent::ClientDisconnected(client_id) => {
                info!("Client disconnected. ClientId: {}", client_id);

                lobby.remove_player_by_client_id(ClientId::Id(*client_id));
            }
        }
        // Sync lobby
        send_message_to_client(None, MessageToClient::LobbySync(lobby.clone()), &mut server);
    }
}

pub(super) fn on_game_started(
    mut event_reader: EventReader<GameStartRequestedEvent>,
    mut server: ResMut<RenetServer>,
) {
    if !event_reader.is_empty() {
        info!("Sending start game request to clients...");
        send_message_to_client(None, MessageToClient::MatchStarted, &mut server);

        event_reader.clear();
    }
}
