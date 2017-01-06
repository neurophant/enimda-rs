use std::collections::HashMap;
use std::cmp::min;

extern crate rand;
use rand::{thread_rng, Rng};

extern crate image;
use image::{GenericImage, ImageBuffer, Luma};


pub fn chop(image: &mut ImageBuffer<Luma<u8>, Vec<u8>>, percentage: f32, limit: u32)
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


pub fn entropy(image: &mut ImageBuffer<Luma<u8>, Vec<u8>>,
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
