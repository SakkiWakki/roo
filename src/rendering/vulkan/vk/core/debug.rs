use std::ffi::c_void;

use super::ffi::{
    message_type, severity, VkDebugUtilsMessageSeverityFlagsEXT, VkDebugUtilsMessageTypeFlagsEXT,
    VkDebugUtilsMessengerCallbackDataEXT, VkDebugUtilsMessengerCreateInfoEXT, VkStructureType,
};

pub fn create_debug_info() -> VkDebugUtilsMessengerCreateInfoEXT {
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

pub unsafe extern "C" fn debug_callback(
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
