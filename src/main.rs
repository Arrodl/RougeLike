use rltk::{Rltk, GameState};

struct State {}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
        ctx.print(1, 1, "Hello Rust World")
    }
}

fn main() -> rltk::BError {
    let context = rltk::RltkBuilder::simple80x50()
        .with_title("Rougelike")
        .build()?;
    let gs = State { };
    rltk::main_loop(context, gs)
}