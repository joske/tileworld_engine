use crate::location::{Located, Location};
use bracket_lib::prelude::*;

pub(crate) struct Hole {
    location: Location,
}

impl Located for Hole {
    fn location(&self) -> Location {
        self.location
    }

    fn set_location(&mut self, new: Location) {
        self.location = new;
    }
}

impl Hole {
    pub(crate) fn new(location: Location) -> Self {
        Self { location }
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
