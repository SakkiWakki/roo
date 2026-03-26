use crate::pal::PlatformWindow;
use super::ffi::{
    image_usage, memory_property, VkDevice, VkDeviceMemory, VkExportMemoryAllocateInfo,
    VkImage, VkImageCreateInfo, VkImageDrmFormatModifierExplicitCreateInfoEXT, VkImageTiling,
    VkFormat, VkExternalMemoryImageCreateInfo, VkMemoryAllocateInfo, VkMemoryGetFdInfoKHR,
    VkMemoryRequirements, VkPhysicalDeviceMemoryProperties, VkResult,
};
use super::loader::{DeviceLoader, InstanceLoader};

fn drm_format_to_vk(drm_format: u32) -> VkFormat {
    match drm_format {
        0x34325241 | 0x34325258 => VkFormat::B8G8R8A8Srgb, // ARGB8888 / XRGB8888
        0x34324152 | 0x34324158 => VkFormat::R8G8B8A8Srgb, // ABGR8888 / XBGR8888
        _ => VkFormat::B8G8R8A8Srgb,
    }
}

pub fn create_image(device: VkDevice, loader: &DeviceLoader, window: &impl PlatformWindow) -> VkImage {
    let (width, height) = window.resolution();
    let first_format = window.formats().first();
    let format = first_format.map(|f| drm_format_to_vk(f.drm_format)).unwrap_or(VkFormat::B8G8R8A8Srgb);
    let modifier = first_format.map(|f| f.modifier);

    let external_memory = VkExternalMemoryImageCreateInfo::new();
    let modifier_info = modifier.map(|m| VkImageDrmFormatModifierExplicitCreateInfoEXT::new(m));

    let img_info = if let Some(ref mod_info) = modifier_info {
        let mut info = VkImageCreateInfo::new(
            width, height, format,
            image_usage::COLOR_ATTACHMENT | image_usage::TRANSFER_SRC,
            &external_memory,
            VkImageTiling::DrmFormatModifierEXT,
        );
        info.pNext = mod_info as *const _ as *const _;
        info
    } else {
        // Fallback to using linear tiling which is slow but usable
        VkImageCreateInfo::new(
            width, height, format,
            image_usage::COLOR_ATTACHMENT | image_usage::TRANSFER_SRC,
            &external_memory,
            VkImageTiling::Linear,
        )
    };

    let mut image = 0u64;
    let res = unsafe {
        (loader.create_image)(device, &img_info, std::ptr::null(), &mut image)
    };
    assert!(matches!(res, VkResult::Success));
    image
}

// Allocates image memory with the dmabuf
pub fn allocate_image_memory(
    device: VkDevice,
    device_loader: &DeviceLoader,
    instance_loader: &InstanceLoader,
    phys_device: super::ffi::VkPhysicalDevice,
    image: VkImage,
) -> VkDeviceMemory {
    let mut mem_reqs = VkMemoryRequirements::new();
    unsafe {
        (device_loader.get_image_memory_requirements)(device, image, &mut mem_reqs);
    };
    let mut mem_props = unsafe { std::mem::zeroed::<VkPhysicalDeviceMemoryProperties>() };
    unsafe { 
        (instance_loader.get_physical_device_memory_properties)(phys_device, &mut mem_props) 
    };
    let required_flags = memory_property::HOST_VISIBLE | memory_property::HOST_COHERENT;
    let memory_type_index = (0..mem_props.memoryTypeCount)
        .find(|&i| {
            let type_bits_match = mem_reqs.memoryTypeBits & (1 << i) != 0;
            let flags_match = mem_props.memoryTypes[i as usize].propertyFlags & required_flags == required_flags;
            type_bits_match && flags_match
        })
        .expect("No suitable memory type");
    let export_info = VkExportMemoryAllocateInfo::new();
    let alloc_info = VkMemoryAllocateInfo::new(mem_reqs.size, memory_type_index, &export_info);
    let mut device_memory: VkDeviceMemory = 0;
    let res = unsafe { (device_loader.allocate_memory)(device, &alloc_info, std::ptr::null(), &mut device_memory) };
    assert!(matches!(res, VkResult::Success));
    let res = unsafe { (device_loader.bind_image_memory)(device, image, device_memory, 0) };
    assert!(matches!(res, VkResult::Success));
    device_memory

}

// Exports mem as a dmabuf fd for Wayland
pub fn export_dmabuf_fd(
    device: VkDevice,
    loader: &DeviceLoader,
    memory: VkDeviceMemory,
) -> i32 {
    let get_fd_info = VkMemoryGetFdInfoKHR::new(memory);
    let mut fd: i32 = -1;
    let res = unsafe { (loader.get_memory_fd)(device, &get_fd_info, &mut fd) };
    assert!(matches!(res, VkResult::Success));
    fd
}
