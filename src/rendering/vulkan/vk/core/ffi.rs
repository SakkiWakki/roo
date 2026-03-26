// https://github.com/KhronosGroup/Vulkan-Headers/blob/main/include/vulkan/vulkan_core.h

#![allow(non_snake_case, non_camel_case_types)]
use std::ffi::{c_char, c_void};

pub type VkInstance = *mut c_void;
pub type VkDebugUtilsMessengerEXT = *mut c_void;
pub type VkDebugUtilsMessengerCreateFlagsEXT = u32;
pub type VkDebugUtilsMessageSeverityFlagsEXT = u32;
pub type VkDebugUtilsMessageTypeFlagsEXT = u32;

pub mod severity {
    pub const VERBOSE: u32 = 0x00000001;
    pub const INFO: u32    = 0x00000010;
    pub const WARNING: u32 = 0x00000100;
    pub const ERROR: u32   = 0x00001000;
}

pub mod message_type {
    pub const GENERAL: u32     = 0x00000001;
    pub const VALIDATION: u32  = 0x00000002;
    pub const PERFORMANCE: u32 = 0x00000004;
}

#[repr(i32)]
pub enum VkResult {
    Success = 0,
    NotReady = 1,
    Timeout = 2,
    EventSet = 3,
    EventReset = 4,
    Incomplete = 5,
    ErrorOutOfHostMemory = -1,
    ErrorOutOfDeviceMemory = -2,
    ErrorInitializationFailed = -3,
    ErrorDeviceLost = -4,
    ErrorMemoryMapFailed = -5,
    ErrorLayerNotPresent = -6,
    ErrorExtensionNotPresent = -7,
    ErrorFeatureNotPresent = -8,
    ErrorIncompatibleDriver = -9,
    ErrorTooManyObjects = -10,
    ErrorFormatNotSupported = -11,
    ErrorFragmentedPool = -12,
    ErrorUnknown = -13,
}

#[repr(i32)]
pub enum VkStructureType {
    ApplicationInfo = 0,
    InstanceCreateInfo = 1,
    DebugUtilsMessengerCreateInfoEXT = 1000128004,
}

#[repr(C)]
pub struct VkApplicationInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pApplicationName: *const c_char,
    pub applicationVersion: u32,
    pub pEngineName: *const c_char,
    pub engineVersion: u32,
    pub apiVersion: u32,
}

#[repr(C)]
pub struct VkInstanceCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: u32,
    pub pApplicationInfo: *const VkApplicationInfo,
    pub enabledLayerCount: u32,
    pub ppEnabledLayerNames: *const *const c_char,
    pub enabledExtensionCount: u32,
    pub ppEnabledExtensionNames: *const *const c_char,
}

#[repr(C)]
pub struct VkDebugUtilsMessengerCallbackDataEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: u32,
    pub pMessageIdName: *const c_char,
    pub messageIdNumber: i32,
    pub pMessage: *const c_char,
    pub queueLabelCount: u32,
    pub pQueueLabels: *const c_void,
    pub cmdBufLabelCount: u32,
    pub pCmdBufLabels: *const c_void,
    pub objectCount: u32,
    pub pObjects: *const c_void,
}

#[repr(C)]
pub struct VkDebugUtilsMessengerCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkDebugUtilsMessengerCreateFlagsEXT,
    pub messageSeverity: VkDebugUtilsMessageSeverityFlagsEXT,
    pub messageType: VkDebugUtilsMessageTypeFlagsEXT,
    pub pfnUserCallback: PFN_vkDebugUtilsMessengerCallbackEXT,
    pub pUserData: *mut c_void,
}

pub type PFN_vkDebugUtilsMessengerCallbackEXT = unsafe extern "C" fn(
    messageSeverity: VkDebugUtilsMessageSeverityFlagsEXT,
    messageTypes: VkDebugUtilsMessageTypeFlagsEXT,
    pCallbackData: *const VkDebugUtilsMessengerCallbackDataEXT,
    pUserData: *mut c_void,
) -> u32;

pub type PFN_vkGetInstanceProcAddr = unsafe extern "C" fn(
    instance: VkInstance,
    pName: *const c_char,
) -> Option<unsafe extern "C" fn()>;

pub type PFN_vkCreateInstance = unsafe extern "C" fn(
    pCreateInfo: *const VkInstanceCreateInfo,
    pAllocator: *const c_void,
    pInstance: *mut VkInstance,
) -> VkResult;

pub type PFN_vkCreateDebugUtilsMessengerEXT = unsafe extern "C" fn(
    instance: VkInstance,
    pCreateInfo: *const VkDebugUtilsMessengerCreateInfoEXT,
    pAllocator: *const c_void,
    pMessenger: *mut VkDebugUtilsMessengerEXT,
) -> VkResult;

pub type PFN_vkDestroyDebugUtilsMessengerEXT = unsafe extern "C" fn(
    instance: VkInstance,
    messenger: VkDebugUtilsMessengerEXT,
    pAllocator: *const c_void,
);
