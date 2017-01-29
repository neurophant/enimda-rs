extern crate rand;
extern crate image;

use std::error::Error;

use image::DynamicImage;
use image::imageops::rotate270;

mod utils;

use utils::{convert, chop, entropy};


pub trait Enimda {
    fn enimda(&self,
              size: u32,
              depth: f32,
              thres: f32,
              ppt: f32,
              lim: u32,
              deep: bool)
              -> Result<Vec<u32>, Box<Error>>;
}


impl Enimda for DynamicImage {
    fn enimda(&self,
              size: u32,
              depth: f32,
              thres: f32,
              ppt: f32,
              lim: u32,
              deep: bool)
              -> Result<Vec<u32>, Box<Error>> {
        let (mul, mut conv) = try!(convert(self, size));
        let mut borders = Vec::new();

        for side in 0..4 {
            let mut strips = try!(chop(&mut conv, ppt, lim));
            let (w, h) = strips.dimensions();
            let height = (depth * h as f32).round() as u32;
            let mut border = 0;

            loop {
                let mut start = border + 1;
                for center in (border + 1)..height {
                    if try!(entropy(&mut strips, 0, border, w, center)) > 0.0 {
                        start = center;
                        break;
                    }
                }

                let mut sub = 0;
                let mut delta = thres;
                for center in (start..height).rev() {
                    let upper = try!(entropy(&mut strips, 0, border, w, center - border));
                    let lower = try!(entropy(&mut strips, 0, center, w, center - border));
                    let diff = match lower != 0.0 {
                        true => upper as f32 / lower as f32,
                        false => delta,
                    };
                    if diff < delta && diff < thres {
                        delta = diff;
                        sub = center;
                    }
                }

                if sub == 0 || border == sub {
                    break;
                }

                border = sub;

                if !deep {
                    break;
                }
            }

            borders.push((border as f32 * mul) as u32);

            if side != 3 {
                conv = rotate270(&conv);
            }
        }

        Ok(borders)
    }
}
