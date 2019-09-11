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

    pub fn render(&self, img_width: usize, img_height: usize) -> Vec<u8> {
        let view_width = if self.x_pos + self.width > self.grid.get_width() {
            let offset = self.x_pos + self.width - self.grid.get_width();

            self.width - offset
        } else {
            self.width
        };

        let view_height = if self.y_pos + self.height > self.grid.get_height() {
            let offset = self.y_pos + self.height - self.grid.get_height();

            self.height - offset
        } else {
            self.height
        };

        self.grid.render(
            self.x_pos,
            self.y_pos,
            view_width,
            view_height,
            img_width,
            img_height,
        )
    }
}

impl Grid {
    // There will be the default view builders
}
