use bevy::prelude::*;

use super::components::{Destination, MovementSpeed};

pub(super) fn handle_movement(
    mut destinations_q: Query<(&mut Transform, &mut Destination, &MovementSpeed)>,
    time: Res<Time>,
) {
    for (mut transform, mut destination, movement_speed) in destinations_q.iter_mut() {
        if destination.0.is_some() {
            let old_translation = transform.translation;
            let direction = (destination.0.unwrap() - old_translation).normalize();

            if transform.translation.abs_diff_eq(
                destination.0.unwrap(),
                movement_speed.0 * time.delta_seconds(),
            ) {
                transform.translation = destination.0.unwrap();
                destination.0 = None;
            } else {
                let new_translation =
                    old_translation + (direction * movement_speed.0 * time.delta_seconds());
                transform.translation = new_translation;
            }
        }
    }
}
