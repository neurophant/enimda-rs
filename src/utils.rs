use std::collections::{HashSet, HashMap};
use std::iter::FromIterator;
use std::cmp::min;
use std::error::Error;
use rand::{thread_rng, Rng};
use image::{GenericImage, DynamicImage, ImageBuffer, Luma, FilterType};
use image::imageops::rotate270;
use image::imageops::colorops::grayscale;

pub fn slice(count: u32, limit: u32) -> Result<HashSet<u32>, Box<Error>> {
    let mut indexes: Vec<u32> = (0..count).collect();

    if limit > 0 && limit < count {
        let mut rng = thread_rng();
        rng.shuffle(&mut indexes);
        let len = indexes.len();
        indexes.truncate(min(len, limit as usize));
    }

    Ok(HashSet::from_iter(indexes.iter().cloned()))
}

fn convert(im: &DynamicImage,
           size: u32)
           -> Result<(f32, ImageBuffer<Luma<u8>, Vec<u8>>), Box<Error>> {
    let mut conv = im.clone();
    let (w, h) = conv.dimensions();

    let mul = match size > 0 && (w > size || h > size) {
        true => {
            conv = conv.resize(size, size, FilterType::Lanczos3);
            match w > h {
                true => w as f32 / size as f32,
                false => h as f32 / size as f32,
            }
        }
        false => 1.0,
    };

    Ok((mul, grayscale(&conv)))
}

fn chop(conv: &mut ImageBuffer<Luma<u8>, Vec<u8>>,
        limit: u32)
        -> Result<ImageBuffer<Luma<u8>, Vec<u8>>, Box<Error>> {
    Ok(match limit > 0 {
        true => {
            let (w, h) = conv.dimensions();
            let rows = slice(w, limit)?;
            let mut strips: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::new(rows.len() as u32, h);
            for (i, row) in rows.iter().enumerate() {
                strips.copy_from(&conv.sub_image(*row, 0, 1, h), i as u32, 0);
            }
            strips
        }
        false => conv.clone(),
    })
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
            size: Option<u32>,
            columns: Option<u32>,
            depth: Option<f32>,
            threshold: Option<f32>,
            deep: Option<bool>)
            -> Result<Vec<u32>, Box<Error>> {
    let threshold = threshold.unwrap_or(0.5);
    let (mul, mut conv) = convert(im, size.unwrap_or(0))?;
    let mut borders = Vec::new();
    let depth = depth.unwrap_or(0.25);
    let deep = deep.unwrap_or(true);

    for side in 0..4 {
        let mut strips = chop(&mut conv, columns.unwrap_or(0))?;
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
            let mut delta = threshold;
            for center in (start..height).rev() {
                let upper = entropy(&mut strips, 0, border, w, center - border)?;
                let lower = entropy(&mut strips, 0, center, w, center - border)?;
                let diff = match lower != 0.0 {
                    true => upper as f32 / lower as f32,
                    false => delta,
                };
                if diff < delta && diff < threshold {
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
