// https://github.com/KhronosGroup/Vulkan-Headers/blob/main/include/vulkan/vulkan_core.h

#![allow(non_snake_case, non_camel_case_types)]
use std::ffi::{c_char, c_void};
pub enum VkInstance_T {}
pub type VkInstance = *mut VkInstance_T;

pub enum VkPhysicalDevice_T {}
pub type VkPhysicalDevice = *mut VkPhysicalDevice_T;

pub enum VkDevice_T {}
pub type VkDevice = *mut VkDevice_T;

pub enum VkQueue_T {}
pub type VkQueue = *mut VkQueue_T;

pub enum VkCommandBuffer_T {}
pub type VkCommandBuffer = *mut VkCommandBuffer_T;

pub type VkDebugUtilsMessengerEXT = *mut c_void;
pub type VkDeviceMemory = u64;
pub type VkImage = u64;
pub type VkImageView = u64;
pub type VkRenderPass = u64;
pub type VkFramebuffer = u64;
pub type VkPipeline = u64;
pub type VkPipelineLayout = u64;
pub type VkShaderModule = u64;
pub type VkCommandPool = u64;
pub type VkFence = u64;
pub type VkSemaphore = u64;

pub type VkDebugUtilsMessengerCreateFlagsEXT = u32;
pub type VkDebugUtilsMessageSeverityFlagsEXT = u32;
pub type VkDebugUtilsMessageTypeFlagsEXT = u32;
pub type VkMemoryPropertyFlags = u32;
pub type VkImageUsageFlags = u32;
pub type VkImageAspectFlags = u32;

// Image usage flags
pub mod image_usage {
    pub const TRANSFER_SRC: u32 = 0x00000001;
    pub const TRANSFER_DST: u32 = 0x00000002;
    pub const COLOR_ATTACHMENT: u32 = 0x00000010;
    pub const DEPTH_STENCIL: u32 = 0x00000020;
    pub const SAMPLED: u32 = 0x00000004;
    pub const STORAGE: u32 = 0x00000008;
}

// Memory property flags
pub mod memory_property {
    pub const DEVICE_LOCAL: u32 = 0x00000001;
    pub const HOST_VISIBLE: u32 = 0x00000002;
    pub const HOST_COHERENT: u32 = 0x00000004;
}

// Debug flags
pub mod severity {
    pub const VERBOSE: u32 = 0x00000001;
    pub const INFO: u32 = 0x00000010;
    pub const WARNING: u32 = 0x00000100;
    pub const ERROR: u32 = 0x00001000;
}

pub mod message_type {
    pub const GENERAL: u32 = 0x00000001;
    pub const VALIDATION: u32 = 0x00000002;
    pub const PERFORMANCE: u32 = 0x00000004;
}

// Enums
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
    MemoryAllocateInfo = 5,
    MemoryDedicatedAllocateInfo = 1000127001,
    ImageCreateInfo = 14,
    ImageViewCreateInfo = 15,
    RenderPassCreateInfo = 38,
    GraphicsPipelineCreateInfo = 28,
    PipelineLayoutCreateInfo = 30,
    ShaderModuleCreateInfo = 16,
    FramebufferCreateInfo = 37,
    CommandPoolCreateInfo = 39,
    CommandBufferAllocateInfo = 40,
    CommandBufferBeginInfo = 42,
    RenderPassBeginInfo = 43,
    FenceCreateInfo = 8,
    SemaphoreCreateInfo = 9,
    SubmitInfo = 4,
    MemoryGetFdInfoKHR = 1000074002,
    ExternalMemoryImageCreateInfo = 1000072001,
    ExportMemoryAllocateInfo = 1000072002,
    DeviceQueueCreateInfo = 2,
    DeviceCreateInfo = 3,
    PhysicalDeviceProperties2 = 1000059001,
    PhysicalDeviceDrmPropertiesEXT = 1000353000,
    DebugUtilsMessengerCreateInfoEXT = 1000128004,
    ImageDrmFormatModifierExplicitCreateInfoEXT = 1000158004,
}

