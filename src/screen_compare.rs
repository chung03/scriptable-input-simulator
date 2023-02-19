use std::io::Cursor;
use image::{self, GenericImageView, RgbaImage, GenericImage};
use screenshots::Screen;
use log::{warn, info};

pub fn compare_screen_to_image_file(input_file_path: &String, start_x: i32, start_y: i32, screen_capture_width: u32, screen_capture_height: u32) -> f64 {
    let screens = Screen::all().unwrap();
    let screen = screens[0];
    info!(target: "commands_debug", "capturer {screen:?}");

    let screen_area = screen.capture_area(start_x, start_y, screen_capture_width, screen_capture_height).unwrap();

    let pixels = screen_area.buffer();

    let img1 = image::open(input_file_path).expect("File not found");

    let (width, height) = img1.dimensions();

    if screen_capture_width != width || screen_capture_height != height {
        warn!(target: "commands_debug", "The screen area specified and the image file specified do not match dimensions. Returning 0% match");
        return 0.0;
    }

    let mut rgba_screen_image: RgbaImage = RgbaImage::new(screen_capture_width, screen_capture_height);

    let screenshot = image::io::Reader::new(Cursor::new(pixels))
            .with_guessed_format()
            .unwrap()
            .decode()
            .unwrap();

    rgba_screen_image.copy_from(&screenshot, 0, 0).unwrap();

    let total_pixels = width * height;
    let mut matching_pixels = 0;
    for x in 0 .. width {
        for y in 0.. height {
            if img1.get_pixel(x, y) == *rgba_screen_image.get_pixel(x, y) {
                matching_pixels += 1;
            }
        }
    }

    return matching_pixels as f64/total_pixels as f64;
}