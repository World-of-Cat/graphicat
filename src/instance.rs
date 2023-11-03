pub struct Instance {
    instance: ash::Instance,
}

impl Instance {
    pub fn vulkan_instance(&self) -> &ash::Instance {
        &self.instance
    }
}