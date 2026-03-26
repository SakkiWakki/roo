use std::ffi::c_void;
use std::os::unix::fs::MetadataExt;

use super::ffi::{
    VkApplicationInfo, VkDebugUtilsMessengerCreateInfoEXT, VkDebugUtilsMessengerEXT, VkDevice,
    VkDeviceCreateInfo, VkDeviceQueueCreateInfo, VkInstance, VkInstanceCreateInfo,
    VkPhysicalDevice, VkPhysicalDeviceDrmPropertiesEXT, VkPhysicalDeviceProperties2, VkResult,
    VkStructureType,
};
use super::loader::{DeviceLoader, InstanceLoader, VulkanLoader};
use crate::pal::GpuInfo;
use VkStructureType::*;

const LAYERS: &[*const u8] = &[b"VK_LAYER_KHRONOS_validation\0".as_ptr()];
const EXTENSIONS: &[*const u8] = &[b"VK_EXT_debug_utils\0".as_ptr()];
const DEVICE_EXTENSIONS: &[*const u8] = &[
    b"VK_KHR_external_memory\0".as_ptr(),
    b"VK_KHR_external_memory_fd\0".as_ptr(),
    b"VK_EXT_external_memory_dma_buf\0".as_ptr(),
    b"VK_EXT_image_drm_format_modifier\0".as_ptr(),
];

pub struct Instance {
    pub handle: VkInstance,
    pub loader: InstanceLoader,
    pub phys_device: VkPhysicalDevice,
    pub device: VkDevice,
    pub device_loader: DeviceLoader,
    pub debug_messenger: VkDebugUtilsMessengerEXT,
}

// We need gpu_info to match the device Wayland chose with the devices Vulkan enumerates for us
pub fn create_instance(
    vk: &VulkanLoader,
    debug_info: &VkDebugUtilsMessengerCreateInfoEXT,
    gpu_info: &GpuInfo,
) -> Instance {
    let (handle, loader) = create_handle(vk, debug_info);
    let debug_messenger = create_debug_messenger(&loader, handle, debug_info);
    let (phys_device, device, device_loader) = get_device(&loader, handle, gpu_info);
    Instance {
        handle,
        loader,
        phys_device,
        device,
        device_loader,
        debug_messenger,
    }
}

fn create_handle(
    vk: &VulkanLoader,
    debug_info: &VkDebugUtilsMessengerCreateInfoEXT,
) -> (VkInstance, InstanceLoader) {
    let app_info = create_app_info();
    let create_info = create_instance_create_info(&app_info, debug_info);
    let mut handle = std::ptr::null_mut();
    let res = unsafe { (vk.create_instance.ptr)(&create_info, std::ptr::null(), &mut handle) };
    assert!(matches!(res, VkResult::Success));
    (handle, InstanceLoader::load(vk, handle))
}

fn create_debug_messenger(
    loader: &InstanceLoader,
    handle: VkInstance,
    debug_info: &VkDebugUtilsMessengerCreateInfoEXT,
) -> VkDebugUtilsMessengerEXT {
    let mut debug_messenger = std::ptr::null_mut();
    let res = unsafe {
        (loader.create_debug_utils_messenger)(
            handle,
            debug_info,
            std::ptr::null(),
            &mut debug_messenger,
        )
    };
    assert!(matches!(res, VkResult::Success));
    debug_messenger
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

// TODO: Safety
pub fn get_device(
    instance: &InstanceLoader,
    handle: VkInstance,
    gpu_info: &GpuInfo,
) -> (VkPhysicalDevice, VkDevice, DeviceLoader) {
    let gpu_meta = gpu_info.device_node.metadata().unwrap();
    let dev_num = gpu_meta.rdev();
    let major = libc::major(dev_num);
    let minor = libc::minor(dev_num);
    let device_count = &mut 0u32;
    let phys_dev: Option<VkPhysicalDevice> = unsafe {
        // Need this or BOF
        (instance.enumerate_physical_devices)(handle, device_count, std::ptr::null_mut());

        let mut devs: Vec<VkPhysicalDevice> = Vec::with_capacity(*device_count as usize);
        ((instance.enumerate_physical_devices)(handle, device_count, devs.as_mut_ptr()));
        devs.set_len(*device_count as usize);
        let mut ret = None;
        for dev in devs {
            let mut drm_props = VkPhysicalDeviceDrmPropertiesEXT::new();
            let mut prop2: VkPhysicalDeviceProperties2 =
                VkPhysicalDeviceProperties2::new(&mut drm_props as *mut _ as *mut c_void);

            (instance.get_physical_device_properties2)(dev, &mut prop2);
            // DO NOT USE PRIMARY
            if drm_props.renderMajor == major as i64 && drm_props.renderMinor == minor as i64 {
                ret = Some(dev);
            }
        }
        ret
    };
    let phys_dev = phys_dev.expect("No matching GPU!");

    let priority: f32 = 1.0;
    let queue_info = VkDeviceQueueCreateInfo::new(0, &priority);
    let device_create_info = VkDeviceCreateInfo::new(&queue_info, DEVICE_EXTENSIONS);

    let mut vk_device = std::ptr::null_mut();
    let res = unsafe {
        (instance.create_device)(
            phys_dev,
            &device_create_info as *const _ as *const c_void,
            std::ptr::null(),
            &mut vk_device,
        )
    };
    assert!(matches!(res, VkResult::Success));
    (phys_dev, vk_device, DeviceLoader::load(instance, vk_device))
}
