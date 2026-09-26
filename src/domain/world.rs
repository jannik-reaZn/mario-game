use crate::domain::{obstacle::Obstacle, player::Player};

pub struct World {
    player: Player,
    obstacles: Vec<Obstacle>,
}

impl World {
    pub fn new(player: Player, obstacles: Vec<Obstacle>) -> Self {
        Self { player, obstacles }
    }

    pub fn player(&self) -> &Player {
        &self.player
    }

    pub fn player_mut(&mut self) -> &mut Player {
        &mut self.player
    }

    pub fn obstacles(&self) -> &[Obstacle] {
        &self.obstacles
    }
}
