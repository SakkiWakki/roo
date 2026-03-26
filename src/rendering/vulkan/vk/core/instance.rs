use super::ffi::{
    VkApplicationInfo, VkInstance, VkInstanceCreateInfo, VkResult, VkStructureType,
};
use super::loader::VulkanLoader;
use VkStructureType::*;

pub fn create_instance(loader: &VulkanLoader) -> VkInstance {
    let app_info = create_app_info();
    let create_info = create_instance_create_info(&app_info);

    let mut instance = std::ptr::null_mut();
    let result = unsafe {
        (loader.create_instance.ptr)(&create_info, std::ptr::null(), &mut instance)
    };
    assert!(matches!(result, VkResult::Success));
    instance
}

pub fn create_app_info() -> VkApplicationInfo {
    VkApplicationInfo {
        sType: ApplicationInfo, 
        pNext: std::ptr::null(),
        pApplicationName: b"roo\0".as_ptr() as _,
        applicationVersion: 1,
        pEngineName: b"roo\0".as_ptr() as _,
        engineVersion: 1,
        apiVersion: 0x00401000, // 1.1
    }
}

pub fn create_instance_create_info(app_info: &VkApplicationInfo) -> VkInstanceCreateInfo {
    VkInstanceCreateInfo {
        sType: InstanceCreateInfo,
        pNext: std::ptr::null(),
        flags: 0,
        pApplicationInfo: app_info,
        enabledLayerCount: 0,
        ppEnabledLayerNames: std::ptr::null(),
        enabledExtensionCount: 0,
        ppEnabledExtensionNames: std::ptr::null(),
    }
}