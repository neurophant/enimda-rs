#[cfg(test)]

extern crate image;
extern crate enimda;

use std::path::Path;
use enimda::{Borders, enimda};

macro_rules! assert_borders {
    ($name:expr, $has:expr) => ({
        let borders = enimda(&Path::new(&format!("./tests/images/{}", $name)),
                             0.1, 10, 256, 0.25, 0.5, 0.1, 32, false).unwrap();
        let sum: u32 = borders.top + borders.right + borders.bottom + borders.left;
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
        let borders = enimda(&Path::new(&format!("./tests/images/{}", $name)),
                             1.0, 1000, 2048, 0.25, 0.5, 1.0, 2048, true).unwrap();
        assert_eq!(borders, $result)
    })
}

#[test]
#[ignore]
fn test_bordered_gif_full() {
    assert_borders_full!("bordered.gif", Borders { top: 41, right: 0, bottom: 39, left: 0 });
}

#[test]
#[ignore]
fn test_bordered_jpg_full() {
    assert_borders_full!("bordered.jpg", Borders { top: 4, right: 4, bottom: 4, left: 4 });
}

#[test]
#[ignore]
fn test_clear_gif_full() {
    assert_borders_full!("clear.gif", Borders { top: 0, right: 0, bottom: 0, left: 0 });
}

#[test]
#[ignore]
fn test_clear_jpg_full() {
    assert_borders_full!("clear.jpg", Borders { top: 0, right: 0, bottom: 0, left: 0 });
}
