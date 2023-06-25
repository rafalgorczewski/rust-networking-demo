use bevy::prelude::*;

/// Ground on which the entities move.
/// Used for casting ray from the camera to determine where the unit move destination should be set.
#[derive(Component)]
pub struct Ground;
