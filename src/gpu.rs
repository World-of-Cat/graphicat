use std::collections::HashSet;
use std::ffi::CStr;
use std::ops::Deref;
use std::rc::Rc;
use ash::{vk, extensions::*};
use crate::instance::Instance;

pub fn required_device_extensions() -> Vec<&'static CStr> {
    vec![
        khr::Swapchain::name(),
    ]
}

#[derive(Clone)]
pub struct PhysicalDevice {
    instance: Rc<Instance>,
    physical_device: vk::PhysicalDevice,
}

pub trait GpuCompatibilityChecker {
    fn is_compatible(&self, physical_device: &PhysicalDevice) -> bool;
}

pub struct GpuSelectionParameters<'a> {
    allowed_types: Vec<vk::PhysicalDeviceType>,
    required_extension_support: Vec<&'a CStr>,
    user_compatibility_checker: Option<Box<dyn GpuCompatibilityChecker>>
}

impl Default for GpuSelectionParameters<'_> {
    fn default() -> Self {
        Self {
            allowed_types: vec![vk::PhysicalDeviceType::DISCRETE_GPU, vk::PhysicalDeviceType::INTEGRATED_GPU, vk::PhysicalDeviceType::CPU, vk::PhysicalDeviceType::VIRTUAL_GPU, vk::PhysicalDeviceType::OTHER],
            required_extension_support: required_device_extensions(),
            user_compatibility_checker: None,
        }
    }
}

impl <'a> GpuSelectionParameters<'a> {
    unsafe fn is_compatible(&self, physical_device: &PhysicalDevice) -> bool {
        if !self.allowed_types.contains(&physical_device.device_type()) { false }
        else {
            match physical_device.instance.enumerate_device_extension_properties(physical_device.physical_device) {
                Ok(extensions) => {
                    let mut available_extension_names = HashSet::<&CStr>::new();
                    for extension in &extensions {
                        available_extension_names.insert(CStr::from_ptr(extension.extension_name.as_ptr()));
                    }

                    for extension in &self.required_extension_support {
                        if !available_extension_names.contains(extension) { return false }
                    }

                    if let Some(checker) = &self.user_compatibility_checker {
                        checker.is_compatible(physical_device)
                    } else {
                        true
                    }
                }
                Err(_) => false
            }
        }
    }
}

impl PhysicalDevice {
    pub fn select(instance: Rc<Instance>, selection_parameters: GpuSelectionParameters) -> Option<Rc<Self>> {
        let mut discrete_device: Option<PhysicalDevice> = None; // this will remain None if the selection parameters dont allow for discrete gpus
        let mut integrated_device: Option<PhysicalDevice> = None;
        let mut supported_device: Option<PhysicalDevice> = None;

        unsafe {
            match instance.enumerate_physical_devices() {
                Ok(physical_devices) => {
                    for physical_device_v in physical_devices {
                        let physical_device = PhysicalDevice::wrap(physical_device_v, instance.clone());

                        if selection_parameters.is_compatible(&physical_device) {
                            // No need to replicate the value across the lower tiers as they will be ignored at the end once the better type of gpu is found (no need for integrated_device once a compatible discrete_device is found).
                            if physical_device.device_type() == vk::PhysicalDeviceType::DISCRETE_GPU {
                                discrete_device = Some(physical_device);
                            } else if physical_device.device_type() == vk::PhysicalDeviceType::INTEGRATED_GPU {
                                integrated_device = Some(physical_device);
                            } else {
                                supported_device = Some(physical_device);
                            }
                        }
                    }
                }
                Err(_) => return None
            }
        }


        if let Some(gpu) = discrete_device {
            Some(Rc::new(gpu))
        } else if let Some(gpu) = integrated_device {
            Some(Rc::new(gpu))
        } else if let Some(gpu) = supported_device {
            Some(Rc::new(gpu))
        } else {
            None
        }
    }

    pub fn wrap(physical_device: vk::PhysicalDevice, instance: Rc<Instance>) -> PhysicalDevice {
        Self {
            instance,
            physical_device,
        }
    }

    pub fn device_type(&self) -> vk::PhysicalDeviceType {
        unsafe {
            self.instance.get_physical_device_properties(self.physical_device).device_type
        }
    }

    pub fn name(&self) -> String {
        unsafe {
            CStr::from_ptr(self.instance.get_physical_device_properties(self.physical_device).device_name.as_ptr()).to_str().unwrap_or("").to_string()
        }
    }

    pub fn handle(&self) -> vk::PhysicalDevice {
        self.physical_device
    }
}

impl Deref for PhysicalDevice {
    type Target = vk::PhysicalDevice;

    fn deref(&self) -> &Self::Target {
        &self.physical_device
    }
}