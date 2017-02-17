extern crate rand;
extern crate image;
extern crate gif;
extern crate gif_dispose;

use std::path::Path;
use std::error::Error;

use image::ImageFormat;


mod utils;

use utils::{info, decompose, scan};


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
            let ims = decompose(path, width, height, frames, fppt, flim)?;

            let mut borders = vec![0, 0, 0, 0];
            for im in ims.iter() {
                let variant = scan(&im, size, depth, thres, sppt, slim, deep)?;
                for i in 0..borders.len() {
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
