use crate::plugins::{game::unit::components::Destination, lobby::types::PlayerId};

pub struct PlayerDestinationRequestedEvent {
    pub player_id: PlayerId,
    pub destination: Destination,
}