#[repr(i32)]
pub enum VkFormat {
    Undefined = 0,
    B8G8R8A8Srgb = 50,
    R8G8B8A8Srgb = 43,
}

#[repr(i32)]
pub enum VkImageLayout {
    Undefined = 0,
    ColorAttachmentOptimal = 2,
    PresentSrcKHR = 1000001002,
}

#[repr(i32)]
pub enum VkImageTiling {
    Optimal = 0,
    Linear = 1,
    DrmFormatModifierEXT = 1000158000,
}

#[repr(i32)]
pub enum VkImageType {
    Type2D = 1,
}

#[repr(i32)]
pub enum VkSharingMode {
    Exclusive = 0,
}

#[repr(i32)]
pub enum VkExternalMemoryHandleTypeFlagBits {
    DmaBufEXT = 0x00000200,
}

// Instance structs
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

// Physical device structs
#[repr(C)]
pub struct VkPhysicalDeviceLimits {
    _data: [u8; 504],
}

#[repr(C)]
pub struct VkPhysicalDeviceSparseProperties {
    pub residencyStandard2DBlockShape: u32,
    pub residencyStandard2DMultisampleBlockShape: u32,
    pub residencyStandard3DBlockShape: u32,
    pub residencyAlignedMipSize: u32,
    pub residencyNonResidentStrict: u32,
}

#[repr(C)]
pub struct VkDeviceQueueCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: u32,
    pub queueFamilyIndex: u32,
    pub queueCount: u32,
    pub pQueuePriorities: *const f32,
}

#[repr(C)]
pub struct VkDeviceCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: u32,
    pub queueCreateInfoCount: u32,
    pub pQueueCreateInfos: *const VkDeviceQueueCreateInfo,
    pub enabledLayerCount: u32,
    pub ppEnabledLayerNames: *const *const u8,
    pub enabledExtensionCount: u32,
    pub ppEnabledExtensionNames: *const *const u8,
    pub pEnabledFeatures: *const c_void,
}

#[repr(C)]
pub struct VkPhysicalDeviceProperties {
    pub apiVersion: u32,
    pub driverVersion: u32,
    pub vendorID: u32,
    pub deviceID: u32,
    pub deviceType: u32,
    pub deviceName: [std::ffi::c_char; 256],
    pub pipelineCacheUUID: [u8; 16],
    pub limits: VkPhysicalDeviceLimits,
    pub sparseProperties: VkPhysicalDeviceSparseProperties,
}

#[repr(C)]
pub struct VkPhysicalDeviceProperties2 {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub properties: VkPhysicalDeviceProperties,
}

#[repr(C)]
pub struct VkPhysicalDeviceDrmPropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub hasPrimary: u32,
    pub hasRender: u32,
    pub primaryMajor: i64,
    pub primaryMinor: i64,
    pub renderMajor: i64,
    pub renderMinor: i64,
}

// Memory structs
#[repr(C)]
pub struct VkMemoryDedicatedAllocateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub image: VkImage,
    pub buffer: u64, // VkBuffer, unused here
}

#[repr(C)]
pub struct VkMemoryAllocateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub allocationSize: u64,
    pub memoryTypeIndex: u32,
}

#[repr(C)]
pub struct VkMemoryRequirements {
    pub size: u64,
    pub alignment: u64,
    pub memoryTypeBits: u32,
}

#[repr(C)]
pub struct VkMemoryType {
    pub propertyFlags: VkMemoryPropertyFlags,
    pub heapIndex: u32,
}

#[repr(C)]
pub struct VkMemoryHeap {
    pub size: u64,
    pub flags: u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceMemoryProperties {
    pub memoryTypeCount: u32,
    pub memoryTypes: [VkMemoryType; 32],
    pub memoryHeapCount: u32,
    pub memoryHeaps: [VkMemoryHeap; 16],
}

#[repr(C)]
pub struct VkExportMemoryAllocateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub handleTypes: u32,
}

