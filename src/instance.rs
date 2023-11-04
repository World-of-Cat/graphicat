use std::ops::Deref;
use std::rc::Rc;
use ash::vk;
use raw_window_handle::{HasWindowHandle, RawWindowHandle, WindowHandle};
use crate::ffi_util::CStringArray;

pub struct Instance {
    entry: ash::Entry,
    instance: ash::Instance,
}

pub trait SurfaceExtensionProvider {
    fn get_surface_extension(&self) -> Option<Vec<String>>;
}

#[derive(Debug)]
pub enum InstanceInitError {
    VulkanLoadingError(ash::LoadingError),
    InstanceCreateError(vk::Result),
}

impl Instance {
    pub fn handle(&self) -> &ash::Instance {
        &self.instance
    }

    pub unsafe fn new(os_extension_provider: &dyn SurfaceExtensionProvider) -> Result<Rc<Instance>, InstanceInitError> {
        match ash::Entry::load() {
            Ok(entry) => {
                let extensions = os_extension_provider.get_surface_extension().expect("Failed to get required instance extensions. Possibly unsupported system.");

                let extensions_cstr_array = CStringArray::from_vec(&extensions);

                let app_info = vk::ApplicationInfo::builder()
                    .api_version(vk::API_VERSION_1_3)
                    .build();

                match entry.create_instance(
                    &vk::InstanceCreateInfo::builder()
                        .application_info(&app_info)
                        .enabled_extension_names(extensions_cstr_array.as_ptr_slice()),
                    None) {
                    Ok(instance) => {
                        Ok(Rc::new(Instance {
                            entry,
                            instance,
                        }))
                    }
                    Err(e) => Err(InstanceInitError::InstanceCreateError(e)),
                }
            },
            Err(loading_error) => Err(InstanceInitError::VulkanLoadingError(loading_error)),
        }
    }


    pub fn entry(&self) -> &ash::Entry {
        &self.entry
    }
}

impl Deref for Instance {
    type Target = ash::Instance;

    fn deref(&self) -> &Self::Target {
        &self.instance
    }
}


#[cfg(feature = "glfw")]
impl SurfaceExtensionProvider for glfw::Glfw {
    fn get_surface_extension(&self) -> Option<Vec<String>> {
        self.get_required_instance_extensions()
    }
}


#[cfg(feature = "raw-window-handle")]
pub struct RwhExtensionProvider {
    window_handle: RawWindowHandle,
}

#[cfg(feature = "raw-window-handle")]
impl RwhExtensionProvider {
    pub fn new<T: HasWindowHandle>(o: &T) -> Self {
        Self {
            window_handle: o.window_handle().unwrap().as_raw(),
        }
    }
}

#[cfg(feature = "raw-window-handle")]
impl SurfaceExtensionProvider for RwhExtensionProvider {
    fn get_surface_extension(&self) -> Option<Vec<String>> {
        match self.window_handle {
            RawWindowHandle::UiKit(_) => None,
            RawWindowHandle::AppKit(_) => None,
            RawWindowHandle::Orbital(_) => None,
            RawWindowHandle::Xlib(_) => Some(vec!["VK_KHR_xlib_surface".to_string(), "VK_KHR_surface".to_string()]),
            RawWindowHandle::Xcb(_) => Some(vec!["VK_KHR_xcb_surface".to_string(), "VK_KHR_surface".to_string()]),
            RawWindowHandle::Wayland(_) => Some(vec!["VK_KHR_wayland_surface".to_string(), "VK_KHR_surface".to_string()]),
            RawWindowHandle::Drm(_) => None,
            RawWindowHandle::Gbm(_) => None,
            RawWindowHandle::Win32(_) => Some(vec!["VK_KHR_win32_surface".to_string(), "VK_KHR_surface".to_string()]),
            RawWindowHandle::WinRt(_) => None,
            RawWindowHandle::Web(_) => None,
            RawWindowHandle::WebCanvas(_) => None,
            RawWindowHandle::WebOffscreenCanvas(_) => None,
            RawWindowHandle::AndroidNdk(_) => None,
            RawWindowHandle::Haiku(_) => None,
            _ => None
        }
    }
}