use image::{GenericImage, DynamicImage, ImageBuffer, Luma, FilterType};
use image::imageops::colorops::grayscale;
use image::imageops::rotate270;


use utils::{chop, entropy};


pub struct Image {
    pub src: DynamicImage
}


impl Image {
    fn convert(&self, size: u32) -> (f32, ImageBuffer<Luma<u8>, Vec<u8>>) {
        let mut conv = self.src.clone();
        let (w, h) = conv.dimensions();

        let mul = match w > size || h > size {
            true => match w > h {
                true => w as f32 / size as f32,
                false => h as f32 / size as f32
            },
            false => 1.0
        };

        if mul != 1.0 {
            conv = conv.resize(size, size, FilterType::Lanczos3);
        }

        (mul, grayscale(&conv))
    }

    pub fn scan(&self,
                size: u32,
                depth: f32,
                thres: f32,
                ppt: f32,
                lim: u32,
                deep: bool) -> Vec<u32> {
        let (mul, mut conv) = self.convert(size);
        let mut borders = Vec::new();

        for side in 0..4 {
            let mut strips = chop(&mut conv, ppt, lim);
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

                let mut sub = 0;
                let mut delta = thres;
                for center in (start..height).rev() {
                    let upper = entropy(&mut strips, 0, border, w, center - border);
                    let lower = entropy(&mut strips, 0, center, w, center - border);
                    let diff = match lower != 0.0 {
                        true => upper as f32 / lower as f32,
                        false => delta
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

        borders
    }
}
