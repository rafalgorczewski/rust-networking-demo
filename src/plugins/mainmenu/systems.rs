use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use std::net::SocketAddrV4;

use crate::{
    plugins::{lobby::resources::Lobby, mainmenu::constants::DEFAULT_CONNECT_ADDRESS},
    states::{GameState, NetworkingState},
};

use super::{
    events::{ConnectRequestedEvent, GameStartRequestedEvent, HostRequestedEvent},
    resources::{ConnectWindowAddress, IsConnectWindowOpen},
};

pub(super) fn main_menu(
    mut egui_contexts: EguiContexts,
    mut is_connect_window_open: ResMut<IsConnectWindowOpen>,
    mut connect_window_address: ResMut<ConnectWindowAddress>,
    mut host_ew: EventWriter<HostRequestedEvent>,
    mut connect_ew: EventWriter<ConnectRequestedEvent>,
    mut exit_ew: EventWriter<AppExit>,
) {
    egui::CentralPanel::default().show(egui_contexts.ctx_mut(), |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(128.0);
            ui.heading("Networking demo");
            ui.add_space(64.0);
            if ui.button("Host").clicked() {
                info!("Host button clicked.");

                host_ew.send(HostRequestedEvent);
            }
            if ui.button("Connect").clicked() {
                info!("Connect button clicked.");

                connect_window_address.0 = DEFAULT_CONNECT_ADDRESS.into();
                is_connect_window_open.0 = true;
            }
            if ui.button("Exit").clicked() {
                info!("Exit button clicked.");

                exit_ew.send(AppExit);
            }
        })
    });

    let mut connect_window_should_close = false;
    egui::Window::new("Connect")
        .open(&mut is_connect_window_open.0)
        .show(egui_contexts.ctx_mut(), |ui| {
            ui.label("Address:");
            if ui
                .text_edit_singleline(&mut connect_window_address.0)
                .lost_focus()
                && ui.input(|i| i.key_pressed(egui::Key::Enter))
            {
                match SocketAddrV4::parse_ascii(
                    connect_window_address.0.clone().into_bytes().as_slice(),
                ) {
                    Ok(address) => {
                        info!("Correct IP in Connect edit.");

                        connect_window_should_close = true;
                        connect_ew.send(ConnectRequestedEvent(address));
                    }
                    Err(_) => {
                        info!("Wrong IP in Connect edit, doing nothing.")
                    }
                }
            }
        });
    if connect_window_should_close {
        is_connect_window_open.0 = false;
    }
}

pub(super) fn lobby_menu(
    mut egui_contexts: EguiContexts,
    networking_state: Res<State<NetworkingState>>,
    lobby: Res<Lobby>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut game_started_ew: EventWriter<GameStartRequestedEvent>,
) {
    egui::Window::new("Lobby").show(egui_contexts.ctx_mut(), |ui| {
        for player_id in lobby.get_players() {
            if let Some(client_id) = lobby.get_client_id(player_id) {
                if let Some(client_info) = lobby.get_client_info(client_id) {
                    ui.label(format!(
                        "Name: {}, ClientId: {}, PlayerId: {}",
                        client_info.nickname, client_id, player_id
                    ));
                } else {
                    ui.label(format!(
                        "Unknown player, ClientId: {}, PlayerId: {}",
                        client_id, player_id
                    ));
                }
            }
        }
        if networking_state.0 == NetworkingState::Hosting {
            if ui.button("Start").clicked() {
                info!("Start button clicked.");

                game_started_ew.send(GameStartRequestedEvent);
                next_game_state.set(GameState::InGame);
            }
        }
    });
}
