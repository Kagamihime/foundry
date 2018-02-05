//! This crate provides an interface to manipulate life
//! cellular automata grids.
//! Those grids can be toroidal or resizable.

extern crate rand;

pub mod analysis;
pub mod error;
pub mod file;
pub mod networking;
pub mod processing;

use std::fmt;
use std::iter::repeat;

use error::GridErrorKind;

/// This struct contains the grid of a life cellular automaton.
///
/// This grid is stored as a `Vec<bool>`.
/// When it is toroidal, its size is constant. When it is not,
/// it is resized when computing the next generation
/// according to the size of the contained pattern.
///
/// The origin of the pattern is also stored in `Grid`:
/// the coordinates of its north west corner is stored as a
/// `(usize, usize)`.
///
/// It also contains the cellular automaton's rules stored as two `Vec<u8>`s.
/// These are the survival and birth conditions into `survival` and `birth`
/// respectivly.
pub struct Grid {
    format: String, // Contains the file format used
    toroidal: bool, // Resizable grid if set to false

    survival: Vec<u8>,
    birth: Vec<u8>,

    grid_size: (usize, usize),
    cells: Vec<bool>,
    // neighborhood_state: u8, // Count of living neighbors
    pattern_origin: (usize, usize),
}

impl Grid {
    /// Returns a new `Grid`:
    /// * containing the file format `frmt`
    /// * toroidal if `trdl` is `true`, resizable otherwise
    /// * containing the rules given by `srvl` and `brth`
    /// * whose grid's size is the same as `rows` and `cols`
    /// * `pttrn_rgn` can represent the relative position  of the pattern within the grid
    pub fn new(
        frmt: &String,
        trdl: bool,
        srvl: &Vec<u8>,
        brth: &Vec<u8>,
        rows: usize,
        cols: usize,
        pttrn_rgn: Option<(usize, usize)>,
    ) -> Grid {
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

    /// Returns a new `Grid` and initializes its cells randomly.
    pub fn new_random(
        frmt: &String,
        trdl: bool,
        srvl: &Vec<u8>,
        brth: &Vec<u8>,
        rows: usize,
        cols: usize,
    ) -> Grid {
        let mut new_grid = Grid::new(frmt, trdl, srvl, brth, rows, cols, None);
        new_grid.randomize();
        new_grid
    }

    /// Returns the file format used.
    pub fn get_format(&self) -> String {
        self.format.clone()
    }

    /// Sets a new file format for this `Grid`.
    pub fn set_format(&mut self, frmt: &String) {
        self.format = frmt.clone();
    }

    /// Returns `true` if the grid is toroidal. Otherwise the grid is resizable.
    pub fn is_toroidal(&self) -> bool {
        self.toroidal
    }

    /// Returns the survival conditions of the cellular automaton.
    pub fn get_survival(&self) -> Vec<u8> {
        self.survival.clone()
    }

    /// Redefines the survival conditions of the cellular automaton.
    pub fn set_survival(&mut self, srvl: &Vec<u8>) {
        self.survival = srvl.clone();
    }

    /// Returns the birth conditions of the cellular automaton.
    pub fn get_birth(&self) -> Vec<u8> {
        self.birth.clone()
    }

    /// Redefines the birth conditions of the cellular automaton.
    pub fn set_birth(&mut self, brth: &Vec<u8>) {
        self.birth = brth.clone();
    }

    /// Returns the size of the grid.
    pub fn get_grid_size(&self) -> (usize, usize) {
        self.grid_size
    }

    /// Returns the state of the cell at the relative coordinates
    /// (`row`, `col`).
    ///
    /// If the coordinates are out of bounds and the grid is toroidal,
    /// then it returns the state of the cell at the coordinates
    /// modulo the size of the grid.
    /// Otherwise, if the coordinates are out of bounds but
    /// the grid is not toroidal, it returns `false`.
    pub fn get_cell_state(&self, row: i64, col: i64) -> bool {
        if self.cells.is_empty() {
            return false;
        }

        // If the `row` and `col` parameters are out of bound of the grid
        if row < 0 || col < 0 || row as usize >= self.grid_size.0
            || col as usize >= self.grid_size.1
        {
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
                    },
                );
                self.cells[row * self.grid_size.1 + col]
            } else {
                false
            }
        } else {
            self.cells[row as usize * self.grid_size.1 + col as usize]
        }
    }

    /// Modifies the state of the cell at the coordinates (`row`, `col`)
    /// with `state`.
    /// Returns `Err(GridErrorKind::OutOfBoundCoords)` if the
    /// coordinates are out of bounds.
    pub fn set_cell_state(
        &mut self,
        row: usize,
        col: usize,
        state: bool,
    ) -> Result<(), GridErrorKind> {
        if row >= self.grid_size.0 || col >= self.grid_size.1 {
            Err(GridErrorKind::OutOfBoundCoords)
        } else {
            self.cells[row * self.grid_size.1 + col] = state;
            Ok(())
        }
    }

    /// Returns the current pattern origin.
    pub fn get_pattern_origin(&self) -> (usize, usize) {
        (self.pattern_origin.0, self.pattern_origin.1)
    }

    // TODO: make this method private so that it will be called
    // by `get_pattern_origin` instead
    pub fn update_pattern_origin(&mut self) {
        self.pattern_origin = self.guess_pattern_origin();
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Grid {
            ref format,
            ref toroidal,
            ref survival,
            ref birth,
            ref grid_size,
            ref pattern_origin,
            ..
        } = *self;

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
