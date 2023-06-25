pub mod camera;
pub mod player;
pub mod ui;
pub mod unit;
pub mod world;

use bevy::prelude::*;

use self::{
    camera::CameraPlugin, player::PlayerPlugin, ui::UiPlugin, unit::UnitPlugin, world::WorldPlugin,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(UiPlugin)
            .add_plugin(CameraPlugin)
            .add_plugin(WorldPlugin)
            .add_plugin(UnitPlugin)
            .add_plugin(PlayerPlugin);
    }
}
