use crate::{
    astar::astar,
    grid::Grid,
    location::{closest, Located, Location},
    State, COLS,
};
use bracket_lib::prelude::*;
use log::{debug, warn};
use rand::Rng;

pub(crate) struct Agent {
    id: u8,
    pub(crate) location: Location,
    pub(crate) state: AgentState,
    score: u32,
    tile_score: Option<u8>,
}

#[derive(Debug)]
pub(crate) enum AgentState {
    MoveToTile,
    MoveToHole,
}

impl Agent {
    pub(crate) fn new(id: u8, location: Location) -> Self {
        Self {
            id,
            location,
            state: AgentState::MoveToTile,
            score: 0,
            tile_score: None,
        }
    }

    pub(crate) fn update(&mut self, state: &State) {
        debug!(
            "Agent {}: Location: {:?} state: {:?}",
            self.id, self.location, self.state
        );
        let mut grid = state.grid.borrow_mut();
        match self.state {
            AgentState::MoveToTile => {
                if let Some(closest) = closest(self.location, &state.tiles) {
                    debug!("Agent {}: Closest tile: {:?}", self.id, closest.borrow());
                    let arrived = self.move_to(&mut grid, closest.borrow().location());
                    if arrived {
                        self.tile_score = Some(closest.borrow().score);
                        grid.remove(self.location);
                        let new_tile = grid.random_location();
                        closest.borrow_mut().location = new_tile;
                        closest.borrow_mut().score = rand::thread_rng().gen_range(1..5);
                        grid.set(new_tile);
                        self.state = AgentState::MoveToHole;
                    }
                } else {
                    warn!("Agent {}: No tile found", self.id);
                }
            }
            AgentState::MoveToHole => {
                if let Some(closest) = closest(self.location, &state.holes) {
                    debug!("Agent {}: Closest hole: {:?}", self.id, closest.borrow());
                    let arrived = self.move_to(&mut grid, closest.borrow().location());
                    if arrived {
                        self.score += self.tile_score.unwrap() as u32;
                        self.tile_score = None;
                        let new_hole = grid.random_location();
                        closest.borrow_mut().location = new_hole;
                        grid.set(new_hole);
                        self.state = AgentState::MoveToTile;
                        debug!("Agent {}: Score: {}", self.id, self.score);
                    }
                } else {
                    warn!("Agent {}: No hole found", self.id);
                }
            }
        }
        if log::log_enabled!(log::Level::Debug) {
            grid.print_grid();
        }
    }

    fn move_to(&mut self, grid: &mut Grid, to: Location) -> bool {
        if let Some(mut path) = astar(grid, self.location, to) {
            if path.len() == 0 {
                warn!("Agent {}: empty path", self.id);
                return false;
            }
            debug!("Agent {}: Path: {:?}", self.id, path);
            let direction = path.remove(0); // guaranteed to have at least one element
            let next = self.location.next_location(direction);
            grid.remove(self.location);
            self.location = next;
            grid.set(self.location);
            if next == to {
                return true;
            }
        } else {
            warn!("Agent {}: No path found", self.id);
        }
        false
    }

    pub(crate) fn render(&self, ctx: &mut BTerm) {
        let color = match self.id {
            0 => RED,
            1 => GREEN,
            2 => BLUE,
            3 => CYAN,
            4 => MAGENTA,
            5 => PLUM,
            _ => BLACK,
        };
        let c: u16 = if self.tile_score.is_some() { 219 } else { 254 };
        ctx.set(self.location.col, self.location.row, color, WHITE, c);
        ctx.print_color(
            COLS + 3,
            self.id as u16 + 3,
            color,
            WHITE,
            &format!("Agent {}: {}", self.id, self.score),
        );
    }
}
