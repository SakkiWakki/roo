/// https://docs.vulkan.org/refpages/latest/refpages/source/vkGetInstanceProcAddr.html
/// https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateInstance.html

// TODO: Switch to generic loader later
use crate::pal::LinuxLoader;
use crate::pal::Symbol;
use super::ffi::{
    PFN_vkAllocateCommandBuffers, PFN_vkAllocateMemory, PFN_vkBeginCommandBuffer,
    PFN_vkBindImageMemory, PFN_vkCmdBeginRenderPass, PFN_vkCmdDraw, PFN_vkCmdEndRenderPass,
    PFN_vkCreateCommandPool, PFN_vkCreateDebugUtilsMessengerEXT, PFN_vkCreateDevice,
    PFN_vkCreateFence, PFN_vkCreateFramebuffer, PFN_vkCreateGraphicsPipelines,
    PFN_vkCreateImage, PFN_vkCreateInstance, PFN_vkCreatePipelineLayout, PFN_vkCreateRenderPass,
    PFN_vkCreateSemaphore, PFN_vkCreateShaderModule, PFN_vkDestroyDebugUtilsMessengerEXT,
    PFN_vkDestroyImage, PFN_vkDestroyShaderModule, PFN_vkEndCommandBuffer,
    PFN_vkEnumeratePhysicalDevices, PFN_vkFreeMemory, PFN_vkGetDeviceProcAddr,
    PFN_vkGetDeviceQueue, PFN_vkGetImageMemoryRequirements, PFN_vkGetInstanceProcAddr,
    PFN_vkGetMemoryFdKHR, PFN_vkGetPhysicalDeviceMemoryProperties,
    PFN_vkGetPhysicalDeviceProperties2, PFN_vkQueueSubmit, PFN_vkResetFences,
    PFN_vkWaitForFences, VkDevice, VkInstance,
};

pub struct VulkanLoader<'a> {
    pub get_instance_proc_addr: Symbol<'a, PFN_vkGetInstanceProcAddr>,
    pub create_instance: Symbol<'a, PFN_vkCreateInstance>,
}

pub struct InstanceLoader {
    pub enumerate_physical_devices: PFN_vkEnumeratePhysicalDevices,
    pub get_physical_device_properties2: PFN_vkGetPhysicalDeviceProperties2,
    pub get_physical_device_memory_properties: PFN_vkGetPhysicalDeviceMemoryProperties,
    pub create_device: PFN_vkCreateDevice,
    pub get_device_proc_addr: PFN_vkGetDeviceProcAddr,
    pub create_debug_utils_messenger: PFN_vkCreateDebugUtilsMessengerEXT,
    pub destroy_debug_utils_messenger: PFN_vkDestroyDebugUtilsMessengerEXT,
}

pub struct DeviceLoader {
    pub get_device_queue: PFN_vkGetDeviceQueue,
    pub allocate_memory: PFN_vkAllocateMemory,
    pub free_memory: PFN_vkFreeMemory,
    pub create_image: PFN_vkCreateImage,
    pub destroy_image: PFN_vkDestroyImage,
    pub get_image_memory_requirements: PFN_vkGetImageMemoryRequirements,
    pub bind_image_memory: PFN_vkBindImageMemory,
    pub get_memory_fd: PFN_vkGetMemoryFdKHR,
    pub create_render_pass: PFN_vkCreateRenderPass,
    pub create_framebuffer: PFN_vkCreateFramebuffer,
    pub create_graphics_pipelines: PFN_vkCreateGraphicsPipelines,
    pub create_pipeline_layout: PFN_vkCreatePipelineLayout,
    pub create_shader_module: PFN_vkCreateShaderModule,
    pub destroy_shader_module: PFN_vkDestroyShaderModule,
    pub create_command_pool: PFN_vkCreateCommandPool,
    pub allocate_command_buffers: PFN_vkAllocateCommandBuffers,
    pub begin_command_buffer: PFN_vkBeginCommandBuffer,
    pub end_command_buffer: PFN_vkEndCommandBuffer,
    pub cmd_begin_render_pass: PFN_vkCmdBeginRenderPass,
    pub cmd_end_render_pass: PFN_vkCmdEndRenderPass,
    pub cmd_draw: PFN_vkCmdDraw,
    pub queue_submit: PFN_vkQueueSubmit,
    pub create_fence: PFN_vkCreateFence,
    pub wait_for_fences: PFN_vkWaitForFences,
    pub reset_fences: PFN_vkResetFences,
    pub create_semaphore: PFN_vkCreateSemaphore,
}

