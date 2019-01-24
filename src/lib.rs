//! This crate provides an interface to manipulate life
//! cellular automata grids.
//! Those grids can be toroidal or resizable.

extern crate rand;
#[macro_use]
extern crate vulkano_shader_derive;
extern crate vulkano;

pub mod analysis;
pub mod error;
pub mod file;
pub mod processing;
pub mod view;
mod vulkan;

use std::fmt;
use std::sync::Arc;

use vulkano::buffer::BufferUsage;
use vulkano::buffer::CpuAccessibleBuffer;
use vulkano::device::Device;
use vulkano::device::Queue;

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
    format: String,                          // Contains the file format used
    toroidal: Arc<CpuAccessibleBuffer<i32>>, // Resizable grid if set to false (note: false = 0 and true = 1)

    survival: Arc<CpuAccessibleBuffer<[u32]>>,
    birth: Arc<CpuAccessibleBuffer<[u32]>>,

    width: usize,
    height: usize,
    cells: Arc<CpuAccessibleBuffer<[u8]>>,

    device: Arc<Device>,
    queue: Arc<Queue>,
}

impl Grid {
    /// Returns a new `Grid`:
    /// * containing the file format `frmt`
    /// * toroidal if `trdl` is `true`, resizable otherwise
    /// * containing the rules given by `srvl` and `brth`
    /// * whose grid's size is determined by `width` and `height`
    pub fn new(
        frmt: &String,
        trdl: bool,
        srvl: &Vec<u32>,
        brth: &Vec<u32>,
        width: usize,
        height: usize,
    ) -> Grid {
        let (device, queue) = vulkan::vk_init();

        let new_cells_iter = (0..width * height).map(|_| 0u8);
        let new_cells =
            CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), new_cells_iter)
                .expect("failed to create buffer");

        let toroidal_val = if trdl { 1 } else { 0 };
        let toroidal =
            CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::all(), toroidal_val)
                .expect("failed to create buffer");

        let survival = CpuAccessibleBuffer::from_iter(
            device.clone(),
            BufferUsage::all(),
            srvl.iter().map(|&n| n),
        )
        .expect("failed to create buffer");

        let birth = CpuAccessibleBuffer::from_iter(
            device.clone(),
            BufferUsage::all(),
            brth.iter().map(|&n| n),
        )
        .expect("failed to create buffer");

        Grid {
            format: frmt.clone(),
            toroidal,
            survival,
            birth,
            width,
            height,
            cells: new_cells,
            device,
            queue,
        }
    }

    /// Returns a new `Grid` and initializes its cells randomly.
    pub fn new_random(
        frmt: &String,
        trdl: bool,
        srvl: &Vec<u32>,
        brth: &Vec<u32>,
        width: usize,
        height: usize,
    ) -> Grid {
        let mut new_grid = Grid::new(frmt, trdl, srvl, brth, width, height);
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
        let toroidal = self.toroidal.read().unwrap();

        match *toroidal {
            0 => false,
            _ => true,
        }
    }

    /// Returns the survival conditions of the cellular automaton.
    pub fn get_survival(&self) -> Vec<u32> {
        self.survival.read().unwrap().to_vec()
    }

    /// Redefines the survival conditions of the cellular automaton.
    pub fn set_survival(&mut self, srvl: &Vec<u32>) {
        self.survival = CpuAccessibleBuffer::from_iter(
            self.device.clone(),
            BufferUsage::all(),
            srvl.iter().map(|&n| n),
        )
        .expect("failed to create buffer");
    }

    /// Returns the birth conditions of the cellular automaton.
    pub fn get_birth(&self) -> Vec<u32> {
        self.birth.read().unwrap().to_vec()
    }

    /// Redefines the birth conditions of the cellular automaton.
    pub fn set_birth(&mut self, brth: &Vec<u32>) {
        self.birth = CpuAccessibleBuffer::from_iter(
            self.device.clone(),
            BufferUsage::all(),
            brth.iter().map(|&n| n),
        )
        .expect("failed to create buffer");
    }

    /// Returns the width of the grid.
    pub fn get_width(&self) -> usize {
        self.width
    }

    /// Returns the height of the grid.
    pub fn get_height(&self) -> usize {
        self.height
    }

    /// Returns the state of the cell at the coordinates (`x`, `y`).
    ///
    /// If the coordinates are out of bounds and the grid is toroidal,
    /// then it returns the state of the cell at the coordinates
    /// modulo the size of the grid.
    /// Otherwise, if the coordinates are out of bounds but
    /// the grid is not toroidal, it returns `0u8`.
    pub fn get_cell_state(&self, x: i64, y: i64) -> u8 {
        let cells = self.cells.write().unwrap();

        if cells.is_empty() {
            return 0;
        }

        // If the `x` and `y` parameters are out of bound of the grid
        if x < 0 || y < 0 || x as usize >= self.width || y as usize >= self.height {
            if self.is_toroidal() {
                let (x, y) = (
                    if x < 0 {
                        (self.width as i64 + x) as usize
                    } else if x as usize >= self.width {
                        x as usize % self.width
                    } else {
                        x as usize
                    },
                    if y < 0 {
                        (self.height as i64 + y) as usize
                    } else if y as usize >= self.height {
                        y as usize % self.height
                    } else {
                        y as usize
                    },
                );
                cells[y * self.width + x]
            } else {
                0
            }
        } else {
            cells[y as usize * self.width + x as usize]
        }
    }

    /// Modifies the state of the cell at the coordinates (`x`, `y`)
    /// with `state`.
    /// Returns `Err(GridErrorKind::OutOfBoundCoords)` if the
    /// coordinates are out of bounds.
    pub fn set_cell_state(&mut self, x: usize, y: usize, state: u8) -> Result<(), GridErrorKind> {
        let mut cells = self.cells.write().unwrap();

        if x >= self.width || y >= self.height {
            Err(GridErrorKind::OutOfBoundCoords)
        } else {
            cells[y * self.width + x] = state;
            Ok(())
        }
    }
}

