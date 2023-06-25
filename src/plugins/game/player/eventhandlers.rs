use bevy::prelude::*;

use crate::{
    plugins::{
        game::unit::components::Destination,
        networking::{
            events::{SendMessageToClientEvent, SendMessageToServerEvent},
            messages::{MessageToClient, MessageToServer},
        },
    },
    states::NetworkingState,
};

use super::{
    components::{Ownership, Player},
    events::PlayerDestinationRequestedEvent,
};

pub(super) fn on_player_destination_requested(
    mut player_destination_er: EventReader<PlayerDestinationRequestedEvent>,
    mut destinations_q: Query<(&Ownership, &mut Destination), With<Player>>,
    mut send_message_to_server_ew: EventWriter<SendMessageToServerEvent>,
    mut send_message_to_client_ew: EventWriter<SendMessageToClientEvent>,
    networking_state: Res<State<NetworkingState>>,
) {
    for event in player_destination_er.iter() {
        // Find player for requested PlayerId
        if let Some((_, mut old_destination)) = destinations_q
            .iter_mut()
            .find(|(ownership, _)| ownership.0 == event.player_id)
        {
            *old_destination = event.destination.clone();
        }

        match networking_state.0 {
            NetworkingState::Connected => send_message_to_server_ew.send(SendMessageToServerEvent(
                MessageToServer::DestinationRequested(event.destination.clone()),
            )),
            NetworkingState::Hosting => send_message_to_client_ew.send(SendMessageToClientEvent(
                MessageToClient::DestinationChanged {
                    player_id: event.player_id,
                    destination: event.destination.clone(),
                },
            )),
            _ => panic!(),
        }
    }
}
