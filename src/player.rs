use core::world::WorldCoordinate;
use std::time::Instant;

pub type PlayerId = u128;

pub struct Player {
    id: PlayerId,
    position: WorldCoordinate,
    last_update: Instant,
}

impl Player {
    pub fn new(id: PlayerId) -> Self {
        Self {
            id,
            position: WorldCoordinate::default(),
            last_update: Instant::now(),
        }
    }

    pub fn id(&self) -> u128 {
        self.id
    }

    pub fn position(&self) -> WorldCoordinate {
        self.position
    }

    pub fn set_position(&mut self, position: WorldCoordinate) {
        self.position = position;
        self.last_update = Instant::now();
    }

    pub fn last_update(&self) -> Instant {
        self.last_update
    }
}
