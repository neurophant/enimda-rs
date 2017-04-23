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
//! let borders = enimda(&path, Some(100), Some(2048), Some(50), 0.25, 0.5, true)?;
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

use utils::{scan};

/// Borders location
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
/// `frames` - absolute limit of frames to use in case of animated image, optimization parameter, no limit by default
///
/// `size` - fit image to this size to improve performance, in pixels, optimization parameter, no resize by default
///
/// `columns` - absolute limit of columns to use for scan, optimization parameter, no limit by default
///
/// `depth` - percent of pixels (height) to use for scanning, 0.25 by default
///
/// `threshold` - threshold, aggressiveness of algorithm, 0.5 by default
///
/// `deep` - set to true for less performant but accurate and to false for quick but inaccurate,
/// optimization parameter, true by default
///
/// Returns Borders struct
pub fn enimda(path: &Path,
              frames: Option<u32>,
              size: Option<u32>,
              columns: Option<u32>,
              depth: Option<f32>,
              threshold: Option<f32>,
              deep: Option<bool>)
              -> Result<Borders, Box<Error>> {

    let inf = info(path)?;

    let borders = match inf.format {
        ImageFormat::GIF => {
            vec![0, 0, 0, 0]
//            decompose(path, frames, size, columns, depth, threshold, deep)?
//            let mut borders = vec![0, 0, 0, 0];
//            for im in ims.iter() {
//                let variant = scan(&im, size, depth, threshold, sppt, slim, deep)?;
//                for i in 0..borders.len() {
//                    if variant[i] == 0 || borders[i] < variant[i] {
//                        borders[i] = variant[i];
//                    }
//                }
//            }
        }
        _ => {
            let im = image::open(path)?;
            scan(&im, size, columns, depth, threshold, deep)?
        }
    };

    Ok(Borders {
        top: borders[0],
        right: borders[1],
        bottom: borders[2],
        left: borders[3],
    })
}