impl Clone for Grid {
    fn clone(&self) -> Grid {
        let new_format = self.get_format();
        let new_toroidal = self.is_toroidal();
        let new_survival = self.get_survival();
        let new_birth = self.get_birth();
        let new_width = self.get_width();
        let new_height = self.get_height();

        let mut new_grid = Grid::new(
            &new_format,
            new_toroidal,
            &new_survival,
            &new_birth,
            new_width,
            new_height,
        );

        new_grid.cells = CpuAccessibleBuffer::from_iter(
            new_grid.device.clone(),
            BufferUsage::all(),
            self.cells.read().unwrap().to_vec().into_iter(),
        )
        .expect("failed to create buffer");

        new_grid
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Grid {
            ref format,
            ref toroidal,
            ref survival,
            ref birth,
            ref width,
            ref height,
            ..
        } = *self;

        write!(f, "Format:\n{:?}\nToroidal:\n{:?}\nSurvival:\n{:?}\nBirth:\n{:?}\nWidth:\n{:?}\nHeight:\n{:?}\nCells:\n{}", *format, *toroidal,  *survival, *birth, *width, height, self)
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Grid { width, height, .. } = *self;

        for y in 0..height {
            for x in 0..width {
                if self.get_cell_state(x as i64, y as i64) == 255 {
                    write!(f, "*")?;
                } else {
                    write!(f, ".")?;
                }
            }
            write!(f, "\n")?;
        }

        write!(f, "")
    }
}

#[cfg(test)]
mod tests {
    use super::vulkano::buffer::BufferUsage;
    use super::vulkano::buffer::CpuAccessibleBuffer;

    use super::vulkan;

    use Grid;

    #[test]
    fn test_toroidal_getters() {
        let (device, queue) = vulkan::vk_init();

        let cells_content: Vec<u8> = vec![0, 0, 0, 255, 255, 255, 0, 0, 0];
        let cells = CpuAccessibleBuffer::from_iter(
            device.clone(),
            BufferUsage::all(),
            cells_content.into_iter(),
        )
        .expect("failed to create buffer");

        let toroidal = CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::all(), 1)
            .expect("failed to create buffer");

        let srvl_content: Vec<u32> = vec![2, 3];
        let survival = CpuAccessibleBuffer::from_iter(
            device.clone(),
            BufferUsage::all(),
            srvl_content.into_iter(),
        )
        .expect("failed to create buffer");

        let brth_content: Vec<u32> = vec![3];
        let birth = CpuAccessibleBuffer::from_iter(
            device.clone(),
            BufferUsage::all(),
            brth_content.into_iter(),
        )
        .expect("failed to create buffer");

