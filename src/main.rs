use roo::pal;
use roo::rendering::vulkan::vk::core::ffi::{
    message_type, severity, VkDebugUtilsMessageSeverityFlagsEXT, VkDebugUtilsMessageTypeFlagsEXT,
    VkDebugUtilsMessengerCallbackDataEXT, VkDebugUtilsMessengerCreateInfoEXT, VkStructureType,
};
use roo::rendering::vulkan::vk::core::{instance::create_instance, loader::VulkanLoader};
use std::ffi::c_void;

fn main() -> Result<(), std::io::Error> {
    let mut window = pal::connect()?;
    let gpu_info = window.gpu_info().expect("no gpu device");

    let loader = pal::LinuxLoader::open("libvulkan.so.1");
    let vk = VulkanLoader::load(&loader);
    let debug_info = create_debug_info();

    let instance = create_instance(&vk, &debug_info, &gpu_info);
    println!("VkInstance: {:p}", instance.handle);

    window.run()
}

fn create_debug_info() -> VkDebugUtilsMessengerCreateInfoEXT {
    VkDebugUtilsMessengerCreateInfoEXT {
        sType: VkStructureType::DebugUtilsMessengerCreateInfoEXT,
        pNext: std::ptr::null(),
        flags: 0,
        messageSeverity: severity::VERBOSE | severity::INFO | severity::WARNING | severity::ERROR,
        messageType: message_type::GENERAL | message_type::VALIDATION | message_type::PERFORMANCE,
        pfnUserCallback: debug_callback,
        pUserData: std::ptr::null_mut(),
    }
}

unsafe extern "C" fn debug_callback(
    message_severity: VkDebugUtilsMessageSeverityFlagsEXT,
    _message_type: VkDebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const VkDebugUtilsMessengerCallbackDataEXT,
    _p_user_data: *mut c_void,
) -> u32 {
    let message = std::ffi::CStr::from_ptr((*p_callback_data).pMessage).to_string_lossy();
    let level = if message_severity >= severity::ERROR {
        "ERROR"
    } else if message_severity >= severity::WARNING {
        "WARN"
    } else if message_severity >= severity::INFO {
        "INFO"
    } else {
        "VERBOSE"
    };
    eprintln!("[Vulkan {level}] {message}");
    0
}
