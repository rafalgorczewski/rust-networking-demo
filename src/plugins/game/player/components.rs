use bevy::prelude::*;

use crate::plugins::lobby::types::PlayerId;

/// Main unit controlled by player.
#[derive(Component)]
pub struct Player;

/// Component used to describe which player the entity belongs to.
#[derive(Component)]
pub struct Ownership(pub PlayerId);

/// Point used to mark the spot that the Player is moving to.
#[derive(Component)]
pub struct PlayerMovementTarget;

/// Point on the ground which is projected from the on-screen camera cursor.
#[derive(Component)]
pub struct GroundCursor(pub Option<Vec3>);

/// Is entity translation updating to be equal with the ground cursor.
/// Used for e.g. building prototype movement.
#[derive(Component)]
pub struct FollowingGroundCursor(pub bool);
