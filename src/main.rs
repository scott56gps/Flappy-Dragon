use bracket_lib::prelude::*;

struct State {}

impl GameState for State {
    /* This function modifies the previous state for the purpose of pressing forward to a new state */
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(1, 1, "FFIX!!!");
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;

    let gs: State = State {};
    main_loop(context, gs)
}
