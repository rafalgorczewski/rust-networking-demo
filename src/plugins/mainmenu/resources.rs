use bevy::prelude::*;

#[derive(Resource)]
pub struct IsConnectWindowOpen(pub bool);

#[derive(Resource)]
pub struct ConnectWindowAddress(pub String);
