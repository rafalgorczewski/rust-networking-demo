pub mod components;
mod systems;
mod types;

use bevy::prelude::*;

use crate::states::GameState;

use self::systems::{orbit_camera_with_buttons, scroll_camera, setup};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(GameState::InGame)));

        app.add_system(scroll_camera.in_set(OnUpdate(GameState::InGame)))
            .add_system(orbit_camera_with_buttons.in_set(OnUpdate(GameState::InGame)));
    }
}
