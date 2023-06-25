use bevy::prelude::*;

use crate::plugins::{game::unit::components::Destination, lobby::resources::CurrentPlayerId};

use super::{
    components::{GroundCursor, PlayerMovementTarget},
    events::PlayerDestinationRequestedEvent,
};

pub(super) fn on_move_click(
    mouse_input: Res<Input<MouseButton>>,
    mut movement_target_q: Query<(&mut Transform, &mut Visibility), With<PlayerMovementTarget>>,
    mut player_destination_changed_ew: EventWriter<PlayerDestinationRequestedEvent>,
    ground_cursor_q: Query<&GroundCursor>,
    current_player_id: Res<CurrentPlayerId>,
) {
    if mouse_input.just_pressed(MouseButton::Right) {
        if let Some(ground_cursor) = ground_cursor_q.single().0 {
            let (mut movement_target, mut visibility) = movement_target_q.single_mut();

            *visibility = Visibility::Visible;
            movement_target.translation = ground_cursor;

            player_destination_changed_ew.send(PlayerDestinationRequestedEvent {
                player_id: current_player_id.0,
                destination: Destination(Some(ground_cursor)),
            });
        }
    }
}