#[repr(C)]
pub struct VkMemoryGetFdInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub memory: VkDeviceMemory,
    pub handleType: u32,
}

// Image structs
#[repr(C)]
pub struct VkExtent3D {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}

#[repr(C)]
pub struct VkImageCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: u32,
    pub imageType: VkImageType,
    pub format: VkFormat,
    pub extent: VkExtent3D,
    pub mipLevels: u32,
    pub arrayLayers: u32,
    pub samples: u32,
    pub tiling: VkImageTiling,
    pub usage: VkImageUsageFlags,
    pub sharingMode: VkSharingMode,
    pub queueFamilyIndexCount: u32,
    pub pQueueFamilyIndices: *const u32,
    pub initialLayout: VkImageLayout,
}

#[repr(C)]
pub struct VkExternalMemoryImageCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub handleTypes: u32,
}

#[repr(C)]
pub struct VkSubresourceLayout {
    pub offset: u64,
    pub size: u64,
    pub rowPitch: u64,
    pub arrayPitch: u64,
    pub depthPitch: u64,
}

#[repr(C)]
pub struct VkImageDrmFormatModifierExplicitCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub drmFormatModifier: u64,
    pub drmFormatModifierPlaneCount: u32,
    pub pPlaneLayouts: *const VkSubresourceLayout,
}

// Debug utils structs
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

// Instance function pointers
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

pub type PFN_vkEnumeratePhysicalDevices = unsafe extern "C" fn(
    instance: VkInstance,
    pPhysicalDeviceCount: *mut u32,
    pPhysicalDevices: *mut VkPhysicalDevice,
) -> VkResult;

pub type PFN_vkGetPhysicalDeviceProperties2 = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    pProperties: *mut VkPhysicalDeviceProperties2,
);

pub type PFN_vkGetPhysicalDeviceMemoryProperties = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties,
);

pub type PFN_vkCreateDevice = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    pCreateInfo: *const c_void,
    pAllocator: *const c_void,
    pDevice: *mut VkDevice,
) -> VkResult;

pub type PFN_vkGetDeviceProcAddr =
    unsafe extern "C" fn(device: VkDevice, pName: *const c_char) -> Option<unsafe extern "C" fn()>;

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

// Device function pointers
pub type PFN_vkGetDeviceQueue = unsafe extern "C" fn(
    device: VkDevice,
    queueFamilyIndex: u32,
    queueIndex: u32,
    pQueue: *mut VkQueue,
);

pub type PFN_vkAllocateMemory = unsafe extern "C" fn(
    device: VkDevice,
    pAllocateInfo: *const VkMemoryAllocateInfo,
    pAllocator: *const c_void,
    pMemory: *mut VkDeviceMemory,
) -> VkResult;

pub type PFN_vkFreeMemory =
    unsafe extern "C" fn(device: VkDevice, memory: VkDeviceMemory, pAllocator: *const c_void);

pub type PFN_vkCreateImage = unsafe extern "C" fn(
    device: VkDevice,
    pCreateInfo: *const VkImageCreateInfo,
    pAllocator: *const c_void,
    pImage: *mut VkImage,
) -> VkResult;

pub type PFN_vkDestroyImage =
    unsafe extern "C" fn(device: VkDevice, image: VkImage, pAllocator: *const c_void);

pub type PFN_vkGetImageMemoryRequirements = unsafe extern "C" fn(
    device: VkDevice,
    image: VkImage,
    pMemoryRequirements: *mut VkMemoryRequirements,
);

pub type PFN_vkBindImageMemory = unsafe extern "C" fn(
    device: VkDevice,
    image: VkImage,
    memory: VkDeviceMemory,
    memoryOffset: u64,
) -> VkResult;

