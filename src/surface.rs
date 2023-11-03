use ash::vk;
use crate::instance::Instance;

pub trait SurfaceProvider {
    fn inner_size(&self) -> (u32, u32);
    fn create_surface(&self, instance: &Instance) -> vk::SurfaceKHR;
}

#[cfg(feature = "glfw")]
impl SurfaceProvider for glfw::Window {
    fn inner_size(&self) -> (u32, u32) {
        let s = self.get_size();
        (s.0 as u32, s.1 as u32)
    }

    fn create_surface(&self, instance: &Instance) {
        unsafe {
            let surf = vk::SurfaceKHR::null();
            self.create_window_surface(instance.vulkan_instance().handle(), std::ptr::null(), surf)
        }
    }
}