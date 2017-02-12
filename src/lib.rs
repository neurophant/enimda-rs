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

use utils::{info, paginate, scan};


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
            if fppt < 0.0 || fppt > 1.0 {
                panic!("0.0 <= ppt <= 1.0 expected");
            }

            let mut decoder = Decoder::new(File::open(path)?);
            decoder.set(ColorOutput::Indexed);
            let mut reader = decoder.read_info().unwrap();
            let mut screen = Screen::new(&reader);

            let iframes = paginate(frames, fppt, flim)?;
            let mut variants = Vec::new();
            let mut iframe = 0;
            while let Some(frame) = reader.read_next_frame().unwrap() {
                if iframes.iter().any(|&x| x == iframe) {
                    screen.blit(&frame)?;
                    let mut buf: Vec<u8> = Vec::new();
                    for pixel in screen.pixels.iter() {
                        buf.push(pixel.r);
                        buf.push(pixel.g);
                        buf.push(pixel.b);
                        buf.push(pixel.a);
                    }
                    let im = ImageRgba8(ImageBuffer::from_raw(width, height, buf).unwrap());
                    variants.push(scan(&im, size, depth, thres, sppt, slim, deep)?);
                }

                iframe += 1;
            }

            let mut borders = vec![0, 0, 0, 0];
            for variant in variants.iter() {
                for i in 0..variant.len() {
                    if variant[i] == 0 || borders[i] < variant[i] {
                        borders[i] = variant[i];
                    }
                }
            }

            borders
        }
        _ => {
            let im = image::open(path)?;
            scan(&im, size, depth, thres, sppt, slim, deep)?
        }
    })
}
