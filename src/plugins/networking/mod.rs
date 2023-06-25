pub mod constants;
pub mod events;
pub mod messages;
pub mod resources;
pub mod types;
pub mod utils;

mod eventhandlers;
mod systems;

use bevy::prelude::*;
use bevy::time::Stopwatch;

use crate::states::NetworkingState;

use self::constants::CONNECTION_RETRY_INTERVAL;
use self::eventhandlers::{
    on_connect_requested, on_game_started, on_host_requested, on_send_message_to_client,
    on_send_message_to_server, on_server_event,
};
use self::events::{SendMessageToClientEvent, SendMessageToServerEvent};
use self::resources::ConnectionRetryTimer;
use self::systems::{check_connection, handle_messages_from_client, handle_messages_from_server};

pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ConnectionRetryTimer {
            timer: Timer::new(CONNECTION_RETRY_INTERVAL, TimerMode::Repeating),
            stopwatch: Stopwatch::new(),
        });

        app.add_event::<SendMessageToClientEvent>()
            .add_event::<SendMessageToServerEvent>();

        app.add_system(on_host_requested.in_set(OnUpdate(NetworkingState::Disconnected)))
            .add_system(on_connect_requested.in_set(OnUpdate(NetworkingState::Disconnected)));

        app.add_system(check_connection.in_set(OnUpdate(NetworkingState::Connecting)));

        app.add_system(handle_messages_from_server.in_set(OnUpdate(NetworkingState::Connected)))
            .add_system(on_send_message_to_server.in_set(OnUpdate(NetworkingState::Connected)));

        app.add_system(handle_messages_from_client.in_set(OnUpdate(NetworkingState::Hosting)))
            .add_system(on_send_message_to_client.in_set(OnUpdate(NetworkingState::Hosting)))
            .add_system(on_server_event.in_set(OnUpdate(NetworkingState::Hosting)))
            .add_system(on_game_started.in_set(OnUpdate(NetworkingState::Hosting)));
    }
}
