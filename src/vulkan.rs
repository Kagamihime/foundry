use std::sync::Arc;

use super::vulkano::device::Device;
use super::vulkano::device::DeviceExtensions;
use super::vulkano::device::Queue;
use super::vulkano::instance::Features;
use super::vulkano::instance::Instance;
use super::vulkano::instance::InstanceExtensions;
use super::vulkano::instance::PhysicalDevice;
use super::vulkano::instance::PhysicalDeviceType::DiscreteGpu;

pub fn vk_init() -> (Arc<Device>, Arc<Queue>) {
    let instance =
        Instance::new(None, &InstanceExtensions::none(), None).expect("failed to create instance");

    let physical = PhysicalDevice::enumerate(&instance)
        .find(|&dev| dev.ty() == DiscreteGpu)
        .expect("no discrete GPU available");

    let queue_family = physical
        .queue_families()
        .find(|&q| q.supports_graphics())
        .expect("couldn't find a graphical queue family");

    let img_extended_formats_feature = Features {
        shader_storage_image_extended_formats: true,
        ..Features::none()
    };

    let (device, mut queues) = Device::new(
        physical,
        &img_extended_formats_feature,
        &DeviceExtensions::none(),
        [(queue_family, 0.5)].iter().cloned(),
    )
    .expect("failed to create device");

    let queue = queues.next().unwrap();

    (device, queue)
}

pub mod ngs {
    #[derive(VulkanoShader)]
    #[ty = "compute"]
    #[src = "
    #version 450

    layout(local_size_x = 8, local_size_y = 8, local_size_z = 1) in;

    layout(set = 0, binding = 0, r8) uniform readonly image2D img_in;

    layout(set = 0, binding = 1, r8) uniform writeonly image2D img_out;

    layout(set = 0, binding = 2) buffer Toroidal {
        int opt;
    } tor;

    layout(set = 0, binding = 3) buffer Survival {
        uint rules[];
    } srvl;

    layout(set = 0, binding = 4) buffer Birth {
        uint rules[];
    } brth;

    void main() {
        ivec2 offsets[8] = { ivec2(-1, -1), ivec2(0, -1), ivec2(1, -1), ivec2(-1, 0), ivec2(1, 0),
                             ivec2(-1, 1), ivec2(0, 1), ivec2(1, 1) };
        ivec2 grid_size = imageSize(img_in);
        int living_neighbors = 0;

        for (int i = 0; i < 8; i++) {
            ivec2 access_coord = ivec2(gl_GlobalInvocationID.xy) + offsets[i];

            if (tor.opt != 0) {
                if (access_coord.x == -1) {
                    access_coord.x = grid_size.x - 1;
                }
                if (access_coord.y == -1) {
                    access_coord.y = grid_size.y - 1;
                }
                if (access_coord.x == grid_size.x) {
                    access_coord.x = 0;
                }
                if (access_coord.y == grid_size.y) {
                    access_coord.y = 0;
                }
            }

            if (access_coord.x >= 0 && access_coord.x < grid_size.x && access_coord.y >= 0 &&
                access_coord.y < grid_size.y) {
                if (imageLoad(img_in, access_coord).x == 1.0) {
                    living_neighbors++;
                }
            }
        }

        vec4 to_write = vec4(0.0);

        if (imageLoad(img_in, ivec2(gl_GlobalInvocationID.xy)).x == 1.0) {
            for (int i = 0; i < srvl.rules.length(); i++) {
                if (living_neighbors == srvl.rules[i]) {
                    to_write.x = 1.0;
                }
            }
        } else {
            for (int i = 0; i < brth.rules.length(); i++) {
                if (living_neighbors == brth.rules[i]) {
                    to_write.x = 1.0;
                }
            }
        }

        imageStore(img_out, ivec2(gl_GlobalInvocationID.xy), to_write);
    }
    "]
    struct Dummy;
}

pub mod fms {
    #[derive(VulkanoShader)]
    #[ty = "compute"]
    #[src = "
    #version 450

    layout(local_size_x = 8, local_size_y = 8, local_size_z = 1) in;

    layout(set = 0, binding = 0, r8) uniform readonly image2D img;

    layout(set = 0, binding = 1) buffer FlatMapX {
        int data[];
    } fmx;

    layout(set = 0, binding = 2) buffer FlatMapY {
        int data[];
    } fmy;

    void main() {
        ivec2 access_coord = ivec2(gl_GlobalInvocationID.xy);

        if (imageLoad(img, access_coord).x == 1.0) {
            fmx.data[access_coord.x] = 1;
            fmy.data[access_coord.y] = 1;
        }
    }
    "]
    struct Dummy;
}
