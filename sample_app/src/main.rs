extern crate glfw;

use graphicat::gpu::{GpuSelectionParameters, PhysicalDevice};
use graphicat::instance::Instance;

fn main() {
    let glfw = glfw::init(glfw::fail_on_errors).unwrap();

    let instance = unsafe { Instance::new(&glfw) }.unwrap();
    let physical_device = PhysicalDevice::select(instance, GpuSelectionParameters::default()).unwrap();

    println!("GPU: {}", physical_device.name());
    println!("GPU Type: {:?}", physical_device.device_type());

    println!("Required Extensions: {:?}", graphicat::gpu::required_device_extensions());
}
