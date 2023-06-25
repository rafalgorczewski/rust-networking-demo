use bevy::prelude::*;
use bevy_renet::renet::{RenetClient, RenetServer};
use strum::IntoEnumIterator;

use crate::{
    plugins::{
        game::{
            player::{components::{Ownership, Player}, events::PlayerDestinationRequestedEvent},
            unit::components::Destination,
        },
        lobby::resources::{CurrentPlayerId, Lobby},
    },
    states::{GameState, NetworkingState},
};

use super::{
    constants::CONNECTION_RETRY_TIMEOUT,
    messages::{MessageToClient, MessageToServer},
    resources::ConnectionRetryTimer,
    types::ClientId,
    utils::{FromClientChannel, FromServerChannel},
};

pub(super) fn check_connection(
    time: Res<Time>,
    mut connection_timer: ResMut<ConnectionRetryTimer>,
    mut game_state: ResMut<NextState<GameState>>,
    mut networking_state: ResMut<NextState<NetworkingState>>,
    client: Res<RenetClient>,
) {
    connection_timer.timer.tick(time.delta());
    connection_timer.stopwatch.tick(time.delta());

    let mut should_stop = false;

    if connection_timer.timer.just_finished() {
        info!("Retrying connection...");
        if client.is_connected() {
            info!("Client connected. Stopping the timer.");
            should_stop = true;

            networking_state.set(NetworkingState::Connected);
            game_state.set(GameState::Lobby);
        } else {
            info!("Client still connecting.");
        }
    }
    if connection_timer.stopwatch.elapsed_secs() > CONNECTION_RETRY_TIMEOUT.as_secs_f32() {
        info!("Client connection timed out. Stopping the timer.");

        networking_state.set(NetworkingState::Disconnected);
        should_stop = true;
    }

    if should_stop {
        connection_timer.timer.pause();
        connection_timer.stopwatch.pause();
    }
}

pub(super) fn handle_messages_from_client(
    mut server: ResMut<RenetServer>,
    lobby: Res<Lobby>,
    mut player_destination_ew: EventWriter<PlayerDestinationRequestedEvent>,
) {
    for channel in FromClientChannel::iter() {
        for client_id in server.clients_id().into_iter() {
            while let Some(serialized_message) = server.receive_message(client_id, channel as u8) {
                let message = bincode::deserialize(&serialized_message);
                match message {
                    Err(error) => {
                        error!("Couldn't deserialize network message! Error: {}", error)
                    }
                    Ok(message) => match message {
                        MessageToServer::DestinationRequested(destination) => {
                            // If client is in lobby and has PlayerId assigned
                            if let Some(player_id) = lobby.get_player_id(ClientId::Id(client_id)) {
                                info!("Destination change requested by Player. PlayerId: {}, Destination: {:?}", player_id, destination);

                                player_destination_ew.send(PlayerDestinationRequestedEvent {
                                    player_id,
                                    destination,
                                });
                            }
                        }
                    },
                }
            }
        }
    }
}

pub(super) fn handle_messages_from_server(
    mut client: ResMut<RenetClient>,
    mut lobby: ResMut<Lobby>,
    mut game_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    mut destinations_q: Query<(&Ownership, &mut Destination), With<Player>>,
) {
    for channel in FromServerChannel::iter() {
        while let Some(serialized_message) = client.receive_message(channel as u8) {
            let message = bincode::deserialize(&serialized_message);
            match message {
                Err(error) => {
                    error!("Couldn't deserialize a network message! Error: {}", error)
                }
                Ok(message) => {
                    match message {
                        MessageToClient::LobbySync(updated_lobby) => {
                            *lobby.as_mut() = updated_lobby;
                        }
                        MessageToClient::AssignedPlayerId(player_id) => {
                            info!("Got new Player Id! PlayerId: {}", player_id);
                            commands.insert_resource(CurrentPlayerId(player_id));
                        }
                        MessageToClient::MatchStarted => {
                            info!("Host has started a match!");
                            game_state.set(GameState::InGame);
                        }
                        MessageToClient::DestinationChanged {
                            player_id,
                            destination,
                        } => {
                            // Find player for requested PlayerId
                            if let Some((_, mut old_destination)) = destinations_q
                                .iter_mut()
                                .find(|(ownership, _)| ownership.0 == player_id)
                            {
                                info!("Destination changed for Player. PlayerId: {}, Destination: {:?}", player_id, destination);

                                old_destination.0 = destination.0;
                            }
                        }
                    }
                }
            }
        }
    }
}
