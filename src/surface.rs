use ash::vk;
use crate::instance::Instance;

pub struct Surface {
    surface: vk::SurfaceKHR,
}

/// Provides a way to create a vulkan surface object
pub trait SurfaceProvider {
    /// Create a surface from self and the provided instance.
    fn create_surface_raw(&self, instance: &Instance) -> Option<vk::SurfaceKHR>;
}

#[derive(Debug)]
pub enum SurfaceInitError {
    CreateSurfaceError,
}

impl Surface {
    pub fn new(instance: &Instance, surface_provider: &dyn SurfaceProvider) -> Result<Self, SurfaceInitError> {
        match surface_provider.create_surface_raw(instance) {
            Some(surface) => {
                Ok(Self {
                    surface,
                })
            },
            None => Err(SurfaceInitError::CreateSurfaceError),
        }
    }

    /// Use this function when using glfw to make sure that you use the glfw logic for creating the surface. otherwise it will switch to the builtin logic for raw-window-handle (if glfw-rs ever updates to raw-window-handle 0.6.0, as of yet it has not), or it might just not work.
    #[cfg(feature = "glfw")]
    pub fn new_glfw(instance: &Instance, window: &glfw::Window) -> Result<Self, SurfaceInitError> {
        match create_surface_glfw(window, instance) {
            Some(surface) => {
                Ok(Self {
                    surface,
                })
            },
            None => Err(SurfaceInitError::CreateSurfaceError),
        }
    }

    // pub fn enumerate_surface_formats(&self, gpu: &PhysicalDevice) -> VkResult<Vec<vk::SurfaceFormatKHR>> {
    // }

    pub fn surface_handle(&self) -> vk::SurfaceKHR {
        self.surface
    }
}

// Window creation crate integrations

// the next 2 functions are a workaround due to conflicting implementations of SurfaceProvider when both features are enabled.
#[cfg(all(feature = "glfw", not(feature="raw-window-handle")))]
impl SurfaceProvider for glfw::Window {
    fn create_surface_raw(&self, instance: &Instance) -> Option<vk::SurfaceKHR> {
        create_surface_glfw(self, instance)
    }
}

#[cfg(feature = "glfw")]
fn create_surface_glfw(window: &glfw::Window, instance: &Instance) -> Option<vk::SurfaceKHR> {
    let mut surf = vk::SurfaceKHR::null();
    match window.create_window_surface(instance.handle().handle(), std::ptr::null(), &mut surf) {
        vk::Result::SUCCESS => Some(surf),
        e => None,
    }
}


// Raw window handle crate integration
#[cfg(feature = "raw-window-handle")]
mod os_raw {
    use ash::vk;
    use ash::vk::SurfaceKHR;
    use raw_window_handle::{HasDisplayHandle, HasWindowHandle, RawDisplayHandle, RawWindowHandle};
    use crate::instance::Instance;
    use crate::surface::SurfaceProvider;

    impl<T: HasWindowHandle + HasDisplayHandle> SurfaceProvider for T {
        fn create_surface_raw(&self, instance: &Instance) -> Option<SurfaceKHR> {
            match (self.window_handle().unwrap().as_raw(), self.display_handle().unwrap().as_raw()) {
                (RawWindowHandle::UiKit(_), RawDisplayHandle::UiKit(_)) => None,
                (RawWindowHandle::AppKit(_), RawDisplayHandle::AppKit(_)) => None,
                (RawWindowHandle::Orbital(_), RawDisplayHandle::Orbital(_)) => None,
                (RawWindowHandle::Drm(_), RawDisplayHandle::Drm(_)) => None,
                (RawWindowHandle::Gbm(_), RawDisplayHandle::Gbm(_)) => None,
                (RawWindowHandle::WinRt(_), RawDisplayHandle::Windows(_)) => None,
                (RawWindowHandle::Web(_), RawDisplayHandle::Web(_)) => None,
                (RawWindowHandle::WebCanvas(_), RawDisplayHandle::Web(_)) => None,
                (RawWindowHandle::WebOffscreenCanvas(_), RawDisplayHandle::Web(_)) => None,
                (RawWindowHandle::Haiku(_), RawDisplayHandle::Haiku(_)) => None,
                (RawWindowHandle::Xlib(wh), RawDisplayHandle::Xlib(dh)) => {
                    unsafe {
                        let sci = vk::XlibSurfaceCreateInfoKHR::builder()
                            .window(wh.window)
                            .dpy(dh.display.unwrap().as_ptr() as *mut _);
                        let surface_fn = ash::extensions::khr::XlibSurface::new(instance.entry(), instance.handle());
                        return surface_fn.create_xlib_surface(&sci, None).map_or(None, |s| Some(s));
                    }
                },
                (RawWindowHandle::Xcb(wh), RawDisplayHandle::Xcb(dh)) => {
                    unsafe {
                        let sci = vk::XcbSurfaceCreateInfoKHR::builder()
                            .window(wh.window.get() as _)
                            .connection(dh.connection.unwrap().as_ptr() as _);
                        let surface_fn = ash::extensions::khr::XcbSurface::new(instance.entry(), instance.handle());
                        return surface_fn.create_xcb_surface(&sci, None).map_or(None, |s| Some(s));
                    }
                },
                (RawWindowHandle::Wayland(wh), RawDisplayHandle::Wayland(dh)) => {
                    unsafe {
                        let sci = vk::WaylandSurfaceCreateInfoKHR::builder()
                            .display(dh.display.as_ptr() as _)
                            .surface(wh.surface.as_ptr() as _);
                        let surface_fn = ash::extensions::khr::WaylandSurface::new(instance.entry(), instance.handle());
                        return surface_fn.create_wayland_surface(&sci, None).map_or(None, |s| Some(s));
                    }
                },
                (RawWindowHandle::Win32(wh), RawDisplayHandle::Windows(_)) => {
                    unsafe {
                        let sci = vk::Win32SurfaceCreateInfoKHR::builder()
                            .hwnd(wh.hwnd.get() as _)
                            .hinstance(wh.hinstance.unwrap().get() as _);
                        let surface_fn = ash::extensions::khr::Win32Surface::new(instance.entry(), instance.handle());
                        return surface_fn.create_win32_surface(&sci, None).map_or(None, |s| Some(s));
                    }
                },
                (RawWindowHandle::AndroidNdk(wh), RawDisplayHandle::Android(_)) => {
                    unsafe {
                        let sci = vk::AndroidSurfaceCreateInfoKHR::builder()
                            .window(wh.a_native_window.as_ptr() as _);
                        let surface_fn = ash::extensions::khr::AndroidSurface::new(instance.entry(), instance.handle());
                        return surface_fn.create_android_surface(&sci, None).map_or(None, |s| Some(s));
                    }
                },
                _ => None
            }
        }
    }
}