//! This module contains some methods to get informations and statistics
//! about the patterns of a grid.

use std::sync::Arc;

use super::vulkano::buffer::BufferUsage;
use super::vulkano::buffer::CpuAccessibleBuffer;
use super::vulkano::command_buffer::AutoCommandBufferBuilder;
use super::vulkano::command_buffer::CommandBuffer;
use super::vulkano::descriptor::descriptor_set::PersistentDescriptorSet;
use super::vulkano::format::Format;
use super::vulkano::image::Dimensions;
use super::vulkano::image::StorageImage;
use super::vulkano::pipeline::ComputePipeline;
use super::vulkano::sync::GpuFuture;

use super::vulkan::fms;
use Grid;

impl Grid {
    /// Returns the coordinates of the cell at the upper left corner of
    /// the current `Grid`.
    pub fn guess_pattern_origin(&self) -> (usize, usize) {
        let width = self.width;
        let height = self.height;

        if width == 0 || height == 0 {
            return (0, 0);
        }

        let mut orig_x = width;
        let mut orig_y = height;

        for y in 0..height {
            for x in 0..width {
                if self.get_cell_state(x as i64, y as i64) == 255 {
                    if x < orig_x {
                        orig_x = x;
                    }
                    if y < orig_y {
                        orig_y = y;
                    }
                }
            }
        }

        (orig_x, orig_y)
    }

    /// Returns the size of the current `Grid`'s pattern.
    pub fn guess_pattern_size(&self) -> (usize, usize) {
        let width = self.width;
        let height = self.height;

        if width == 0 || height == 0 {
            return (0, 0);
        }

        let (orig_x, orig_y) = self.guess_pattern_origin();
        let (mut lim_x, mut lim_y) = (orig_x, orig_y);

        for y in 0..height {
            for x in 0..width {
                if self.get_cell_state(x as i64, y as i64) == 255 {
                    if x > lim_x {
                        lim_x = x;
                    }
                    if y > lim_y {
                        lim_y = y;
                    }
                }
            }
        }

        (lim_x - orig_x + 1, lim_y - orig_y + 1)
    }

    pub fn compute_pattern_boundaries(
        &self,
    ) -> (Option<usize>, Option<usize>, Option<usize>, Option<usize>) {
        let cells_img = StorageImage::new(
            self.device.clone(),
            Dimensions::Dim2d {
                width: self.width as u32,
                height: self.height as u32,
            },
            Format::R8Unorm,
            Some(self.queue.family()),
        )
        .expect("failed to create image");

        let flat_map_x = CpuAccessibleBuffer::from_iter(
            self.device.clone(),
            BufferUsage::all(),
            (0..self.width).map(|_| 0),
        )
        .expect("failed to create buffer");

        let flat_map_y = CpuAccessibleBuffer::from_iter(
            self.device.clone(),
            BufferUsage::all(),
            (0..self.height).map(|_| 0),
        )
        .expect("failed to create buffer");

        let shader =
            fms::Shader::load(self.device.clone()).expect("failed to create shader module");
        let compute_pipeline = Arc::new(
            ComputePipeline::new(self.device.clone(), &shader.main_entry_point(), &())
                .expect("failed to create compute pipeline"),
        );

        let set = Arc::new(
            PersistentDescriptorSet::start(compute_pipeline.clone(), 0)
                .add_image(cells_img.clone())
                .unwrap()
                .add_buffer(flat_map_x.clone())
                .unwrap()
                .add_buffer(flat_map_y.clone())
                .unwrap()
                .build()
                .unwrap(),
        );

        let command_buffer =
            AutoCommandBufferBuilder::new(self.device.clone(), self.queue.family())
                .unwrap()
                .copy_buffer_to_image(self.cells.clone(), cells_img.clone())
                .unwrap()
                .dispatch(
                    [
                        (self.width as f64 / 8.0).ceil() as u32,
                        (self.height as f64 / 8.0).ceil() as u32,
                        1,
                    ],
                    compute_pipeline.clone(),
                    set.clone(),
                    (),
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

        let min_x = flat_map_x.read().unwrap().iter().position(|&n| n > 0);
        let max_x = flat_map_x.read().unwrap().iter().rposition(|&n| n > 0);

        let min_y = flat_map_y.read().unwrap().iter().position(|&n| n > 0);
        let max_y = flat_map_y.read().unwrap().iter().rposition(|&n| n > 0);

        (min_x, max_x, min_y, max_y)
    }
}
