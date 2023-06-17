use crate::hole::Hole;
use crate::tile::Tile;
use agent::Agent;
use bracket_lib::prelude::*;
use obstacle::Obstacle;

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
    grid: grid::Grid,
    agents: Vec<Agent>,
    tiles: Vec<Tile>,
    holes: Vec<Hole>,
    obstacles: Vec<Obstacle>,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        self.update(ctx);
    }
}

impl State {
    fn render(&self, ctx: &mut BTerm) {
        ctx.cls_bg(WHITE);
        for agent in self.agents.iter() {
            agent.render(ctx);
        }
        for tile in self.tiles.iter() {
            tile.render(ctx);
        }
        for hole in self.holes.iter() {
            hole.render(ctx);
        }
        for obstacle in self.obstacles.iter() {
            obstacle.render(ctx);
        }
    }

    fn update(&mut self, ctx: &mut BTerm) {
        for agent in self.agents.iter_mut() {
            // agent.update(self);
        }
        self.render(ctx);
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Tileworld")
        .with_fps_cap(30.0)
        .build()?;
    let mut rng = RandomNumberGenerator::new();
    let mut grid = grid::Grid::new();
    let mut agents: Vec<Agent> = Vec::new();
    for i in 0..NUM_AGENTS as u8 {
        let location = grid.random_location(&mut rng);
        grid.set(location);
        let a = Agent::new(i, location);
        agents.push(a);
    }
    let mut tiles: Vec<Tile> = Vec::new();
    for _ in 0..NUM_TILES as u8 {
        let location = grid.random_location(&mut rng);
        grid.set(location);
        let score = rng.range(1, 5);
        let a = Tile::new(location, score);
        tiles.push(a);
    }
    let mut holes: Vec<Hole> = Vec::new();
    for _ in 0..NUM_HOLES as u8 {
        let location = grid.random_location(&mut rng);
        grid.set(location);
        let a = Hole::new(location);
        holes.push(a);
    }
    let mut obstacles: Vec<Obstacle> = Vec::new();
    for _ in 0..NUM_OBSTACLES as u8 {
        let location = grid.random_location(&mut rng);
        grid.set(location);
        let a = Obstacle::new(location);
        obstacles.push(a);
    }
    let state = State {
        grid,
        agents,
        tiles,
        holes,
        obstacles,
    };
    main_loop(context, state)
}
