use roo::pal::{self, PlatformWindow};
use roo::rendering::vulkan::vk::core::{
    debug::create_debug_info,
    image::{allocate_image_memory, create_image, export_dmabuf_fd},
    instance::create_instance,
    loader::VulkanLoader,
};

fn main() -> Result<(), std::io::Error> {
    let mut window = pal::connect()?;
    let gpu_info = window.gpu_info().expect("no gpu device");

    let loader = pal::LinuxLoader::open("libvulkan.so.1");
    let vk = VulkanLoader::load(&loader);
    let debug_info = create_debug_info();

    let instance = create_instance(&vk, &debug_info, &gpu_info);

    let image = create_image(instance.device, &instance.device_loader, &window);
    let memory = allocate_image_memory(
        instance.device,
        &instance.device_loader,
        &instance.loader,
        instance.phys_device,
        image,
    );
    let fd = export_dmabuf_fd(instance.device, &instance.device_loader, memory);
    println!("dmabuf fd: {fd}");

    window.run()
}
