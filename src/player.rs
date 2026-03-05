use bracket_lib::prelude::*;

pub struct Player {
    pub x: i32, // Position in *world-space*.  We don't let the user manipulate the x part.  Player is stuck to left-side of screen
    pub y: i32, // Position in *screen-space*
    pub velocity: f32, // Fractional floating-point to represent the velocity of the player *against* gravity.
}

impl Player {
    pub fn new(x: i32, y: i32) -> Self {
        Player {
            x,
            y,
            velocity: 0.0,
        }
    }

    pub fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(0, self.y, YELLOW, BLACK, to_cp437('@'));
    }

    pub fn gravity_and_move(&mut self) {
        if self.velocity < 1.8 {
            self.velocity += 0.4; // POSITIVE means down, because the TOP of the screen is 0
        }
        self.y += self.velocity as i32; // Here, we apply the new velocity to the actual position
        self.x += 1; // We always advance the player forward in *world-space*
        if self.y < 0 {
            self.y = 0; // If we're at the top, keep us at the top
        }
    }

    pub fn flap(&mut self) {
        self.velocity = -2.1; // Negative means up
    }
}
