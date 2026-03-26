/// https://docs.vulkan.org/refpages/latest/refpages/source/vkGetInstanceProcAddr.html
/// https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateInstance.html

// TODO: Switch to generic loader later
use create::pal::LinuxLoader;
use super::ffi::{PFN_vkCreateInstance, PFN_vkGetInstanceProcAddr, VkInstance};

pub struct VulkanLoader<'a> {
    pub get_instance_proc_addr: Symbol<'a, PFN_vkGetInstanceProcAddr>,
    pub create_instance: Symbol<'a, PFN_vkCreateInstance>
}

impl<'a> VulkanLoader<'a> {
    pub fn load(loader: &'a LinuxLoader) -> Self {
        let get_instance_proc_addr = unsafe {
            loader.symbol::<PFN_vkGetInstanceProcAddr>("vkGetInstanceProcAddr")
        }.unwrap();

        let create_instance = unsafe {
            loader.symbol::<PFN_vkCreateInstance>("vkCreateInstance")
        }.unwrap();
        Self { 
            get_instance_proc_addr, create_instance
        }
    }
}