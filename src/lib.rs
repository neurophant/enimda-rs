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
            let frames = paginate(frames, fppt, flim)?;

            let mut decoder = Decoder::new(File::open(path)?);
            decoder.set(ColorOutput::Indexed);
            let mut reader = decoder.read_info().unwrap();
            let mut screen = Screen::new(&reader);

            let mut borders = vec![0, 0, 0, 0];
            let mut i = 0;
            while let Some(frame) = reader.read_next_frame().unwrap() {
                if fppt == 1.0 || flim == 0 || frames.iter().any(|&x| x == i) {
                    screen.blit(&frame)?;
                    let mut buf: Vec<u8> = Vec::new();
                    for pixel in screen.pixels.iter() {
                        buf.push(pixel.r);
                        buf.push(pixel.g);
                        buf.push(pixel.b);
                        buf.push(pixel.a);
                    }
                    let im = ImageRgba8(ImageBuffer::from_raw(width, height, buf).unwrap());
                    let variant = scan(&im, size, depth, thres, sppt, slim, deep)?;
                    for j in 0..borders.len() {
                        if variant[j] == 0 || borders[j] < variant[j] {
                            borders[j] = variant[j];
                        }
                    }
                }

                i += 1;
            }

            borders
        }
        _ => {
            let im = image::open(path)?;
            scan(&im, size, depth, thres, sppt, slim, deep)?
        }
    })
}
