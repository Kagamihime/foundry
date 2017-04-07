//! This crate provides an interface to manipulate life cellular automata grids.
//! Those grids can be toroidal or resizable.

extern crate rand;

pub mod analysis;
pub mod error;
pub mod file;
pub mod networking;
pub mod processing;

use std::iter::repeat;
use std::fmt;

use error::GridError;

// The grid of a life CA
pub struct Grid {
    format: String,
    toroidal: bool, // Resizable grid if set to false

    survival: Vec<u8>,
    birth: Vec<u8>,

    grid_size: (usize, usize),
    cells: Vec<bool>,
    // neighborhood_state: u8, // Count of living neighbors

    pattern_origin: (usize, usize),
}

impl Grid {
    pub fn new(frmt: &String, trdl: bool, srvl: &Vec<u8>, brth: &Vec<u8>, rows: usize, cols: usize, pttrn_rgn: Option<(usize, usize)>) -> Grid {
        let new_cells: Vec<bool> = repeat(false).take(rows * cols).collect();

        Grid {
            format: frmt.clone(),
            toroidal: trdl,
            survival: srvl.clone(),
            birth: brth.clone(),
            grid_size: (rows, cols),
            cells: new_cells,
            pattern_origin: match pttrn_rgn {
                None => (0, 0),
                Some((row, col)) => (row, col),
            },
        }
    }

    pub fn new_random(frmt: &String, trdl: bool, srvl: &Vec<u8>, brth: &Vec<u8>, rows: usize, cols: usize) -> Grid {
        let mut new_grid = Grid::new(frmt, trdl, srvl, brth, rows, cols, None);
        new_grid.randomize();
        new_grid
    }

    pub fn get_format(&self) -> String {
        self.format.clone()
    }
    pub fn set_format(&mut self, frmt: &String) {
        self.format = frmt.clone();
    }

    pub fn is_toroidal(&self) -> bool {
        self.toroidal
    }

    pub fn get_survival(&self) -> Vec<u8> {
        self.survival.clone()
    }
    pub fn set_survival(&mut self, srvl: &Vec<u8>) {
        self.survival = srvl.clone();
    }

    pub fn get_birth(&self) -> Vec<u8> {
        self.birth.clone()
    }
    pub fn set_birth(&mut self, brth: &Vec<u8>) {
        self.birth = brth.clone();
    }

    pub fn get_cell_state(&self, row: i64, col: i64) -> bool {
        if self.cells.len() == 0 {
            return false;
        }

        if row < 0 || col < 0 || row as usize >= self.grid_size.0 || col as usize >= self.grid_size.1 {
            if self.toroidal {
                let (row, col) = (
                    if row < 0 {
                        (self.grid_size.0 as i64 + row) as usize
                    } else if row as usize >= self.grid_size.0 {
                        row as usize % self.grid_size.0
                    } else {
                        row as usize
                    },
                    if col < 0 {
                        (self.grid_size.1 as i64 + col) as usize
                    } else if col as usize >= self.grid_size.1 {
                        col as usize % self.grid_size.1
                    } else {
                        col as usize
                    }
                );
                self.cells[row * self.grid_size.1 + col]
            } else {
                false
            }
        } else {
            self.cells[row as usize * self.grid_size.1 + col as usize]
        }
    }
    pub fn set_cell_state(&mut self, row: usize, col: usize, state: bool) -> Result<(), GridError> {
        if row >= self.grid_size.0 || col >= self.grid_size.1 {
            Err(GridError::OutOfBoundCoords)
        } else {
            self.cells[row * self.grid_size.1 + col] = state;
            Ok(())
        }
    }
    pub fn get_grid_size(&self) -> (usize, usize) {
        self.grid_size
    }

    pub fn get_pattern_origin(&self) -> (usize, usize) {
        (self.pattern_origin.0, self.pattern_origin.1)
    }
    pub fn update_pattern_origin(&mut self) {
        self.pattern_origin = self.guess_pattern_origin();
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Grid { ref format, ref toroidal, ref survival, ref birth, ref grid_size, ref pattern_origin, .. } = *self;

        write!(f, "Format:\n{:?}\nToroidal:\n{:?}\nSurvival:\n{:?}\nBirth:\n{:?}\nGrid size:\n{:?}\nPattern origin:\n{:?}\nCells:\n{}", *format, *toroidal,  *survival, *birth, *grid_size, *pattern_origin, self)
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Grid { ref grid_size, .. } = *self;

        for row in 0..grid_size.0 {
            for col in 0..grid_size.1 {
                if self.get_cell_state(row as i64, col as i64) {
                    try!(write!(f, "*"));
                } else {
                    try!(write!(f, "."));
                }
            }
            try!(write!(f, "\n"));
        }

        write!(f, "")
    }
}
