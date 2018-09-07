//! This module contains some methods to modify a grid and
//! compute its next generations.

extern crate rand;

use std::sync::Arc;

use rand::Rng;

use super::vulkano::buffer::BufferUsage;
use super::vulkano::buffer::CpuAccessibleBuffer;
use super::vulkano::command_buffer::AutoCommandBufferBuilder;
use super::vulkano::command_buffer::CommandBuffer;
use super::vulkano::descriptor::descriptor_set::PersistentDescriptorSet;
use super::vulkano::format::ClearValue;
use super::vulkano::format::Format;
use super::vulkano::image::Dimensions;
use super::vulkano::image::StorageImage;
use super::vulkano::pipeline::ComputePipeline;
use super::vulkano::sync::GpuFuture;

use super::vulkan::ngs;
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
    pub fn next_gen(&mut self) {
        if !self.is_toroidal() {
            self.recenter_pattern();
        }

        let cells_in_img = StorageImage::new(
            self.device.clone(),
            Dimensions::Dim2d {
                width: self.grid_size.1 as u32,
                height: self.grid_size.0 as u32,
            },
            Format::R8Unorm,
            Some(self.queue.family()),
        ).expect("failed to create image");

        let cells_out_img = StorageImage::new(
            self.device.clone(),
            Dimensions::Dim2d {
                width: self.grid_size.1 as u32,
                height: self.grid_size.0 as u32,
            },
            Format::R8Unorm,
            Some(self.queue.family()),
        ).expect("failed to create image");

        let shader =
            ngs::Shader::load(self.device.clone()).expect("failed to create shader module");
        let compute_pipeline = Arc::new(
            ComputePipeline::new(self.device.clone(), &shader.main_entry_point(), &())
                .expect("failed to create compute pipeline"),
        );

        let set = Arc::new(
            PersistentDescriptorSet::start(compute_pipeline.clone(), 0)
                .add_image(cells_in_img.clone())
                .unwrap()
                .add_image(cells_out_img.clone())
                .unwrap()
                .add_buffer(self.toroidal.clone())
                .unwrap()
                .add_buffer(self.survival.clone())
                .unwrap()
                .add_buffer(self.birth.clone())
                .unwrap()
                .build()
                .unwrap(),
        );

        let command_buffer =
            AutoCommandBufferBuilder::new(self.device.clone(), self.queue.family())
                .unwrap()
                .copy_buffer_to_image(self.cells.clone(), cells_in_img.clone())
                .unwrap()
                .dispatch(
                    [
                        (self.grid_size.1 as f64 / 8.0).ceil() as u32,
                        (self.grid_size.0 as f64 / 8.0).ceil() as u32,
                        1,
                    ],
                    compute_pipeline.clone(),
                    set.clone(),
                    (),
                )
                .unwrap()
                .copy_image_to_buffer(cells_out_img.clone(), self.cells.clone())
                .unwrap()
                .build()
                .unwrap();

        let finished = command_buffer.execute(self.queue.clone()).unwrap();
        finished
            .then_signal_fence_and_flush()
            .unwrap()
            .wait(None)
            .unwrap();
    }

    fn recenter_pattern(&mut self) {
        let (min_x, max_x, min_y, max_y) = self.compute_pattern_boundaries();

        if min_x.is_none() || max_x.is_none() || min_y.is_none() || max_y.is_none() {
            return;
        }

        let (min_x, max_x, min_y, max_y) = (
            min_x.unwrap(),
            max_x.unwrap(),
            min_y.unwrap(),
            max_y.unwrap(),
        );

        let pattern_origin = (min_x as i32, min_y as i32);
        let pattern_size = ((max_x - min_x + 1), (max_y - min_y + 1));

        let cells_img = StorageImage::new(
            self.device.clone(),
            Dimensions::Dim2d {
                width: self.grid_size.1 as u32,
                height: self.grid_size.0 as u32,
            },
            Format::R8Unorm,
            Some(self.queue.family()),
        ).expect("failed to create image");

        let centered_img = StorageImage::new(
            self.device.clone(),
            Dimensions::Dim2d {
                width: pattern_size.0 as u32 + 2,
                height: pattern_size.1 as u32 + 2,
            },
            Format::R8Unorm,
            Some(self.queue.family()),
        ).expect("failed to create image");

        let centered_buff = unsafe {
            CpuAccessibleBuffer::uninitialized_array(
                self.device.clone(),
                (pattern_size.0 + 2) * (pattern_size.1 + 2),
                BufferUsage::all(),
            ).expect("failed to create buffer")
        };

        let command_buffer =
            AutoCommandBufferBuilder::new(self.device.clone(), self.queue.family())
                .unwrap()
                .clear_color_image(
                    centered_img.clone(),
                    ClearValue::Float([0.0, 0.0, 0.0, 0.0]),
                )
                .unwrap()
                .build()
                .unwrap();

        let finished = command_buffer.execute(self.queue.clone()).unwrap();
        finished
            .then_signal_fence_and_flush()
            .unwrap()
            .wait(None)
            .unwrap();

        let command_buffer =
            AutoCommandBufferBuilder::new(self.device.clone(), self.queue.family())
                .unwrap()
                .copy_buffer_to_image(self.cells.clone(), cells_img.clone())
                .unwrap()
                .copy_image(
                    cells_img.clone(),
                    [pattern_origin.0, pattern_origin.1, 0],
                    0,
                    0,
                    centered_img.clone(),
                    [1, 1, 0],
                    0,
                    0,
                    [pattern_size.0 as u32, pattern_size.1 as u32, 1],
                    1,
                )
                .unwrap()
                .copy_image_to_buffer(centered_img.clone(), centered_buff.clone())
                .unwrap()
                .build()
                .unwrap();

        let finished = command_buffer.execute(self.queue.clone()).unwrap();
        finished
            .then_signal_fence_and_flush()
            .unwrap()
            .wait(None)
            .unwrap();

        self.grid_size = (pattern_size.1 + 2, pattern_size.0 + 2);
        self.cells = centered_buff;
    }
}
