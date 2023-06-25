pub mod constants;
pub mod events;
pub mod resources;

mod systems;

use bevy::prelude::*;

use crate::states::GameState;

use self::{
    constants::DEFAULT_CONNECT_ADDRESS,
    events::{ConnectRequestedEvent, GameStartRequestedEvent, HostRequestedEvent},
    resources::{ConnectWindowAddress, IsConnectWindowOpen},
    systems::{lobby_menu, main_menu},
};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<HostRequestedEvent>()
            .add_event::<ConnectRequestedEvent>()
            .add_event::<GameStartRequestedEvent>();

        app.insert_resource(IsConnectWindowOpen(false))
            .insert_resource(ConnectWindowAddress(DEFAULT_CONNECT_ADDRESS.into()));

        app.add_system(main_menu.in_set(OnUpdate(GameState::MainMenu)))
            .add_system(lobby_menu.in_set(OnUpdate(GameState::Lobby)));
    }
}
