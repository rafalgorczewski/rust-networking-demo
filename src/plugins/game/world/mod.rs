pub mod types;

mod components;
mod systems;

use bevy::prelude::*;

use crate::states::GameState;

use self::systems::{setup, setup_players};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(GameState::InGame)))
            .add_system(setup_players.in_schedule(OnEnter(GameState::InGame)));
    }
}
