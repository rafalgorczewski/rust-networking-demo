pub mod resources;

use bevy::prelude::*;

use self::resources::{KeybindingsConfig, MainCameraConfig};

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MainCameraConfig>()
            .init_resource::<KeybindingsConfig>();
    }
}
