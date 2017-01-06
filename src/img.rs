use image::{GenericImage, DynamicImage, ImageBuffer, Luma, FilterType};
use image::imageops::colorops::grayscale;
use image::imageops::rotate270;


use utils::{chop, entropy};


pub struct Image {
    pub source: DynamicImage
}


impl Image {
    fn convert(&self, size: u32) -> (f32, ImageBuffer<Luma<u8>, Vec<u8>>) {
        let mut converted = self.source.clone();
        let (w, h) = converted.dimensions();

        let multiplier = match w > size || h > size {
            true => match w > h {
                true => w as f32 / size as f32,
                false => h as f32 / size as f32
            },
            false => 1.0
        };

        if multiplier != 1.0 {
            converted = converted.resize(size, size, FilterType::Lanczos3);
        }

        (multiplier, grayscale(&converted))
    }

    pub fn scan(&self,
                size: u32,
                depth: f32,
                threshold: f32,
                percentage: f32,
                limit: u32,
                deep: bool) -> Vec<u32> {
        let (multiplier, mut converted) = self.convert(size);
        let mut borders = Vec::new();

        for side in 0..4 {
            let mut strips = chop(&mut converted, percentage, limit);
            let (w, h) = strips.dimensions();
            let height = (depth * h as f32).round() as u32;
            let mut border = 0;

            loop {
                let mut start = border + 1;
                for center in (border + 1)..height {
                    if entropy(&mut strips, 0, border, w, center) > 0.0 {
                        start = center;
                        break;
                    }
                }

                let mut subborder = 0;
                let mut delta = threshold;
                for center in (start..height).rev() {
                    let upper = entropy(&mut strips, 0, border, w, center - border);
                    let lower = entropy(&mut strips, 0, center, w, center - border);
                    let diff = match lower != 0.0 {
                        true => upper as f32 / lower as f32,
                        false => delta
                    };
                    if diff < delta && diff < threshold {
                        delta = diff;
                        subborder = center;
                    } 
                }

                if subborder == 0 || border == subborder {
                    break;
                }

                border = subborder;

                if !deep {
                    break;
                }
            }

            borders.push((border as f32 * multiplier) as u32);

            if side != 3 {
                converted = rotate270(&converted);
            }
        }

        borders
    }
}
