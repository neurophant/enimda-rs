extern crate rand;
extern crate image;

use std::path::Path;
use std::string::String;

use image::DynamicImage;


mod utils;

use utils::process;


pub trait Enimda {
    fn scan(src: &Self, size: u32, depth: f32, thres: f32, ppt: f32, lim: u32, deep: bool)
        -> Vec<u32>;
}


impl Enimda for str {
    fn scan(src: &str, size: u32, depth: f32, thres: f32, ppt: f32, lim: u32, deep: bool)
            -> Vec<u32> {
        process(&image::open(&Path::new(src)).unwrap(), size, depth, thres, ppt, lim, deep)
    }
}


impl Enimda for String {
    fn scan(src: &String, size: u32, depth: f32, thres: f32, ppt: f32, lim: u32, deep: bool)
            -> Vec<u32> {
        process(&image::open(&Path::new(src)).unwrap(), size, depth, thres, ppt, lim, deep)
    }
}


impl Enimda for DynamicImage {
    fn scan(src: &DynamicImage, size: u32, depth: f32, thres: f32, ppt: f32, lim: u32, deep: bool)
            -> Vec<u32> {
        process(src, size, depth, thres, ppt, lim, deep)
    }
}
