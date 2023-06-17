use crate::{
    location::{closest, Located, Location},
    State,
};
use bracket_lib::prelude::*;

pub(crate) struct Agent {
    id: u8,
    pub(crate) location: Location,
    pub(crate) state: AgentState,
    score: u32,
}

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
        }
    }

    pub(crate) fn update(&mut self, state: &State) {
        match self.state {
            AgentState::MoveToTile => {
                if let Some(closest) = closest(self.location, &state.tiles) {
                    self.move_to(closest.location());
                }
            }
            AgentState::MoveToHole => {
                if let Some(closest) = closest(self.location, &state.holes) {
                    self.move_to(closest.location());
                }
            }
        }
    }

    fn move_to(&mut self, location: Location) {}

    pub(crate) fn render(&self, ctx: &mut BTerm) {
        let color = match self.id {
            0 => RED,
            1 => GREEN,
            2 => BLUE,
            3 => CYAN,
            4 => MAGENTA,
            5 => YELLOW,
            _ => WHITE,
        };
        ctx.set(
            self.location.col,
            self.location.row,
            color,
            WHITE,
            to_cp437('A'),
        );
    }
}
