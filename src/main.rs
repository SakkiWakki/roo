mod pal;
use roo::pal::{GpuInfo, Window};

fn main() -> Result<(), std::io::Error> {
    let mut window = pal::connect()?;
    let gpu_info = window.gpu_info().expect("no gpu device");
    window.run()
}
