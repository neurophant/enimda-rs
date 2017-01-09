use std::collections::HashMap;
use std::cmp::min;

use rand::{thread_rng, Rng};

use image::{GenericImage, DynamicImage, ImageBuffer, Luma, FilterType};
use image::imageops::colorops::grayscale;


pub fn convert(im: &DynamicImage, size: u32) -> (f32, ImageBuffer<Luma<u8>, Vec<u8>>) {
    let mut conv = im.clone();
    let (w, h) = conv.dimensions();

    let mul = match w > size || h > size {
        true => {
            match w > h {
                true => w as f32 / size as f32,
                false => h as f32 / size as f32,
            }
        }
        false => 1.0,
    };

    if mul != 1.0 {
        conv = conv.resize(size, size, FilterType::Lanczos3);
    }

    (mul, grayscale(&conv))
}


pub fn chop(conv: &mut ImageBuffer<Luma<u8>, Vec<u8>>,
            ppt: f32,
            lim: u32)
            -> ImageBuffer<Luma<u8>, Vec<u8>> {
    if ppt < 0.0 || ppt > 1.0 {
        panic!("0.0 <= ppt <= 1.0 expected");
    }

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
    let mut strips: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::new(len as u32, h);
    for (i, row) in rows.iter().enumerate() {
        strips.copy_from(&conv.sub_image(*row, 0, 1, h), i as u32, 0);
    }

    strips
}


pub fn entropy(strip: &mut ImageBuffer<Luma<u8>, Vec<u8>>,
               x: u32,
               y: u32,
               width: u32,
               height: u32)
               -> f32 {
    let sub = strip.sub_image(x, y, width, height);
    let (w, h) = sub.dimensions();
    let len = (w * h) as f32;

    let hm = sub.pixels().fold(HashMap::new(), |mut acc, e| {
        *acc.entry(e.2.data[0]).or_insert(0) += 1;
        acc
    });

    hm.values().fold(0f32, |acc, &x| {
        let f = x as f32 / len;
        acc - (f * f.log2())
    })
}
