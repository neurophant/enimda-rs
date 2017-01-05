use std::path::Path;
use std::collections::HashMap;
use std::cmp::min;
use std::string::String;
use std::fmt;

extern crate rand;
use rand::{thread_rng, Rng};

extern crate image;
use image::{GenericImage, DynamicImage, ImageBuffer, Luma, FilterType};
use image::imageops::colorops::grayscale;
use image::imageops::rotate270;


pub struct Image {
    pub source: DynamicImage
}


impl fmt::Debug for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.source.dimensions())
    }
}


pub struct Service {
    pub converted: ImageBuffer<Luma<u8>, Vec<u8>>,
    pub multiplier: f32,
    pub strips: Vec<ImageBuffer<Luma<u8>, Vec<u8>>>
}


impl fmt::Debug for Service {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {}", self.converted.dimensions(), self.multiplier)
    }
}


pub struct Borders {
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
    pub left: u32
}


impl fmt::Debug for Borders {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", (self.top, self.right, self.bottom, self.left))
    }
}


pub struct Output {
    pub service: Service,
    pub borders: Borders
}


impl fmt::Debug for Output {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {:?}", self.service, self.borders)
    }
}


fn chop(image: &mut ImageBuffer<Luma<u8>, Vec<u8>>, percentage: f32, limit: u32)
        -> ImageBuffer<Luma<u8>, Vec<u8>> {
    if percentage == 1.0 || limit == 0 {
        return image.clone();
    }

    let (w, h) = image.dimensions();
    let paginate = (1.0 / percentage).round() as u32;
    let (pages, rem) = (w / paginate, w % paginate);
    let mut rows = Vec::new();    
    let mut rng = thread_rng();
    for p in 0..pages {
        rows.push(rng.gen_range(p * paginate, (p + 1) * paginate));
    }
    if rem != 0 {
        rows.push(rng.gen_range(pages * paginate, w));
    }
    rng.shuffle(&mut rows);
    let len = rows.len();
    rows.truncate(min(len, limit as usize));
    let len = rows.len();
    let mut strips : ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::new(len as u32, h);
    for (i, r) in rows.iter().enumerate() {
        strips.copy_from(&image.sub_image(*r, 0, 1, h), i as u32, 0);
    }

    strips
}


fn entropy(image: &mut ImageBuffer<Luma<u8>, Vec<u8>>,
           x: u32,
           y: u32,
           width: u32,
           height:u32) -> f32 {
    let sub = image.sub_image(x, y, width, height);
    let (w, h) = sub.dimensions();
    let len = (w * h) as f32;
    let hm = sub.pixels().fold(
        HashMap::new(),
        |mut acc, e| {
            *acc.entry(e.2.data[0]).or_insert(0) += 1;
            acc
        }
    );
    hm.values().fold(
        0f32,
        |acc, &x| {
            let f = x as f32 / len;
            acc - (f * f.log2())
        }
    )
}


impl Image {
    fn convert(&self, size: u32, percentage: f32, limit: u32) -> Service {
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
        let converted = grayscale(&converted);

        let mut rotated = converted.clone();
        let mut strips = Vec::new();
        for side in 0..4 {
            strips.push(chop(&mut rotated, percentage, limit));
            if side != 3 {
                rotated = rotate270(&rotated);
            }
        }

        Service { converted: converted, multiplier: multiplier, strips: strips }
    }

    pub fn scan(&self,
            size: u32,
            depth: f32,
            threshold: f32,
            percentage: f32,
            stripes: u32,
            deep: bool) -> Output {
        let service = self.convert(size, percentage, stripes);
        let mut borders = Vec::new();

        for strip in service.strips.iter() {
            let mut strip = strip.clone();
            let (w, h) = strip.dimensions();
            let height = (depth * h as f32).round() as u32;
            let mut border = 0;
            loop {
                let mut start = border + 1;
                for s in (border + 1)..height {
                    if entropy(&mut strip, 0, border, w, s) > 0.0 {
                        start = s;
                        break;
                    }
                }
                let mut subborder = 0;
                let mut delta = threshold;
                for center in (start..height).rev() {
                    let upper = entropy(&mut strip, 0, border, w, center - border);
                    let lower = entropy(&mut strip, 0, center, w, center - border);
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
            borders.push((border as f32 * service.multiplier) as u32);
        }

        Output {
            service: service,
            borders: Borders { 
                top: borders[0],
                right: borders[1],
                bottom: borders[2],
                left: borders[3]
            }
        }
    }
}


pub trait Enimda {
    fn new(src: &Self) -> Image;
}


impl Enimda for str {
    fn new(src: &str) -> Image {
        Image { source: image::open(&Path::new(src)).unwrap() }
    }
}


impl Enimda for String {
    fn new(src: &String) -> Image {
        Image { source: image::open(&Path::new(src)).unwrap() }
    }
}


impl Enimda for DynamicImage {
    fn new(src: &DynamicImage) -> Image {
        Image { source: src.clone() }
    }
}