pub type PFN_vkGetMemoryFdKHR = unsafe extern "C" fn(
    device: VkDevice,
    pGetFdInfo: *const VkMemoryGetFdInfoKHR,
    pFd: *mut i32,
) -> VkResult;

pub type PFN_vkCreateRenderPass = unsafe extern "C" fn(
    device: VkDevice,
    pCreateInfo: *const c_void,
    pAllocator: *const c_void,
    pRenderPass: *mut VkRenderPass,
) -> VkResult;

pub type PFN_vkCreateFramebuffer = unsafe extern "C" fn(
    device: VkDevice,
    pCreateInfo: *const c_void,
    pAllocator: *const c_void,
    pFramebuffer: *mut VkFramebuffer,
) -> VkResult;

pub type PFN_vkCreateGraphicsPipelines = unsafe extern "C" fn(
    device: VkDevice,
    pipelineCache: u64,
    createInfoCount: u32,
    pCreateInfos: *const c_void,
    pAllocator: *const c_void,
    pPipelines: *mut VkPipeline,
) -> VkResult;

pub type PFN_vkCreatePipelineLayout = unsafe extern "C" fn(
    device: VkDevice,
    pCreateInfo: *const c_void,
    pAllocator: *const c_void,
    pPipelineLayout: *mut VkPipelineLayout,
) -> VkResult;

pub type PFN_vkCreateShaderModule = unsafe extern "C" fn(
    device: VkDevice,
    pCreateInfo: *const c_void,
    pAllocator: *const c_void,
    pShaderModule: *mut VkShaderModule,
) -> VkResult;

pub type PFN_vkDestroyShaderModule =
    unsafe extern "C" fn(device: VkDevice, shaderModule: VkShaderModule, pAllocator: *const c_void);

pub type PFN_vkCreateCommandPool = unsafe extern "C" fn(
    device: VkDevice,
    pCreateInfo: *const c_void,
    pAllocator: *const c_void,
    pCommandPool: *mut VkCommandPool,
) -> VkResult;

pub type PFN_vkAllocateCommandBuffers = unsafe extern "C" fn(
    device: VkDevice,
    pAllocateInfo: *const c_void,
    pCommandBuffers: *mut VkCommandBuffer,
) -> VkResult;

pub type PFN_vkBeginCommandBuffer =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, pBeginInfo: *const c_void) -> VkResult;

pub type PFN_vkEndCommandBuffer = unsafe extern "C" fn(commandBuffer: VkCommandBuffer) -> VkResult;

pub type PFN_vkCmdBeginRenderPass = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    pRenderPassBegin: *const c_void,
    contents: i32,
);

pub type PFN_vkCmdEndRenderPass = unsafe extern "C" fn(commandBuffer: VkCommandBuffer);

pub type PFN_vkCmdDraw = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    vertexCount: u32,
    instanceCount: u32,
    firstVertex: u32,
    firstInstance: u32,
);

pub type PFN_vkQueueSubmit = unsafe extern "C" fn(
    queue: VkQueue,
    submitCount: u32,
    pSubmits: *const c_void,
    fence: VkFence,
) -> VkResult;

pub type PFN_vkCreateFence = unsafe extern "C" fn(
    device: VkDevice,
    pCreateInfo: *const c_void,
    pAllocator: *const c_void,
    pFence: *mut VkFence,
) -> VkResult;

pub type PFN_vkWaitForFences = unsafe extern "C" fn(
    device: VkDevice,
    fenceCount: u32,
    pFences: *const VkFence,
    waitAll: u32,
    timeout: u64,
) -> VkResult;

pub type PFN_vkResetFences =
    unsafe extern "C" fn(device: VkDevice, fenceCount: u32, pFences: *const VkFence) -> VkResult;

pub type PFN_vkCreateSemaphore = unsafe extern "C" fn(
    device: VkDevice,
    pCreateInfo: *const c_void,
    pAllocator: *const c_void,
    pSemaphore: *mut VkSemaphore,
) -> VkResult;
