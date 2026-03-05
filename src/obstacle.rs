use bracket_lib::prelude::*;

use crate::constants::SCREEN_HEIGHT;
use crate::player::Player;

pub struct Obstacle {
    pub x: i32,      // Position in *world-space*
    pub gap_y: i32,  // Position of the center of the gap
    pub size: i32,   // The length of the gap
}

impl Obstacle {
    pub fn new(x: i32, score: i32) -> Self {
        let mut random = RandomNumberGenerator::new();
        Obstacle {
            x,
            gap_y: random.range(10, 40),
            size: i32::max(2, 20 - score)
        }
    }

    /**
     * Calculate the top and bottom bounds of the gap.
     * The left element is the lower bound and the right element is the upper bound.
     */
    fn get_bounds(&self) -> (i32, i32) {
        let half_size = self.size / 2;
        (self.gap_y + half_size, self.gap_y - half_size)
    }

    pub fn render(&mut self, ctx: &mut BTerm, player_x: i32) {
        let screen_x = self.x - player_x;   // This obstacle's x coordinate in screen space
        let (lower_bound, upper_bound) = self.get_bounds();

        // Render the top half of the obstacle
        for y in 0..upper_bound {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }

        // Render the bottom half of the obstacle
        for y in lower_bound..SCREEN_HEIGHT {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }
    }

    pub fn is_hit(&self, player: &Player) -> bool {
        // Only try to detect a hit if the player and the obstacle have x values that have met
        if player.x == self.x {
            let (lower_bound, upper_bound) = self.get_bounds();
            player.y < upper_bound || player.y > lower_bound
        } else {
            false
        }
    }
}
