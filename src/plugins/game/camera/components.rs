use bevy::prelude::*;

/// Main camera used for gameplay view and control.
#[derive(Component)]
pub struct MainCamera;

/// Focus point used for camera to rotate around.
#[derive(Component)]
pub struct MainCameraFocus;
