use crate::location::{Located, Location};
use bracket_lib::prelude::*;

pub(crate) struct Tile {
    location: Location,
    pub(crate) score: u8,
}

impl Located for Tile {
    fn location(&self) -> Location {
        self.location
    }

    fn set_location(&mut self, new: Location) {
        self.location = new;
    }
}

impl Tile {
    pub(crate) fn new(location: Location, score: u8) -> Self {
        Self { location, score }
    }

    pub(crate) fn render(&self, ctx: &mut BTerm) {
        ctx.set(
            self.location.col,
            self.location.row,
            BLACK,
            WHITE,
            to_cp437('*'),
        );
    }
}
