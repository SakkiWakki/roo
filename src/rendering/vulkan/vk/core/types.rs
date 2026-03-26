use crate::rendering::vulkan::vk::core::ffi::VkMemoryRequirements;

use super::ffi::{
    VkDeviceCreateInfo, VkDeviceMemory, VkDeviceQueueCreateInfo, VkExportMemoryAllocateInfo,
    VkExtent3D, VkExternalMemoryHandleTypeFlagBits, VkExternalMemoryImageCreateInfo, VkFormat,
    VkImage, VkImageCreateInfo, VkImageDrmFormatModifierExplicitCreateInfoEXT, VkImageLayout,
    VkImageTiling, VkImageType, VkImageUsageFlags, VkMemoryAllocateInfo,
    VkMemoryDedicatedAllocateInfo, VkMemoryGetFdInfoKHR, VkPhysicalDeviceDrmPropertiesEXT,
    VkPhysicalDeviceProperties2, VkSharingMode, VkStructureType, VkSubresourceLayout,
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

impl VkExternalMemoryImageCreateInfo {
    pub fn new() -> Self {
        Self {
            sType: VkStructureType::ExternalMemoryImageCreateInfo,
            pNext: std::ptr::null(),
            handleTypes: VkExternalMemoryHandleTypeFlagBits::DmaBufEXT as u32,
        }
    }
}

impl VkImageDrmFormatModifierExplicitCreateInfoEXT {
    pub fn new(modifier: u64) -> Self {
        Self {
            sType: VkStructureType::ImageDrmFormatModifierExplicitCreateInfoEXT,
            pNext: std::ptr::null(),
            drmFormatModifier: modifier,
            drmFormatModifierPlaneCount: 1,
            pPlaneLayouts: &VkSubresourceLayout {
                offset: 0,
                size: 0,
                rowPitch: 0,
                arrayPitch: 0,
                depthPitch: 0,
            },
        }
    }
}

impl VkImageCreateInfo {
    pub fn new(
        width: u32,
        height: u32,
        format: VkFormat,
        usage: VkImageUsageFlags,
        external_memory: &VkExternalMemoryImageCreateInfo,
        tiling: VkImageTiling,
    ) -> Self {
        Self {
            sType: VkStructureType::ImageCreateInfo,
            pNext: external_memory as *const _ as *const c_void,
            flags: 0,
            imageType: VkImageType::Type2D,
            format,
            extent: VkExtent3D {
                width,
                height,
                depth: 1,
            },
            mipLevels: 1,
            arrayLayers: 1,
            samples: 1,
            tiling,
            usage,
            sharingMode: VkSharingMode::Exclusive,
            queueFamilyIndexCount: 0,
            pQueueFamilyIndices: std::ptr::null(),
            initialLayout: VkImageLayout::Undefined,
        }
    }
}

impl VkExportMemoryAllocateInfo {
    pub fn new() -> Self {
        Self {
            sType: VkStructureType::ExportMemoryAllocateInfo,
            pNext: std::ptr::null(),
            handleTypes: VkExternalMemoryHandleTypeFlagBits::DmaBufEXT as u32,
        }
    }
}

impl VkMemoryDedicatedAllocateInfo {
    pub fn new(image: VkImage, export: &VkExportMemoryAllocateInfo) -> Self {
        Self {
            sType: VkStructureType::MemoryDedicatedAllocateInfo,
            pNext: export as *const _ as *const c_void,
            image,
            buffer: 0,
        }
    }
}

impl VkMemoryAllocateInfo {
    pub fn new(
        size: u64,
        memory_type_index: u32,
        dedicated: &VkMemoryDedicatedAllocateInfo,
    ) -> Self {
        Self {
            sType: VkStructureType::MemoryAllocateInfo,
            pNext: dedicated as *const _ as *const c_void,
            allocationSize: size,
            memoryTypeIndex: memory_type_index,
        }
    }
}

impl VkMemoryGetFdInfoKHR {
    pub fn new(memory: VkDeviceMemory) -> Self {
        Self {
            sType: VkStructureType::MemoryGetFdInfoKHR,
            pNext: std::ptr::null(),
            memory,
            handleType: VkExternalMemoryHandleTypeFlagBits::DmaBufEXT as u32,
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

impl VkMemoryRequirements {
    pub fn new() -> Self {
        Self {
            size: 0,
            alignment: 0,
            memoryTypeBits: 0,
        }
    }
}
