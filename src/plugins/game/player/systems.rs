use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier3d::prelude::*;

use crate::plugins::game::{camera::components::MainCamera, world::types::CollisionGroup};

use super::components::{GroundCursor, PlayerMovementTarget};

/// Max distance from camera to the ground to calculate mouse clicks.
const MAX_TOI: f32 = 90.0;

pub(super) fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    // Spawn player movement target
    commands.spawn((
        PlayerMovementTarget,
        PbrBundle {
            mesh: meshes.add(
                shape::UVSphere {
                    radius: 0.1,
                    sectors: 10,
                    stacks: 10,
                }
                .into(),
            ),
            visibility: Visibility::Hidden,
            ..default()
        },
    ));

    // Spawn ground cursor
    commands.spawn(GroundCursor(None));
}

pub(super) fn update_ground_cursor_position(
    window_q: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut ground_cursor_q: Query<&mut GroundCursor>,
    rapier_context: Res<RapierContext>,
) {
    let (camera, camera_transform) = camera_q.single();
    let window = window_q.get_single().unwrap();
    if let Some(cursor_position) = window.cursor_position() {
        let ray = camera.viewport_to_world(camera_transform, cursor_position);
        if ray.is_none() {
            return;
        }

        let ray = ray.unwrap();
        let max_toi = MAX_TOI;
        let solid = true;
        let filter = QueryFilter::new().groups(CollisionGroups::new(
            Group::from_bits(CollisionGroup::Ground as u32).unwrap(),
            Default::default(),
        ));

        let mut ground_cursor = ground_cursor_q.single_mut();
        if let Some((_, toi)) =
            rapier_context.cast_ray(ray.origin, ray.direction, max_toi, solid, filter)
        {
            ground_cursor.0 = Some(ray.origin + ray.direction * toi);
        } else {
            ground_cursor.0 = None;
        }
    }
}
