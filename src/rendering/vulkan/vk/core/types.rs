use super::ffi::{
    VkDeviceCreateInfo, VkDeviceQueueCreateInfo, VkPhysicalDeviceDrmPropertiesEXT,
    VkPhysicalDeviceProperties2, VkStructureType,
};
use std::ffi::c_void;

impl VkPhysicalDeviceDrmPropertiesEXT {
    pub fn new() -> Self {
        Self {
            sType: VkStructureType::PhysicalDeviceDrmPropertiesEXT,
            pNext: std::ptr::null_mut(),
            hasPrimary: 0,
            hasRender: 0,
            primaryMajor: 0,
            primaryMinor: 0,
            renderMajor: 0,
            renderMinor: 0,
        }
    }
}

impl VkPhysicalDeviceProperties2 {
    pub fn new(next: *mut c_void) -> Self {
        Self {
            sType: VkStructureType::PhysicalDeviceProperties2,
            pNext: next,
            properties: unsafe { std::mem::zeroed() },
        }
    }
}

impl VkDeviceQueueCreateInfo {
    pub fn new(queue_family_index: u32, priority: &f32) -> Self {
        Self {
            sType: VkStructureType::DeviceQueueCreateInfo,
            pNext: std::ptr::null(),
            flags: 0,
            queueFamilyIndex: queue_family_index,
            queueCount: 1,
            pQueuePriorities: priority,
        }
    }
}

impl VkDeviceCreateInfo {
    pub fn new(queue_info: &VkDeviceQueueCreateInfo, extensions: &[*const u8]) -> Self {
        Self {
            sType: VkStructureType::DeviceCreateInfo,
            pNext: std::ptr::null(),
            flags: 0,
            queueCreateInfoCount: 1,
            pQueueCreateInfos: queue_info,
            enabledLayerCount: 0,
            ppEnabledLayerNames: std::ptr::null(),
            enabledExtensionCount: extensions.len() as u32,
            ppEnabledExtensionNames: extensions.as_ptr() as _,
            pEnabledFeatures: std::ptr::null(),
        }
    }
}
