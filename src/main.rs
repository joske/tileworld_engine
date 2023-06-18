use std::cell::RefCell;
use std::rc::Rc;

use crate::hole::Hole;
use crate::tile::Tile;
use agent::Agent;
use bracket_lib::prelude::*;
use obstacle::Obstacle;
use rand::Rng;

mod agent;
mod astar;
mod grid;
mod hole;
mod location;
mod obstacle;
mod tile;

const COLS: u16 = 80;
const ROWS: u16 = 50;
const NUM_AGENTS: u8 = 6;
const NUM_TILES: u8 = 20;
const NUM_HOLES: u8 = 20;
const NUM_OBSTACLES: u8 = 20;

struct State {
    frame_time: f32,
    grid: Rc<RefCell<grid::Grid>>,
    agents: Vec<Rc<RefCell<Agent>>>,
    tiles: Vec<Rc<RefCell<Tile>>>,
    holes: Vec<Rc<RefCell<Hole>>>,
    obstacles: Vec<Obstacle>,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > 1000.0 / 3.0 {
            self.frame_time = 0.0;
            self.update(ctx);
        }
    }
}

impl State {
    fn render(&self, ctx: &mut BTerm) {
        ctx.cls_bg(WHITE);
        for agent in self.agents.iter() {
            let agent = agent.borrow();
            agent.render(ctx);
        }
        for tile in self.tiles.iter() {
            let tile = tile.borrow();
            tile.render(ctx);
        }
        for hole in self.holes.iter() {
            let hole = hole.borrow();
            hole.render(ctx);
        }
        for obstacle in self.obstacles.iter() {
            obstacle.render(ctx);
        }
    }

    fn update(&mut self, ctx: &mut BTerm) {
        for agent in self.agents.iter() {
            let mut agent = agent.borrow_mut();
            agent.update(self);
        }
        self.render(ctx);
    }
}

fn main() -> BError {
    env_logger::init();
    let context = BTermBuilder::simple80x50()
        .with_title("Tileworld")
        .with_fps_cap(30.0)
        .build()?;
    let mut rng = rand::thread_rng();
    let mut grid = grid::Grid::new();
    let mut agents = Vec::new();
    for i in 0..NUM_AGENTS as u8 {
        let location = grid.random_location();
        grid.set(location);
        let a = Agent::new(i, location);
        agents.push(Rc::new(RefCell::new(a)));
    }
    let mut tiles = Vec::new();
    for _ in 0..NUM_TILES as u8 {
        let location = grid.random_location();
        grid.set(location);
        let score = rng.gen_range(1..5);
        let a = Tile::new(location, score);
        tiles.push(Rc::new(RefCell::new(a)));
    }
    let mut holes = Vec::new();
    for _ in 0..NUM_HOLES as u8 {
        let location = grid.random_location();
        grid.set(location);
        let a = Hole::new(location);
        holes.push(Rc::new(RefCell::new(a)));
    }
    let mut obstacles: Vec<Obstacle> = Vec::new();
    for _ in 0..NUM_OBSTACLES as u8 {
        let location = grid.random_location();
        grid.set(location);
        let a = Obstacle::new(location);
        obstacles.push(a);
    }
    let state = State {
        frame_time: 0.0,
        grid: Rc::new(RefCell::new(grid)),
        agents,
        tiles,
        holes,
        obstacles,
    };
    main_loop(context, state)
}
