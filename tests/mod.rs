#[cfg(test)]

extern crate image;
extern crate enimda;

use std::path::Path;
use enimda::Enimda;


macro_rules! assert_borders {
    ($name:expr, $has:expr) => ({
        let im = image::open(&Path::new(&format!("./tests/images/{}", $name))).unwrap();
        let borders = im.enimda(256, 0.25, 0.5, 0.1, 32, false).unwrap();
        let sum: u32 = borders.iter().sum();
        assert_eq!(sum != 0, $has)
    })
}


#[test]
fn test_bordered_gif() {
    assert_borders!("bordered.gif", true);
}


#[test]
fn test_bordered_jpg() {
    assert_borders!("bordered.jpg", true);
}


#[test]
fn test_clear_gif() {
    assert_borders!("clear.gif", false);
}


#[test]
fn test_clear_jpg() {
    assert_borders!("clear.jpg", false);
}


#[test]
#[should_panic]
fn test_fail_ppt() {
    assert_borders!("algorithm.gif", true);
}


macro_rules! assert_borders_full {
    ($name:expr, $result:expr) => ({
        let im = image::open(&Path::new(&format!("./tests/images/{}", $name))).unwrap();
        let borders = im.enimda(2048, 0.25, 0.5, 1.0, 2048, true).unwrap();
        assert_eq!(borders, $result)
    })
}


#[test]
#[ignore]
fn test_bordered_gif_full() {
    assert_borders_full!("bordered.gif", vec![41, 0, 39, 0]);
}


#[test]
#[ignore]
fn test_bordered_jpg_full() {
    assert_borders_full!("bordered.jpg", vec![4, 4, 4, 4]);
}


#[test]
#[ignore]
fn test_clear_gif_full() {
    assert_borders_full!("clear.gif", vec![0, 0, 0, 0]);
}


#[test]
#[ignore]
fn test_clear_jpg_full() {
    assert_borders_full!("clear.jpg", vec![0, 0, 0, 0]);
}
