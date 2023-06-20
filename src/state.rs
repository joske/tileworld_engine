use bracket_lib::terminal::*;

use crate::agent::Agent;
use crate::grid::Grid;
use crate::hole::Hole;
use crate::obstacle::Obstacle;
use crate::tile::Tile;
use crate::{NUM_AGENTS, NUM_HOLES, NUM_OBSTACLES, NUM_TILES};
use rand::Rng;
use std::cell::RefCell;
use std::rc::Rc;

pub(crate) struct State {
    frame_time: f32,
    pub grid: Rc<RefCell<Grid>>,
    pub agents: Vec<Rc<RefCell<Agent>>>,
    pub tiles: Vec<Rc<RefCell<Tile>>>,
    pub holes: Vec<Rc<RefCell<Hole>>>,
    obstacles: Vec<Obstacle>,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > 1000.0 / 5.0 {
            self.frame_time = 0.0;
            self.update(ctx);
        }
    }
}

impl State {
    pub(crate) fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut grid = Grid::new();
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
        State {
            frame_time: 0.0,
            grid: Rc::new(RefCell::new(grid)),
            agents,
            tiles,
            holes,
            obstacles,
        }
    }

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
