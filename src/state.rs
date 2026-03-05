use bracket_lib::prelude::*;

use crate::constants::{SCREEN_HEIGHT, FRAME_DURATION};
use crate::player::Player;

pub struct State {
    pub player: Player,
    pub frame_time: f32,  // I believe this means the current time inside of the current frame
    pub mode: GameMode,
}

impl State {
    pub fn new() -> Self {
        State {
            player: Player::new(5, 25),
            frame_time: 0.0,
            mode: GameMode::Menu,
        }
    }

    pub fn play(&mut self, ctx: &mut BTerm) {
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

    pub fn restart(&mut self) {
        self.player = Player::new(5, 25);
        self.frame_time = 0.0;
        self.mode = GameMode::Playing;
    }

    pub fn main_menu(&mut self, ctx: &mut BTerm) {
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

    pub fn dead(&mut self, ctx: &mut BTerm) {
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


pub enum GameMode {
    Menu,
    Playing,
    End,
}
