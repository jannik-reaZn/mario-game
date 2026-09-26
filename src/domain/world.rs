use crate::domain::{obstacle::Obstacle, player::Player};

pub struct World {
    player: Player,
    obstacles: Vec<Obstacle>,
    bottom: f32,
}

impl World {
    pub fn new(player: Player, obstacles: Vec<Obstacle>, bottom: f32) -> Self {
        Self {
            player,
            obstacles,
            bottom,
        }
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

    pub fn bottom(&self) -> f32 {
        self.bottom
    }
}
