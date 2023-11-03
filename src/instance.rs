use std::ops::Deref;
use std::rc::Rc;
use ash::vk;
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

