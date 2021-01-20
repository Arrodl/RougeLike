use rltk::{Rltk, GameState};
use specs::prelude::*;
use std::cmp::{man, min, max};
use specs-derive::Component;

struct State {}

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

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