        let control_grid = Grid {
            format: String::from("#Toroidal Life"),
            toroidal,
            survival,
            birth,
            width: 3,
            height: 3,
            cells,
            device,
            queue,
        };

        // Test meta-data getters.
        assert_eq!("#Toroidal Life", control_grid.get_format());
        assert_eq!(true, control_grid.is_toroidal());
        assert_eq!(vec![2, 3], control_grid.get_survival());
        assert_eq!(vec![3], control_grid.get_birth());
        assert_eq!(3, control_grid.get_width());
        assert_eq!(3, control_grid.get_height());

        // Test cells getter.
        assert_eq!(0, control_grid.get_cell_state(0, 0));
        assert_eq!(255, control_grid.get_cell_state(0, 1));
        assert_eq!(0, control_grid.get_cell_state(0, 2));
        assert_eq!(0, control_grid.get_cell_state(1, 0));
        assert_eq!(255, control_grid.get_cell_state(1, 1));
        assert_eq!(0, control_grid.get_cell_state(1, 2));
        assert_eq!(0, control_grid.get_cell_state(2, 0));
        assert_eq!(255, control_grid.get_cell_state(2, 1));
        assert_eq!(0, control_grid.get_cell_state(2, 2));

        // Test cells getter for out of bound values.
        assert_eq!(0, control_grid.get_cell_state(-1, -1));
        assert_eq!(0, control_grid.get_cell_state(3, 3));
        assert_eq!(255, control_grid.get_cell_state(-1, 1));
        assert_eq!(255, control_grid.get_cell_state(3, 1));
    }

    #[test]
    fn test_toroidal_setters() {
        let (device, queue) = vulkan::vk_init();

        let cells_content: Vec<u8> = vec![0, 0, 0, 255, 255, 255, 0, 0, 0];
        let cells = CpuAccessibleBuffer::from_iter(
            device.clone(),
            BufferUsage::all(),
            cells_content.into_iter(),
        )
        .expect("failed to create buffer");

        let toroidal = CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::all(), 1)
            .expect("failed to create buffer");

        let srvl_content: Vec<u32> = vec![2, 3];
        let survival = CpuAccessibleBuffer::from_iter(
            device.clone(),
            BufferUsage::all(),
            srvl_content.into_iter(),
        )
        .expect("failed to create buffer");

        let brth_content: Vec<u32> = vec![3];
        let birth = CpuAccessibleBuffer::from_iter(
            device.clone(),
            BufferUsage::all(),
            brth_content.into_iter(),
        )
        .expect("failed to create buffer");

        let mut control_grid = Grid {
            format: String::from("#Toroidal Life"),
            toroidal,
            survival,
            birth,
            width: 3,
            height: 3,
            cells,
            device,
            queue,
        };

        control_grid.set_format(&String::from("#Resizable Life"));
        control_grid.set_survival(&vec![1, 7]);
        control_grid.set_birth(&vec![5]);
        control_grid.set_cell_state(0, 0, 255).unwrap();
        control_grid.set_cell_state(1, 1, 0).unwrap();

        // Check new meta-data.
        assert_eq!("#Resizable Life", control_grid.get_format());
        assert_eq!(vec![1, 7], control_grid.get_survival());
        assert_eq!(vec![5], control_grid.get_birth());

        // Check new cells.
        assert_eq!(255, control_grid.get_cell_state(0, 0));
        assert_eq!(255, control_grid.get_cell_state(0, 1));
        assert_eq!(0, control_grid.get_cell_state(0, 2));
        assert_eq!(0, control_grid.get_cell_state(1, 0));
        assert_eq!(0, control_grid.get_cell_state(1, 1));
        assert_eq!(0, control_grid.get_cell_state(1, 2));
        assert_eq!(0, control_grid.get_cell_state(2, 0));
        assert_eq!(255, control_grid.get_cell_state(2, 1));
        assert_eq!(0, control_grid.get_cell_state(2, 2));
    }

    #[test]
    fn test_resizable_getters() {
        let (device, queue) = vulkan::vk_init();

        let cells_content: Vec<u8> = vec![0, 0, 0, 255, 255, 255, 0, 0, 0];
        let cells = CpuAccessibleBuffer::from_iter(
            device.clone(),
            BufferUsage::all(),
            cells_content.into_iter(),
        )
        .expect("failed to create buffer");

        let toroidal = CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::all(), 0)
            .expect("failed to create buffer");

        let srvl_content: Vec<u32> = vec![2, 3];
        let survival = CpuAccessibleBuffer::from_iter(
            device.clone(),
            BufferUsage::all(),
            srvl_content.into_iter(),
        )
        .expect("failed to create buffer");

        let brth_content: Vec<u32> = vec![3];
        let birth = CpuAccessibleBuffer::from_iter(
            device.clone(),
            BufferUsage::all(),
            brth_content.into_iter(),
        )
        .expect("failed to create buffer");

        let control_grid = Grid {
            format: String::from("#Resizable Life"),
            toroidal,
            survival,
            birth,
            width: 3,
            height: 3,
            cells,
            device,
            queue,
        };

        // Test meta-data getters.
        assert_eq!("#Resizable Life", control_grid.get_format());
        assert_eq!(false, control_grid.is_toroidal());
        assert_eq!(vec![2, 3], control_grid.get_survival());
        assert_eq!(vec![3], control_grid.get_birth());
        assert_eq!(3, control_grid.get_width());
        assert_eq!(3, control_grid.get_height());

        // Test cells getter.
        assert_eq!(0, control_grid.get_cell_state(0, 0));
        assert_eq!(255, control_grid.get_cell_state(0, 1));
        assert_eq!(0, control_grid.get_cell_state(0, 2));
        assert_eq!(0, control_grid.get_cell_state(1, 0));
        assert_eq!(255, control_grid.get_cell_state(1, 1));
        assert_eq!(0, control_grid.get_cell_state(1, 2));
        assert_eq!(0, control_grid.get_cell_state(2, 0));
        assert_eq!(255, control_grid.get_cell_state(2, 1));
        assert_eq!(0, control_grid.get_cell_state(2, 2));

        // Test cells getter for out of bound values.
        assert_eq!(0, control_grid.get_cell_state(-1, -1));
        assert_eq!(0, control_grid.get_cell_state(3, 3));
        assert_eq!(0, control_grid.get_cell_state(1, -1));
        assert_eq!(0, control_grid.get_cell_state(1, 3));
    }

    #[test]
    fn test_resizable_setters() {
        let (device, queue) = vulkan::vk_init();

        let cells_content: Vec<u8> = vec![0, 0, 0, 255, 255, 255, 0, 0, 0];
        let cells = CpuAccessibleBuffer::from_iter(
            device.clone(),
            BufferUsage::all(),
            cells_content.into_iter(),
        )
        .expect("failed to create buffer");

        let toroidal = CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::all(), 0)
            .expect("failed to create buffer");

        let srvl_content: Vec<u32> = vec![2, 3];
        let survival = CpuAccessibleBuffer::from_iter(
            device.clone(),
            BufferUsage::all(),
            srvl_content.into_iter(),
        )
        .expect("failed to create buffer");

        let brth_content: Vec<u32> = vec![3];
        let birth = CpuAccessibleBuffer::from_iter(
            device.clone(),
            BufferUsage::all(),
            brth_content.into_iter(),
        )
        .expect("failed to create buffer");

        let mut control_grid = Grid {
            format: String::from("#Resizable Life"),
            toroidal,
            survival,
            birth,
            width: 3,
            height: 3,
            cells,
            device,
            queue,
        };

        control_grid.set_format(&String::from("#Toroidal Life"));
        control_grid.set_survival(&vec![1, 7]);
        control_grid.set_birth(&vec![5]);
        control_grid.set_cell_state(0, 0, 255).unwrap();
        control_grid.set_cell_state(1, 1, 0).unwrap();

        // Check new meta-data.
        assert_eq!("#Toroidal Life", control_grid.get_format());
        assert_eq!(vec![1, 7], control_grid.get_survival());
        assert_eq!(vec![5], control_grid.get_birth());

        // Check new cells.
        assert_eq!(255, control_grid.get_cell_state(0, 0));
        assert_eq!(255, control_grid.get_cell_state(0, 1));
        assert_eq!(0, control_grid.get_cell_state(0, 2));
        assert_eq!(0, control_grid.get_cell_state(1, 0));
        assert_eq!(0, control_grid.get_cell_state(1, 1));
        assert_eq!(0, control_grid.get_cell_state(1, 2));
        assert_eq!(0, control_grid.get_cell_state(2, 0));
        assert_eq!(255, control_grid.get_cell_state(2, 1));
        assert_eq!(0, control_grid.get_cell_state(2, 2));
    }
}
