extern crate rand;
extern crate image;
extern crate gif;
extern crate gif_dispose;

use std::path::Path;
use std::fs::File;
use std::error::Error;

use image::{ImageRgba8, ImageFormat, ImageBuffer};
use gif::{Decoder, SetParameter, ColorOutput};
use gif_dispose::Screen;


mod utils;

use utils::{info, scan};


pub fn enimda(path: &Path,
              fppt: f32,
              flim: u32,
              size: u32,
              depth: f32,
              thres: f32,
              sppt: f32,
              slim: u32,
              deep: bool)
              -> Result<Vec<u32>, Box<Error>> {

    let (format, width, height, frames) = info(path)?;
    Ok(match format {
        ImageFormat::GIF => {
            let mut variants = Vec::new();
            let mut decoder = Decoder::new(File::open(path)?);
            decoder.set(ColorOutput::Indexed);
            let mut reader = decoder.read_info().unwrap();
            let mut screen = Screen::new(&reader);
            while let Some(frame) = reader.read_next_frame().unwrap() {
                screen.blit(&frame)?;
                let mut buf: Vec<u8> = Vec::new();
                for i in 0..screen.pixels.len() {
                    buf.push(screen.pixels[i].r);
                    buf.push(screen.pixels[i].g);
                    buf.push(screen.pixels[i].b);
                    buf.push(screen.pixels[i].a);
                }
                let im = ImageRgba8(ImageBuffer::from_raw(width, height, buf).unwrap());
                variants.push(scan(&im, size, depth, thres, sppt, slim, deep)?);
            }
            println!("{:?}", variants);
            //
            vec![0, 0, 0, 0]
        }
        _ => {
            let im = image::open(path)?;
            scan(&im, size, depth, thres, sppt, slim, deep)?
        }
    })
}
