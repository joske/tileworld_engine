use bracket_lib::prelude::*;
use state::State;

mod agent;
mod astar;
mod grid;
mod hole;
mod location;
mod obstacle;
mod state;
mod tile;

const COLS: u16 = 40;
const ROWS: u16 = 40;
const NUM_AGENTS: u8 = 6;
const NUM_TILES: u8 = 20;
const NUM_HOLES: u8 = 20;
const NUM_OBSTACLES: u8 = 20;

fn main() -> BError {
    env_logger::init();
    let context = BTermBuilder::simple(COLS + 20, ROWS)?
        .with_title("Tileworld")
        .with_fps_cap(30.0)
        .build()?;
    let state = State::new();
    main_loop(context, state)
}
