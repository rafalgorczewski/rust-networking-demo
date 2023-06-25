pub mod constants;
pub mod resources;
pub mod types;

use bevy::prelude::*;

use self::resources::Lobby;

pub struct LobbyPlugin;

impl Plugin for LobbyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Lobby>();
    }
}
