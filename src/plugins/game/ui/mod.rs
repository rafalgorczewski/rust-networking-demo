mod systems;

use bevy::prelude::*;

use crate::states::GameState;

use self::systems::developer_menu;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(developer_menu.in_set(OnUpdate(GameState::InGame)));
    }
}
