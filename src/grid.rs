use log::debug;
use rand::Rng;

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
        self.occupancy[index] = Some(());
        debug!("set {:?}: count={}", location, self.count());
    }

    pub(crate) fn remove(&mut self, location: Location) {
        let index = (location.row as usize * COLS as usize + location.col as usize) as usize;
        self.occupancy[index] = None;
        debug!("remove {:?}: count={}", location, self.count());
    }

    pub(crate) fn is_free(&self, location: Location) -> bool {
        assert!((location.row as usize) < ROWS as usize);
        assert!((location.col as usize) < COLS as usize);
        let index = (location.row as usize * COLS as usize + location.col as usize) as usize;
        self.occupancy[index].is_none()
    }

    pub fn random_location(&self) -> Location {
        let mut rng = rand::thread_rng();
        let mut c: u16 = rng.gen_range(0..COLS);
        let mut r: u16 = rng.gen_range(0..ROWS);

        let mut new_loc = Location::new(c, r);
        while !self.is_free(new_loc) {
            c = rng.gen_range(0..COLS);
            r = rng.gen_range(0..ROWS);
            new_loc = Location::new(c, r);
        }
        new_loc
    }

    fn count(&self) -> usize {
        self.occupancy.iter().filter(|o| o.is_some()).count()
    }

    pub fn print_grid(&self) {
        for r in 0..ROWS {
            for c in 0..COLS {
                let loc = Location::new(c, r);
                if self.is_free(loc) {
                    print!("0");
                } else {
                    print!("1");
                }
            }
            println!();
        }
        println!("Objects: {}", self.count());
    }
}

#[cfg(test)]
mod tests {

    use crate::{grid::Grid, location::Location};

    #[test]
    fn test_grid() {
        let mut grid = Grid::new();
        assert!(grid.is_free(Location::new(0, 0)));
        assert_eq!(0, grid.count());

        grid.set(Location::new(0, 0));
        assert!(!grid.is_free(Location::new(0, 0)));
        assert_eq!(1, grid.count());

        grid.set(Location::new(9, 9));
        assert!(!grid.is_free(Location::new(9, 9)));
        assert_eq!(2, grid.count());

        grid.remove(Location::new(9, 9));
        assert!(grid.is_free(Location::new(9, 9)));
        assert_eq!(1, grid.count());
    }

    #[test]
    fn test_count() {
        let mut grid = Grid::new();
        for i in 1..100 {
            let loc = grid.random_location();
            assert!(grid.is_free(loc));
            grid.set(loc);
            assert_eq!(i, grid.count());
        }
    }

    #[test]
    fn test_print() {
        let mut grid = Grid::new();
        grid.set(Location::new(1, 0));
        grid.set(Location::new(0, 9));
        grid.print_grid();
    }
}
