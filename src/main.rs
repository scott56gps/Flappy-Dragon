use bracket_lib::prelude::*;

const SCREEN_WIDTH : i32 = 80;
const SCREEN_HEIGHT : i32 = 50;
const FRAME_DURATION : f32 = 75.0;

struct State {
    player: Player,
    frame_time: f32,  // I believe this means the current time inside of the current frame
    mode: GameMode,
}

impl State {
    fn new() -> Self {
        State {
            player: Player::new(5, 25),
            frame_time: 0.0,
            mode: GameMode::Menu,
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;  // Increment the current time inside of the current frame
        if self.frame_time > FRAME_DURATION {  // NOW that the frame is complete, run the actual simulation.  Computer is faster than human.
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }
        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }
        self.player.render(ctx);
        ctx.print(0, 0, "Press SPACE to flap.");
        if self.player.y > SCREEN_HEIGHT {    // We've hit the bottom (top of screen is 0)
            self.mode = GameMode::End;
        }
    }

    fn restart(&mut self) {
        self.player = Player::new(5, 25);
        self.frame_time = 0.0;
        self.mode = GameMode::Playing;
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_color_centered(5, RGBA { r: 0.44, g: 0.26, b: 0.78, a: 1.0 }, RGBA { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }, "Welcome to Flappy Dragon!");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");

        // Read input, if any
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_color_centered(5, RGBA { r: 1.0, g: 0.0, b: 0.0, a: 1.0 }, RGBA { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }, "You're dead!");
        ctx.print_centered(8, "(P) Play Again");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
}

impl GameState for State {
    /* This function modifies the previous state for the purpose of pressing forward to a new state */
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::End => self.dead(ctx),
            GameMode::Playing => self.play(ctx),
        }
    }
}

struct Player {
    x: i32, // Position in *world-space*.  We don't let the user manipulate the x part.  Player is stuck to left-side of screen
    y: i32, // Position in *screen-space*
    velocity: f32, // Fractional floating-point to represent the velocity of the player *against* gravity.
}

impl Player {
    fn new(x: i32, y: i32) -> Self {
        Player {
            x, y, velocity: 0.0,
        }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(0, self.y, YELLOW, BLACK, to_cp437('@'));
    }

    fn gravity_and_move(&mut self) {
        if self.velocity < 2.0 {
            self.velocity += 0.2;        // POSITIVE means down, because the TOP of the screen is 0
        }
        self.y += self.velocity as i32;  // Here, we apply the new velocity to the actual position
        self.x += 1;                     // We always advance the player forward in *world-space*
        if self.y < 0 {
            self.y = 0;                  // If we're at the top, keep us at the top
        }
    }

    fn flap(&mut self) {
        self.velocity = -2.0;            // Negative means up
    }
}

enum GameMode {
    Menu,
    Playing,
    End,
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;

    main_loop(context, State::new())
}
