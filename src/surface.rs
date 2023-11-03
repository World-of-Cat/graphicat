use ash::vk;
use crate::instance::Instance;

pub struct Surface {
    surface: vk::SurfaceKHR,
}

pub trait SurfaceProvider {
    fn create_surface_raw(&self, instance: &Instance) -> Result<vk::SurfaceKHR, vk::Result>;
}

#[derive(Debug)]
pub enum SurfaceInitError {
    CreateSurfaceError(vk::Result),
}

impl Surface {
    pub fn new(instance: &Instance, surface_provider: &dyn SurfaceProvider) -> Result<Self, SurfaceInitError> {
        match surface_provider.create_surface_raw(instance) {
            Ok(surface) => {
                Ok(Self {
                    surface,
                })
            },
            Err(result) => Err(SurfaceInitError::CreateSurfaceError(result)),
        }
    }

    // pub fn enumerate_surface_formats(&self, gpu: &PhysicalDevice) -> VkResult<Vec<vk::SurfaceFormatKHR>> {
    // }

    pub fn surface_handle(&self) -> vk::SurfaceKHR {
        self.surface
    }
}

// Window creation crate integrations

#[cfg(feature = "glfw")]
impl SurfaceProvider for glfw::Window {
    fn create_surface_raw(&self, instance: &Instance) -> Result<vk::SurfaceKHR, vk::Result> {
        let mut surf = vk::SurfaceKHR::null();
        match self.create_window_surface(instance.handle().handle(), std::ptr::null(), &mut surf) {
            vk::Result::SUCCESS => Ok(surf),
            e => Err(e),
        }
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
                RawWindowHandle::Xcb(_) => create_xcb_surface(window, entry, instance),
                RawWindowHandle::Wayland(_) => create_wayland_surface(window, entry, instance),
                RawWindowHandle::Win32(_) => create_win32_surface(window, entry, instance),
                RawWindowHandle::AndroidNdk(_) => create_android_surface(window, entry, instance),
                _ => None
            }
        }
    }

    pub(crate) fn create_surface(window: &Window, entry: &ash::Entry, instance: &ash::Instance) -> Option<vk::SurfaceKHR> {

    }

    fn create_xlib_surface(window: &Window, entry: &ash::Entry, instance: &ash::Instance) -> Option<vk::SurfaceKHR> {
        if let RawWindowHandle::Xlib(wh) = window.raw_window_handle().unwrap() {
            if let RawDisplayHandle::Xlib(dh) = window.raw_display_handle().unwrap() {

            }
        }

        None
    }

    fn create_xcb_surface(window: &Window, entry: &ash::Entry, instance: &ash::Instance) -> Option<vk::SurfaceKHR> {
        if let RawWindowHandle::Xcb(wh) = window.raw_window_handle().unwrap() {
            if let RawDisplayHandle::Xcb(dh) = window.raw_display_handle().unwrap() {
                unsafe {
                    let sci = vk::XcbSurfaceCreateInfoKHR::builder()
                        .window(wh.window.get() as _)
                        .connection(dh.connection.unwrap().as_ptr() as _);
                    let surface_fn = ash::extensions::khr::XcbSurface::new(entry, instance);
                    return surface_fn.create_xcb_surface(&sci, None).map_or(None, |s| Some(s));
                }
            }
        }

        None
    }

    fn create_wayland_surface(window: &Window, entry: &ash::Entry, instance: &ash::Instance) -> Option<vk::SurfaceKHR> {
        if let RawWindowHandle::Wayland(wh) = window.raw_window_handle().unwrap() {
            if let RawDisplayHandle::Wayland(dh) = window.raw_display_handle().unwrap() {
                unsafe {
                    let sci = vk::WaylandSurfaceCreateInfoKHR::builder()
                        .display(dh.display.as_ptr() as _)
                        .surface(wh.surface.as_ptr() as _);
                    let surface_fn = ash::extensions::khr::WaylandSurface::new(entry, instance);
                    return surface_fn.create_wayland_surface(&sci, None).map_or(None, |s| Some(s));
                }
            }
        }

        None
    }

    fn create_win32_surface(window: &Window, entry: &ash::Entry, instance: &ash::Instance) -> Option<vk::SurfaceKHR> {
        if let RawWindowHandle::Win32(wh) = window.raw_window_handle().unwrap() {
            unsafe {
                let sci = vk::Win32SurfaceCreateInfoKHR::builder()
                    .hwnd(wh.hwnd.get() as _)
                    .hinstance(wh.hinstance.unwrap().get() as _);
                let surface_fn = ash::extensions::khr::Win32Surface::new(entry, instance);
                return surface_fn.create_win32_surface(&sci, None).map_or(None, |s| Some(s));
            }
        }

        None
    }

    fn create_android_surface(window: &Window, entry: &ash::Entry, instance: &ash::Instance) -> Option<vk::SurfaceKHR> {
        if let RawWindowHandle::AndroidNdk(wh) = window.raw_window_handle().unwrap() {
            unsafe {
                let sci = vk::AndroidSurfaceCreateInfoKHR::builder()
                    .window(wh.a_native_window.as_ptr() as _);
                let surface_fn = ash::extensions::khr::AndroidSurface::new(entry, instance);
                return surface_fn.create_android_surface(&sci, None).map_or(None, |s| Some(s));
            }
        }

        None
    }
}