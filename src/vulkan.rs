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
    ).expect("failed to create device");

    let queue = queues.next().unwrap();

    (device, queue)
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
