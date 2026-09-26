use crate::domain::obstacle::Obstacle;
use crate::domain::player::Player;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CollisionSide {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Collision {
    side: CollisionSide,
}

impl Collision {
    pub fn new(side: CollisionSide) -> Self {
        Self { side }
    }

    pub fn side(&self) -> CollisionSide {
        self.side
    }
}

pub fn detect(player: &Player, obstacle: &Obstacle) -> Option<Collision> {
    let p = player.bounds();
    let o = obstacle.bounds();

    // Broad answer first: no overlap on both axes, no collision.
    if !p.intersects(&o) {
        return None;
    }

    // How deep the player has sunk into each face of the obstacle.
    // The face with the smallest penetration is the one we entered through.
    let into_top = p.bottom() - o.top(); // player came from above
    let into_bottom = o.bottom() - p.top(); // player came from below
    let into_left = p.right() - o.left(); // player came from the left
    let into_right = o.right() - p.left(); // player came from the right

    let min = into_top.min(into_bottom).min(into_left).min(into_right);

    let side = if min == into_top {
        CollisionSide::Top
    } else if min == into_bottom {
        CollisionSide::Bottom
    } else if min == into_left {
        CollisionSide::Left
    } else {
        CollisionSide::Right
    };

    Some(Collision::new(side))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::color::Color;
    use crate::domain::position::Position;
    use crate::domain::size::Size;
    use crate::domain::state::State;
    use crate::domain::velocity::Velocity;

    // Obstacle spans x 100..200, y 100..150
    fn obstacle() -> Obstacle {
        Obstacle::new(
            Position::new(100.0, 100.0).unwrap(),
            Size::new(100.0, 50.0).unwrap(),
        )
    }

    // 20x20 player with its top-left corner at (x, y)
    fn player_at(x: f32, y: f32) -> Player {
        Player {
            size: Size::new(20.0, 20.0).unwrap(),
            velocity: Velocity::stationary(),
            position: Position::new(x, y).unwrap(),
            state: State::Grounded,
            color: Color::Blue,
        }
    }

    #[test]
    fn no_collision_when_far_away() {
        assert_eq!(detect(&player_at(0.0, 0.0), &obstacle()), None);
    }

    #[test]
    fn no_collision_when_only_touching_edge() {
        // player bottom == obstacle top
        assert_eq!(detect(&player_at(120.0, 80.0), &obstacle()), None);
    }

    #[test]
    fn landing_on_top() {
        let collision = detect(&player_at(120.0, 85.0), &obstacle()).unwrap();
        assert_eq!(collision.side(), CollisionSide::Top);
    }

    #[test]
    fn hitting_head_from_below() {
        let collision = detect(&player_at(120.0, 145.0), &obstacle()).unwrap();
        assert_eq!(collision.side(), CollisionSide::Bottom);
    }

    #[test]
    fn hitting_from_the_left() {
        let collision = detect(&player_at(85.0, 115.0), &obstacle()).unwrap();
        assert_eq!(collision.side(), CollisionSide::Left);
    }

    #[test]
    fn hitting_from_the_right() {
        let collision = detect(&player_at(195.0, 115.0), &obstacle()).unwrap();
        assert_eq!(collision.side(), CollisionSide::Right);
    }

    #[test]
    fn corner_hit_picks_side_with_smallest_penetration() {
        // sinks 2 into the top face but 15 into the left face
        let collision = detect(&player_at(95.0, 82.0), &obstacle()).unwrap();
        assert_eq!(collision.side(), CollisionSide::Top);
    }
}
