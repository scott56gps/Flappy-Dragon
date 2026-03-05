use bracket_lib::prelude::*;

mod constants;
mod obstacle;
mod player;
mod state;
use state::State;

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;

    main_loop(context, State::new())
}
