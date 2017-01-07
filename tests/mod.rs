#[cfg(test)]

extern crate image;
extern crate enimda;

use std::path::Path;
use enimda::Enimda;


macro_rules! assert_borders {
    ($name:expr, $result:expr) => ({
        let im = image::open(&Path::new(&format!("./tests/images/{}", $name))).unwrap();
        let borders = im.enimda(2048, 0.25, 0.5, 1.0, 2048, true);
        assert_eq!(borders, $result)
    })
}


#[test]
fn tst_bordered_gif() {
    assert_borders!("bordered.gif", vec![22, 86, 16, 86]);
}


#[test]
fn tst_bordered_jpg() {
    assert_borders!("bordered.jpg", vec![4, 4, 4, 4]);
}


#[test]
fn tst_clear_gif() {
    assert_borders!("clear.gif", vec![0, 0, 0, 0]);
}


#[test]
fn tst_clear_jpg() {
    assert_borders!("clear.jpg", vec![0, 0, 0, 0]);
}
