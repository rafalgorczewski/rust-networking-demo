use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::window::PrimaryWindow;

use crate::plugins::config::resources::{KeybindingsConfig, MainCameraConfig};

use super::{
    components::{MainCamera, MainCameraFocus},
    types::ScrollDirection,
};

pub(super) fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    commands.spawn((
        MainCamera,
        Camera3dBundle {
            projection: OrthographicProjection {
                scale: 15.0,
                scaling_mode: ScalingMode::FixedVertical(1.0),
                ..default()
            }
            .into(),
            transform: Transform::from_xyz(0.0, 6., 12.0)
                .looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
            ..default()
        },
    ));

    commands.spawn((
        MainCameraFocus,
        PbrBundle {
            mesh: meshes.add(
                shape::UVSphere {
                    radius: 0.1,
                    sectors: 10,
                    stacks: 10,
                }
                .into(),
            ),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
    ));
}

pub(super) fn orbit_camera_with_buttons(
    mut camera_transform_q: Query<&mut Transform, With<MainCamera>>,
    focus_transform_q: Query<&Transform, (With<MainCameraFocus>, Without<MainCamera>)>,
    main_camera_config: Res<MainCameraConfig>,
    keybindings_config: Res<KeybindingsConfig>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let focus_transform = focus_transform_q.single();
    let mut camera_transform = camera_transform_q.single_mut();

    if keyboard_input.pressed(keybindings_config.main_camera_orbit_left) {
        camera_transform.rotate_around(
            focus_transform.translation,
            Quat::from_rotation_y(main_camera_config.orbiting_speed * time.delta_seconds()),
        );
    } else if keyboard_input.pressed(keybindings_config.main_camera_orbit_right) {
        camera_transform.rotate_around(
            focus_transform.translation,
            Quat::from_rotation_y(-main_camera_config.orbiting_speed * time.delta_seconds()),
        );
    }
}

pub(super) fn scroll_camera(
    mut camera_transform_q: Query<&mut Transform, With<MainCamera>>,
    mut focus_transform_q: Query<&mut Transform, (With<MainCameraFocus>, Without<MainCamera>)>,
    window_q: Query<&Window, With<PrimaryWindow>>,
    keybindings_config: Res<KeybindingsConfig>,
    main_camera_config: Res<MainCameraConfig>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let mut scroll_direction = ScrollDirection::None;

    if keyboard_input.pressed(keybindings_config.main_camera_scroll_up) {
        scroll_direction = ScrollDirection::Up;
    } else if keyboard_input.pressed(keybindings_config.main_camera_scroll_down) {
        scroll_direction = ScrollDirection::Down;
    } else if keyboard_input.pressed(keybindings_config.main_camera_scroll_left) {
        scroll_direction = ScrollDirection::Left;
    } else if keyboard_input.pressed(keybindings_config.main_camera_scroll_right) {
        scroll_direction = ScrollDirection::Right;
    } else if main_camera_config.edge_scrolling_enabled {
        let camera_scroll_boundary_range = main_camera_config.scroll_boundary_range;

        let window = window_q.get_single().unwrap();

        if let Some(position) = window.cursor_position() {
            if position.x < camera_scroll_boundary_range {
                scroll_direction = ScrollDirection::Left;
            } else if position.x > window.width() - camera_scroll_boundary_range {
                scroll_direction = ScrollDirection::Right;
            } else if position.y < camera_scroll_boundary_range {
                scroll_direction = ScrollDirection::Down;
            } else if position.y > window.height() - camera_scroll_boundary_range {
                scroll_direction = ScrollDirection::Up;
            }
        }
    }

    let scroll_speed = main_camera_config.scroll_speed;
    let mut camera_transform = camera_transform_q.single_mut();
    let mut focus_transform = focus_transform_q.single_mut();
    let mut camera_transform_rotated = *camera_transform;
    camera_transform_rotated.translation.y = 0.0;
    camera_transform_rotated.look_at(focus_transform.translation, Vec3::Y);

    let translation = scroll_speed
        * match scroll_direction {
            ScrollDirection::None => Vec3::ZERO,
            ScrollDirection::Up => camera_transform_rotated.forward(),
            ScrollDirection::Down => camera_transform_rotated.back(),
            ScrollDirection::Left => camera_transform_rotated.left(),
            ScrollDirection::Right => camera_transform_rotated.right(),
        };

    camera_transform.translation += translation * time.delta_seconds();
    focus_transform.translation += translation * time.delta_seconds();
}
