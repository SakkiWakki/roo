use std::ffi::c_void;

use super::ffi::{
    VkApplicationInfo, VkDebugUtilsMessengerCreateInfoEXT, VkDebugUtilsMessengerEXT,
    VkInstance, VkInstanceCreateInfo, VkResult, VkStructureType,
};
use super::loader::{InstanceLoader, VulkanLoader};
use VkStructureType::*;

const LAYERS: &[*const u8] = &[b"VK_LAYER_KHRONOS_validation\0".as_ptr()];
const EXTENSIONS: &[*const u8] = &[b"VK_EXT_debug_utils\0".as_ptr()];

pub struct Instance {
    pub handle: VkInstance,
    pub loader: InstanceLoader,
    pub debug_messenger: VkDebugUtilsMessengerEXT,
}

pub fn create_instance(vk: &VulkanLoader, debug_info: &VkDebugUtilsMessengerCreateInfoEXT) -> Instance {
    let app_info = create_app_info();
    let create_info = create_instance_create_info(&app_info, debug_info);

    let mut handle = std::ptr::null_mut();
    let result = unsafe {
        (vk.create_instance.ptr)(&create_info, std::ptr::null(), &mut handle)
    };
    assert!(matches!(result, VkResult::Success));

    let loader = InstanceLoader::load(vk, handle);

    let mut debug_messenger = std::ptr::null_mut();
    let result = unsafe {
        (loader.create_debug_utils_messenger)(handle, debug_info, std::ptr::null(), &mut debug_messenger)
    };
    assert!(matches!(result, VkResult::Success));

    Instance { handle, loader, debug_messenger }
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
