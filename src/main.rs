use bracket_lib::prelude::*;

struct State {
    mode: GameMode,
}

impl State {
    fn new() -> Self {
        State {
            mode: GameMode::Menu,
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        // TODO: Fill in with actual logic
        self.mode = GameMode::End;
    }

    fn restart(&mut self) {
        self.mode = GameMode::Playing;
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        // TODO: Fill in with actual logic
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
