use std::collections::HashMap;
use std::cmp::min;

use rand::{thread_rng, Rng};

use image::{GenericImage, DynamicImage, ImageBuffer, Luma, FilterType};
use image::imageops::colorops::grayscale;
use image::imageops::rotate270;


pub fn convert(im: &DynamicImage, size: u32) -> (f32, ImageBuffer<Luma<u8>, Vec<u8>>) {
    let mut conv = im.clone();
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


pub fn chop(conv: &mut ImageBuffer<Luma<u8>, Vec<u8>>, ppt: f32, lim: u32)
        -> ImageBuffer<Luma<u8>, Vec<u8>> {
    if ppt == 1.0 || lim == 0 {
        return conv.clone();
    }

    let (w, h) = conv.dimensions();

    let count = (1.0 / ppt).round() as u32;
    let (int, rem) = (w / count, w % count);

    let mut rows = Vec::new();    
    let mut rng = thread_rng();
    for page in 0..int {
        rows.push(rng.gen_range(page * count, (page + 1) * count));
    }
    if rem != 0 {
        rows.push(rng.gen_range(int * count, w));
    }
    rng.shuffle(&mut rows);
    let mut len = rows.len();
    rows.truncate(min(len, lim as usize));

    len = rows.len();
    let mut strips : ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::new(len as u32, h);
    for (i, row) in rows.iter().enumerate() {
        strips.copy_from(&conv.sub_image(*row, 0, 1, h), i as u32, 0);
    }

    strips
}


pub fn entropy(strip: &mut ImageBuffer<Luma<u8>, Vec<u8>>,
               x: u32,
               y: u32,
               width: u32,
               height:u32) -> f32 {
    let sub = strip.sub_image(x, y, width, height);
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


pub fn process(im: &DynamicImage,
               size: u32,
               depth: f32,
               thres: f32,
               ppt: f32,
               lim: u32,
               deep: bool) -> Vec<u32> {
    let (mul, mut conv) = convert(im, size);
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
