pub mod components;

mod systems;

use bevy::prelude::*;

use crate::states::GameState;

use self::systems::handle_movement;

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_movement.in_set(OnUpdate(GameState::InGame)));
    }
}
