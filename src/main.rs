#![feature(addr_parse_ascii)]

mod plugins;
mod states;

use bevy::prelude::{App, DefaultPlugins};
use bevy_egui::EguiPlugin;
use bevy_rapier3d::prelude::{NoUserData, RapierPhysicsPlugin};
use bevy_renet::{RenetClientPlugin, RenetServerPlugin};

use plugins::{
    config::ConfigPlugin, game::GamePlugin, lobby::LobbyPlugin, mainmenu::MainMenuPlugin,
    networking::NetworkingPlugin,
};
use states::{GameState, NetworkingState};

fn main() {
    let mut app = App::new();

    // External plugins
    app.add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RenetServerPlugin::default())
        .add_plugin(RenetClientPlugin::default());

    // Internal states
    app.add_state::<GameState>().add_state::<NetworkingState>();

    // Internal plugins
    app.add_plugin(MainMenuPlugin)
        .add_plugin(NetworkingPlugin)
        .add_plugin(LobbyPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(ConfigPlugin);

    app.run();
}
