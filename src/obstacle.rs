use bracket_lib::prelude::*;

use crate::{location::Location, BTerm};

pub(crate) struct Obstacle {
    location: Location,
}

impl Obstacle {
    pub(crate) fn new(location: Location) -> Self {
        Self { location }
    }

    pub(crate) fn render(&self, ctx: &mut BTerm) {
        ctx.set(
            self.location.col,
            self.location.row,
            BLACK,
            WHITE,
            to_cp437('#'),
        );
    }
}
