use std::ffi::c_void;

use super::ffi::{
    VkApplicationInfo, VkDebugUtilsMessengerCreateInfoEXT, VkInstance, VkInstanceCreateInfo,
    VkResult, VkStructureType,
};
use super::loader::VulkanLoader;
use VkStructureType::*;

const LAYERS: &[*const u8] = &[b"VK_LAYER_KHRONOS_validation\0".as_ptr()];
const EXTENSIONS: &[*const u8] = &[b"VK_EXT_debug_utils\0".as_ptr()];

pub fn create_instance(loader: &VulkanLoader, debug_info: &VkDebugUtilsMessengerCreateInfoEXT) -> VkInstance {
    let app_info = create_app_info();
    let create_info = create_instance_create_info(&app_info, debug_info);

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

pub fn create_instance_create_info<'a>(
    app_info: &'a VkApplicationInfo,
    debug_info: &'a VkDebugUtilsMessengerCreateInfoEXT,
) -> VkInstanceCreateInfo {
    VkInstanceCreateInfo {
        sType: InstanceCreateInfo,
        pNext: debug_info as *const _ as *const c_void,
        flags: 0,
        pApplicationInfo: app_info,
        enabledLayerCount: LAYERS.len() as u32,
        ppEnabledLayerNames: LAYERS.as_ptr() as _,
        enabledExtensionCount: EXTENSIONS.len() as u32,
        ppEnabledExtensionNames: EXTENSIONS.as_ptr() as _,
    }
}
