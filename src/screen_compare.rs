use std::io::Cursor;
use image::{self, GenericImageView, RgbaImage, GenericImage};
use screenshots::Screen;
use log::info;

pub fn compare_screen_to_image_file(input_file_path: &String, start_x: i32, start_y: i32) -> f64 {
    let img1 = image::open(input_file_path).expect("File not found");

    let (width, height) = img1.dimensions();

    let screens = Screen::all().unwrap();
    let screen = screens[0];
    info!(target: "commands_debug", "capturer {screen:?}");

    let screen_area = screen.capture_area(start_x, start_y, width, height).unwrap();

    let pixels = screen_area.buffer();


    let mut rgba_screen_image: RgbaImage = RgbaImage::new(width, height);

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