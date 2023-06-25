use bevy::prelude::*;

#[derive(Resource)]
pub struct KeybindingsConfig {
    pub main_camera_scroll_button: MouseButton,
    pub main_camera_orbit_left: KeyCode,
    pub main_camera_orbit_right: KeyCode,
    pub main_camera_scroll_up: KeyCode,
    pub main_camera_scroll_down: KeyCode,
    pub main_camera_scroll_left: KeyCode,
    pub main_camera_scroll_right: KeyCode,
}

impl Default for KeybindingsConfig {
    fn default() -> Self {
        Self {
            main_camera_scroll_button: MouseButton::Middle,
            main_camera_orbit_left: KeyCode::Q,
            main_camera_orbit_right: KeyCode::E,
            main_camera_scroll_up: KeyCode::W,
            main_camera_scroll_down: KeyCode::S,
            main_camera_scroll_left: KeyCode::A,
            main_camera_scroll_right: KeyCode::D,
        }
    }
}

#[derive(Resource)]
pub struct MainCameraConfig {
    /// Camera scroll speed.
    pub scroll_speed: f32,
    /// Distance from screen edge at which camera starts scrolling.
    pub scroll_boundary_range: f32,
    /// Speed of camera orbiting.
    pub orbiting_speed: f32,
    /// Edge scrolling enabled.
    pub edge_scrolling_enabled: bool,
}

impl Default for MainCameraConfig {
    fn default() -> Self {
        Self {
            scroll_speed: 17.5,
            scroll_boundary_range: 100.0,
            orbiting_speed: 2.0,
            edge_scrolling_enabled: false,
        }
    }
}
