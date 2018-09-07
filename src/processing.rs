//! This module contains some methods to modify a grid and
//! compute its next generations.

extern crate rand;

use rand::Rng;

use Grid;

const NEIGHBORHOOD_OFFSETS: [(i64, i64); 8] = [
    (-1, -1), // NW
    (-1, 0),  // N
    (-1, 1),  // NE
    (0, -1),  // W
    (0, 1),   // E
    (1, -1),  // SW
    (1, 0),   // S
    (1, 1),   // SE
];

impl Grid {
    /// Randomizes the current `Grid` by setting a random state to
    /// each cell.
    pub fn randomize(&mut self) {
        let mut rng = rand::thread_rng();

        let (rows, cols) = self.get_grid_size();
        for row in 0..rows {
            for col in 0..cols {
                if rng.gen::<bool>() {
                    self.set_cell_state(row, col, 255).unwrap(); // Shouldn't fail
                } else {
                    self.set_cell_state(row, col, 0).unwrap(); // Shouldn't fail
                }
            }
        }
    }

    /// Computes the next generation of the current `Grid` and updates it.
    pub fn next_gen(&self) -> Grid {
        if self.is_toroidal() {
            self.toroidal_next_gen()
        } else {
            self.resizable_next_gen()
        }
    }

    fn toroidal_next_gen(&self) -> Grid {
        let mut grid = {
            let (rows, cols) = self.get_grid_size();
            Grid::new(
                &self.get_format(),
                self.is_toroidal(),
                &self.get_survival(),
                &self.get_birth(),
                rows,
                cols,
                None,
            )
        };

        let (rows, cols) = grid.get_grid_size();
        for row in 0..rows {
            for col in 0..cols {
                let mut living_cells_around: u32 = 0;
                for offset in NEIGHBORHOOD_OFFSETS.iter() {
                    if self.get_cell_state(row as i64 + offset.0, col as i64 + offset.1) == 255 {
                        living_cells_around += 1;
                    }
                }
                if self.get_cell_state(row as i64, col as i64) == 255 {
                    if self.get_survival().contains(&living_cells_around) {
                        grid.set_cell_state(row, col, 255).unwrap(); // Shouldn't fail
                    } // else: remains false
                } else {
                    if self.get_birth().contains(&living_cells_around) {
                        grid.set_cell_state(row, col, 255).unwrap(); // Shouldn't fail
                    } // else: remains false
                }
            }
        }

        grid
    }

    fn resizable_next_gen(&self) -> Grid {
        let old_grid_size = self.get_grid_size();
        let old_pattern_origin = self.guess_pattern_origin();
        let old_pattern_size = self.guess_pattern_size();

        let new_grid_size: (usize, usize) =
            (2 + old_pattern_size.0 + 2, 2 + old_pattern_size.1 + 2);
        let new_pattern_origin: (usize, usize) = (2, 2);

        let mut grid = Grid::new(
            &self.get_format(),
            self.is_toroidal(),
            &self.get_survival(),
            &self.get_birth(),
            new_grid_size.0,
            new_grid_size.1,
            Some(new_pattern_origin),
        );

        let vertical_offset: i64 = new_pattern_origin.0 as i64 - old_pattern_origin.0 as i64;
        let horizontal_offset: i64 = new_pattern_origin.1 as i64 - old_pattern_origin.1 as i64;

        for row in 0..old_grid_size.0 {
            for col in 0..old_grid_size.1 {
                let mut living_cells_around: u32 = 0;
                for offset in NEIGHBORHOOD_OFFSETS.iter() {
                    if self.get_cell_state(row as i64 + offset.0, col as i64 + offset.1) == 255 {
                        living_cells_around += 1;
                    }
                }
                if self.get_cell_state(row as i64, col as i64) == 255 {
                    if self.get_survival().contains(&living_cells_around) {
                        grid.set_cell_state(
                            (row as i64 + vertical_offset) as usize,
                            (col as i64 + horizontal_offset) as usize,
                            255,
                        ).unwrap(); // Shouldn't fail
                    } // else: remains false
                } else {
                    if self.get_birth().contains(&living_cells_around) {
                        grid.set_cell_state(
                            (row as i64 + vertical_offset) as usize,
                            (col as i64 + horizontal_offset) as usize,
                            255,
                        ).unwrap(); // Shouldn't fail
                    } // else: remains false
                }
            }
        }

        grid.update_pattern_origin();
        grid
    }
}