impl<'a> VulkanLoader<'a> {
    pub fn load(loader: &'a LinuxLoader) -> Self {
        let get_instance_proc_addr = unsafe {
            loader.symbol::<PFN_vkGetInstanceProcAddr>("vkGetInstanceProcAddr")
        }.unwrap();
        let create_instance = unsafe {
            loader.symbol::<PFN_vkCreateInstance>("vkCreateInstance")
        }.unwrap();
        Self { get_instance_proc_addr, create_instance }
    }
}

// TODO: Safety
impl InstanceLoader {
    pub fn load(vk: &VulkanLoader, instance: VkInstance) -> Self {
        let load = |name: &[u8]| unsafe {
            let proc = vk.get_instance_proc_addr.ptr;
            proc(instance, name.as_ptr() as _)
                .expect("missing instance function")
        };
        unsafe {
            Self {
                enumerate_physical_devices: std::mem::transmute(
                    load(b"vkEnumeratePhysicalDevices\0")),
                get_physical_device_properties2: std::mem::transmute(
                    load(b"vkGetPhysicalDeviceProperties2\0")),
                get_physical_device_memory_properties: std::mem::transmute(
                    load(b"vkGetPhysicalDeviceMemoryProperties\0")),
                create_device: std::mem::transmute(
                    load(b"vkCreateDevice\0")),
                get_device_proc_addr: std::mem::transmute(
                    load(b"vkGetDeviceProcAddr\0")),
                create_debug_utils_messenger: std::mem::transmute(
                    load(b"vkCreateDebugUtilsMessengerEXT\0")),
                destroy_debug_utils_messenger: std::mem::transmute(
                    load(b"vkDestroyDebugUtilsMessengerEXT\0")),
            }
        }
    }
}

// TODO: Safety
impl DeviceLoader {
    pub fn load(instance: &InstanceLoader, device: VkDevice) -> Self {
        let load = |name: &[u8]| unsafe {
            let proc = instance.get_device_proc_addr;
            proc(device, name.as_ptr() as _)
                .expect("missing device function")
        };
        unsafe {
            Self {
                get_device_queue: std::mem::transmute(
                    load(b"vkGetDeviceQueue\0")),
                allocate_memory: std::mem::transmute(
                    load(b"vkAllocateMemory\0")),
                free_memory: std::mem::transmute(
                    load(b"vkFreeMemory\0")),
                create_image: std::mem::transmute(
                    load(b"vkCreateImage\0")),
                destroy_image: std::mem::transmute(
                    load(b"vkDestroyImage\0")),
                get_image_memory_requirements: std::mem::transmute(
                    load(b"vkGetImageMemoryRequirements\0")),
                bind_image_memory: std::mem::transmute(
                    load(b"vkBindImageMemory\0")),
                get_memory_fd: std::mem::transmute(
                    load(b"vkGetMemoryFdKHR\0")),
                create_render_pass: std::mem::transmute(
                    load(b"vkCreateRenderPass\0")),
                create_framebuffer: std::mem::transmute(
                    load(b"vkCreateFramebuffer\0")),
                create_graphics_pipelines: std::mem::transmute(
                    load(b"vkCreateGraphicsPipelines\0")),
                create_pipeline_layout: std::mem::transmute(
                    load(b"vkCreatePipelineLayout\0")),
                create_shader_module: std::mem::transmute(
                    load(b"vkCreateShaderModule\0")),
                destroy_shader_module: std::mem::transmute(
                    load(b"vkDestroyShaderModule\0")),
                create_command_pool: std::mem::transmute(
                    load(b"vkCreateCommandPool\0")),
                allocate_command_buffers: std::mem::transmute(
                    load(b"vkAllocateCommandBuffers\0")),
                begin_command_buffer: std::mem::transmute(
                    load(b"vkBeginCommandBuffer\0")),
                end_command_buffer: std::mem::transmute(
                    load(b"vkEndCommandBuffer\0")),
                cmd_begin_render_pass: std::mem::transmute(
                    load(b"vkCmdBeginRenderPass\0")),
                cmd_end_render_pass: std::mem::transmute(
                    load(b"vkCmdEndRenderPass\0")),
                cmd_draw: std::mem::transmute(
                    load(b"vkCmdDraw\0")),
                queue_submit: std::mem::transmute(
                    load(b"vkQueueSubmit\0")),
                create_fence: std::mem::transmute(
                    load(b"vkCreateFence\0")),
                wait_for_fences: std::mem::transmute(
                    load(b"vkWaitForFences\0")),
                reset_fences: std::mem::transmute(
                    load(b"vkResetFences\0")),
                create_semaphore: std::mem::transmute(
                    load(b"vkCreateSemaphore\0")),
            }
        }
    }
}
