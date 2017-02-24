//! Crate to detect image borders and whitespace using entropy-based image border detection
//! algorithm.
//!
//! # Example
//!
//! ```rust,ignore
//! extern crate enimda;
//!
//! use std::path::Path;
//! use enimda::enimda;
//!
//! let path = Path::new("test.jpg");
//! let borders = enimda(&path, 1.0, 100, 2048, 0.25, 0.5, 1.0, 2048, true)?;
//!
//! println!("{:?}", borders);
//! ```

#![deny(missing_docs)]

extern crate rand;
extern crate image;
extern crate gif;
extern crate gif_dispose;
extern crate image_utils;

use std::path::Path;
use std::error::Error;
use image::ImageFormat;
use image_utils::info;

mod utils;

use utils::{decompose, scan};

/// enimda function result
#[derive(Debug, PartialEq)]
pub struct Borders {
    /// Border offset from the top
    pub top: u32,
    /// Border offset from the right
    pub right: u32,
    /// Border offset from the bottom
    pub bottom: u32,
    /// Border offset from the left
    pub left: u32,
}

/// Scan image and find its borders
///
/// `path` - path to image file
///
/// `fppt` - percent of frames to use in case of animated image, optimization parameter
///
/// `flim` - absolute limit of frames to use in case of animated image, optimization parameter
///
/// `size` - fit image to this size to improve performance, in pixels, optimization parameter
///
/// `depth` - percent of pixels (depth) to use for scanning, use 0.25 if not sure what are you
/// doing
///
/// `thres` - threshold, aggressiveness of algorithm, use 0.5 if not sure what are you doing
///
/// `sppt` - percent of columns to use for scan, optimization parameter
///
/// `slim` - absolute limit of columns to use for scan, optimization parameter
///
/// `deep` - set to true for less performant but accurate and to false for quick but inaccurate,
/// optimization parameter
pub fn enimda(path: &Path,
              fppt: f32,
              flim: u32,
              size: u32,
              depth: f32,
              thres: f32,
              sppt: f32,
              slim: u32,
              deep: bool)
              -> Result<Borders, Box<Error>> {

    let inf = info(path)?;

    let borders = match inf.format {
        ImageFormat::GIF => {
            let ims = decompose(path, inf.width, inf.height, inf.frames, fppt, flim)?;

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
    };

    Ok(Borders {
        top: borders[0],
        right: borders[1],
        bottom: borders[2],
        left: borders[3],
    })
}
