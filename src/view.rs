use crate::Grid;

pub struct View<'a> {
    grid: &'a Grid,

    x_pos: usize,
    y_pos: usize,

    width: usize,
    height: usize,
}

impl<'a> View<'a> {
    pub fn new(grid: &'a Grid) -> View<'a> {
        View::new_positioned(grid, 0, 0, grid.get_width(), grid.get_height())
    }

    pub fn new_positioned(
        grid: &'a Grid,
        x_pos: usize,
        y_pos: usize,
        width: usize,
        height: usize,
    ) -> View {
        View {
            grid,
            x_pos,
            y_pos,
            width,
            height,
        }
    }

    pub fn position(&self) -> (usize, usize) {
        (self.x_pos, self.y_pos)
    }

    pub fn set_position(&mut self, x: usize, y: usize) {
        self.x_pos = x;
        self.y_pos = y;
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn set_width(&mut self, w: usize) {
        self.width = w;
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn set_height(&mut self, h: usize) {
        self.height = h;
    }

    pub fn render(&self, img_height: usize, img_width: usize) -> Vec<u8> {
        unimplemented!();
    }
}
