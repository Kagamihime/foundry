//! This module contains some methods to get informations and statistics
//! about the patterns of a grid.

use Grid;

impl Grid {
    pub fn guess_pattern_origin(&self) -> (usize, usize) {
        let grid_size = self.get_grid_size();

        if grid_size == (0, 0) {
            return (0, 0);
        }

        let mut pattern_origin = grid_size;

        for row in 0..grid_size.0 {
            for col in 0..grid_size.1 {
                if self.get_cell_state(row as i64, col as i64) {
                    if row < pattern_origin.0 {
                        pattern_origin.0 = row;
                    }
                    if col < pattern_origin.1 {
                        pattern_origin.1 = col;
                    }
                }
            }
        }

        pattern_origin
    }

    pub fn guess_pattern_size(&self) -> (usize, usize) {
        let grid_size = self.get_grid_size();

        if grid_size == (0, 0) {
            return (0, 0);
        }

        let pattern_origin = self.guess_pattern_origin();
        let mut pattern_limit = pattern_origin;

        for row in 0..grid_size.0 {
            for col in 0..grid_size.1 {
                if self.get_cell_state(row as i64, col as i64) {
                    if row > pattern_limit.0 {
                        pattern_limit.0 = row;
                    }
                    if col > pattern_limit.1 {
                        pattern_limit.1 = col;
                    }
                }
            }
        }

        (pattern_limit.0 - pattern_origin.0 + 1, pattern_limit.1 - pattern_origin.1 + 1)
    }
}
