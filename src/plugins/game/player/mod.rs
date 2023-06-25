pub mod components;
pub mod events;

mod eventhandlers;
mod input;
mod systems;

use bevy::prelude::*;

use crate::states::{GameState, NetworkingState};

use self::{
    eventhandlers::on_player_destination_requested,
    events::PlayerDestinationRequestedEvent,
    input::on_move_click,
    systems::{setup, update_ground_cursor_position},
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerDestinationRequestedEvent>();

        app.add_system(setup.in_schedule(OnEnter(GameState::InGame)));

        app.add_system(on_move_click.in_set(OnUpdate(GameState::InGame)))
            .add_system(update_ground_cursor_position.in_set(OnUpdate(GameState::InGame)));

        app.add_system(
            on_player_destination_requested.in_set(OnUpdate(GameState::InGame)), //.run_if(in_state(NetworkingState::Connected)),
        );
    }
}
