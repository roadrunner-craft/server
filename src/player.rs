use core::world::WorldCoordinate;

pub type PlayerId = u128;

pub struct Player {
    pub id: PlayerId,
    pub position: WorldCoordinate,
}

impl Player {
    pub fn new(id: PlayerId) -> Self {
        Self {
            id,
            position: WorldCoordinate::default(),
        }
    }
}
