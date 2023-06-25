use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::plugins::{
    game::{
        player::components::{Ownership, Player},
        unit::components::{Destination, MovementSpeed},
    },
    lobby::resources::Lobby,
};

use super::{components::Ground, types::CollisionGroup};

pub(super) fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 9000.0,
            range: 100.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 16.0, 8.0),
        ..default()
    });

    // Spawn ground
    commands.spawn((
        Ground,
        PbrBundle {
            mesh: meshes.add(
                shape::Plane {
                    size: 50.,
                    subdivisions: 10,
                }
                .into(),
            ),
            material: materials.add(Color::SILVER.into()),
            ..default()
        },
        Collider::cuboid(50.0, 0.1, 50.0),
        CollisionGroups::new(
            Group::from_bits(CollisionGroup::Ground as u32).unwrap(),
            Default::default(),
        ),
    ));
}

pub(super) fn setup_players(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    lobby: Res<Lobby>,
) {
    // Spawn all players in lobby
    for player_id in lobby.get_players() {
        // Spawn player
        commands.spawn((
            Player,
            Ownership(player_id),
            Destination(None),
            PbrBundle {
                mesh: meshes.add(
                    shape::UVSphere {
                        radius: 0.5,
                        sectors: 20,
                        stacks: 20,
                    }
                    .into(),
                ),
                material: materials.add(Color::LIME_GREEN.into()),
                transform: Transform::from_xyz(0.0, 0.1, 0.0),
                ..default()
            },
            MovementSpeed(10.0),
        ));
    }
}
