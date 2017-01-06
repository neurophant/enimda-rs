extern crate rand;
extern crate image;

use std::path::Path;
use std::string::String;

use image::DynamicImage;


mod utils;
mod img;

use img::Image;


pub trait Enimda {
    fn new(src: &Self) -> Image;
}


impl Enimda for str {
    fn new(src: &str) -> Image {
        Image { src: image::open(&Path::new(src)).unwrap() }
    }
}


impl Enimda for String {
    fn new(src: &String) -> Image {
        Image { src: image::open(&Path::new(src)).unwrap() }
    }
}


impl Enimda for DynamicImage {
    fn new(src: &DynamicImage) -> Image {
        Image { src: src.clone() }
    }
}
