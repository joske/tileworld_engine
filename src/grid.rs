use bracket_lib::random::RandomNumberGenerator;

use crate::{location::Location, COLS, ROWS};

pub(crate) struct Grid {
    pub occupancy: Vec<Option<()>>,
}

impl Grid {
    pub(crate) fn new() -> Grid {
        Grid {
            occupancy: vec![None; COLS as usize * ROWS as usize],
        }
    }

    pub(crate) fn set(&mut self, location: Location) {
        let index = (location.row as usize * COLS as usize + location.col as usize) as usize;
        self.occupancy.insert(index, Some(()));
    }

    pub(crate) fn remove(&mut self, location: Location) {
        let index = (location.row as usize * COLS as usize + location.col as usize) as usize;
        self.occupancy.insert(index, None);
    }

    pub(crate) fn is_free(&self, location: Location) -> bool {
        assert!((location.row as usize) < ROWS as usize);
        assert!((location.col as usize) < COLS as usize);
        let index = (location.row as usize * COLS as usize + location.col as usize) as usize;
        self.occupancy[index].is_none()
    }

    pub fn random_location(&self, rng: &mut RandomNumberGenerator) -> Location {
        let mut c: u16 = rng.range(0, COLS);
        let mut r: u16 = rng.range(0, ROWS);

        let mut new_loc = Location::new(c, r);
        while !self.is_free(new_loc) {
            c = rng.range(0, COLS);
            r = rng.range(0, ROWS);
            new_loc = Location::new(c, r);
        }
        new_loc
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_grid() {
        let mut grid = Grid::new();
        assert!(grid.is_free(Location::new(0, 0)));
        grid.set(Location::new(0, 0));
        assert!(!grid.is_free(Location::new(0, 0)));
        grid.set(Location::new(9, 9));
        assert!(!grid.is_free(Location::new(9, 9)));
        grid.remove(Location::new(9, 9));
        assert!(grid.is_free(Location::new(9, 9)));
    }
}
