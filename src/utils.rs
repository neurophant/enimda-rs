use std::collections::HashMap;
use std::cmp::min;
use std::error::Error;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

use rand::{thread_rng, Rng};

use image;
use image::{GenericImage, DynamicImage, ImageBuffer, Luma, FilterType, ImageFormat, guess_format};
use image::imageops::rotate270;
use image::imageops::colorops::grayscale;
use gif::{Decoder, SetParameter, ColorOutput};


pub fn info(path: &Path) -> Result<(ImageFormat, u32, u32, u32), Box<Error>> {
    let mut im = File::open(path)?;
    let mut buf = [0; 16];
    im.read(&mut buf)?;
    let format = guess_format(&buf)?;

    let im = image::open(path)?;
    let (width, height) = im.dimensions();

    let frames = match format {
        ImageFormat::GIF => {
            let mut decoder = Decoder::new(File::open(path)?);
            decoder.set(ColorOutput::Indexed);
            let mut reader = decoder.read_info().unwrap();
            let mut frames = 0;
            while let Some(_) = reader.read_next_frame().unwrap() {
                frames += 1;
            }
            frames
        }
        _ => 1,
    };

    Ok((format, width, height, frames))
}


fn convert(im: &DynamicImage,
           size: u32)
           -> Result<(f32, ImageBuffer<Luma<u8>, Vec<u8>>), Box<Error>> {
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

    Ok((mul, grayscale(&conv)))
}


fn chop(conv: &mut ImageBuffer<Luma<u8>, Vec<u8>>,
        ppt: f32,
        lim: u32)
        -> Result<ImageBuffer<Luma<u8>, Vec<u8>>, Box<Error>> {
    if ppt < 0.0 || ppt > 1.0 {
        panic!("0.0 <= ppt <= 1.0 expected");
    }

    if ppt == 1.0 || lim == 0 {
        return Ok(conv.clone());
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

    Ok(strips)
}


fn entropy(strip: &mut ImageBuffer<Luma<u8>, Vec<u8>>,
           x: u32,
           y: u32,
           width: u32,
           height: u32)
           -> Result<f32, Box<Error>> {
    let sub = strip.sub_image(x, y, width, height);
    let (w, h) = sub.dimensions();
    let len = (w * h) as f32;

    let hm = sub.pixels().fold(HashMap::new(), |mut acc, e| {
        *acc.entry(e.2.data[0]).or_insert(0) += 1;
        acc
    });

    Ok(hm.values().fold(0f32, |acc, &x| {
        let f = x as f32 / len;
        acc - (f * f.log2())
    }))
}


pub fn scan(im: &DynamicImage,
            size: u32,
            depth: f32,
            thres: f32,
            ppt: f32,
            lim: u32,
            deep: bool)
            -> Result<Vec<u32>, Box<Error>> {
    let (mul, mut conv) = convert(im, size)?;
    let mut borders = Vec::new();

    for side in 0..4 {
        let mut strips = chop(&mut conv, ppt, lim)?;
        let (w, h) = strips.dimensions();
        let height = (depth * h as f32).round() as u32;
        let mut border = 0;

        loop {
            let mut start = border + 1;
            for center in (border + 1)..height {
                if entropy(&mut strips, 0, border, w, center)? > 0.0 {
                    start = center;
                    break;
                }
            }

            let mut sub = 0;
            let mut delta = thres;
            for center in (start..height).rev() {
                let upper = entropy(&mut strips, 0, border, w, center - border)?;
                let lower = entropy(&mut strips, 0, center, w, center - border)?;
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
