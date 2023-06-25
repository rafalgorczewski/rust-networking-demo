use bevy::prelude::*;
use bevy::time::Stopwatch;

#[derive(Resource)]
pub struct ConnectionRetryTimer {
    pub timer: Timer,
    pub stopwatch: Stopwatch,
}
