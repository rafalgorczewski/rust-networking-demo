use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Destination to which the unit is heading.
/// Can be None, in which case the unit is standing still.
#[derive(Component, Deserialize, Serialize, Clone, Debug)]
pub struct Destination(pub Option<Vec3>);

/// Movement speed of units.
#[derive(Component)]
pub struct MovementSpeed(pub f32);
