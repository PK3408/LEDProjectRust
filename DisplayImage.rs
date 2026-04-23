use show_image::{ImageView, ImageInfo, create_window, WindowOptions, WindowHandle};
use walkdir::WalkDir;
use image::ImageReader;
use std::{thread, time};


#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {

  loop{

  //remember to replace with absolute path in new linux kernel
  for entry in WalkDir::new("/home/pranavkartha/Documents/LEDProject/LEDProject/ads").min_depth(1){

  let entry = entry.unwrap();

    let window = create_window("image", WindowOptions {
    borderless: true,
    fullscreen: true,
    ..Default::default()
})?;
    println!(" {}",entry.path().display());
    let img = ImageReader::open(entry.path())?.decode()?;

    let rgb = img.to_rgb8();          // convert to RGB format
    let pixel_data = rgb.as_raw();    // &Vec<u8> of raw bytes
    let newimage = ImageView::new(ImageInfo::rgb8(rgb.width(),rgb.height()), pixel_data);
    window.set_image("image-001", newimage)?;
    //destroy window after time elapses
    let times = time::Duration::from_millis(10000);
    let now = time::Instant::now();
    thread::sleep(times);
    window.run_function_wait(|WindowHandle| {WindowHandle.destroy();});
    
    
    
    


  
}


  //Ok(())

}
}