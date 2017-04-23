#[cfg(test)]

extern crate image;
extern crate enimda;

use std::path::Path;
use enimda::{Borders, enimda};

macro_rules! assert_borders {
    ($name:expr, $has:expr) => ({
        let borders = enimda(&Path::new(&format!("./tests/images/{}", $name)),
                             Some(10), Some(256), Some(32), Some(0.25), Some(0.5),
                             Some(false)).unwrap();
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

macro_rules! assert_borders_full {
    ($name:expr, $result:expr) => ({
        let borders = enimda(&Path::new(&format!("./tests/images/{}", $name)),
                             None, None, None, None, None, None).unwrap();
        assert_eq!(borders, $result)
    })
}

#[test]
fn test_bordered_gif_full() {
    assert_borders_full!("bordered.gif",
                         Borders {
                             top: 41,
                             right: 0,
                             bottom: 39,
                             left: 0,
                         });
}

#[test]
fn test_bordered_jpg_full() {
    assert_borders_full!("bordered.jpg",
                         Borders {
                             top: 4,
                             right: 4,
                             bottom: 4,
                             left: 4,
                         });
}

#[test]
fn test_clear_gif_full() {
    assert_borders_full!("clear.gif",
                         Borders {
                             top: 0,
                             right: 0,
                             bottom: 0,
                             left: 0,
                         });
}

#[test]
fn test_clear_jpg_full() {
    assert_borders_full!("clear.jpg",
                         Borders {
                             top: 0,
                             right: 0,
                             bottom: 0,
                             left: 0,
                         });
}
