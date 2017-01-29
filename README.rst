ENIMDA
======

ENtropy-based IMage border Detection Algorithm: detect border or whitespace offset for every side of image, including GIF (first frame only).

.. image:: https://img.shields.io/crates/v/enimda.svg
    :alt: https://crates.io/crates/enimda
.. image:: https://travis-ci.org/embali/enimda-rs.svg?branch=master
    :alt: https://travis-ci.org/embali/enimda-rs

Algorithm (simplified)
----------------------

For each side of the image starting from top, rotating image counterclockwise to keep side of interest on top:

* Get upper block 25% of image height
* Get lower block with the same height as the upper one
* Calculate entropy for both blocks and their difference
* Make upper block 1px less
* Repeat from p.2 until we hit image edge
* Border is between blocks with entropy difference maximum

.. image:: https://raw.githubusercontent.com/embali/enimda-rs/master/algorithm.gif
    :alt: Sliding from center to edge - searching for maximum entropy difference
    :width: 300
    :height: 300

Example
-----

Find image borders:

.. code-block:: rust

    extern crate image;
    extern crate enimda;

    use std::path::Path;
    use image::GenericImage;
    use enimda::Enimda;

    fn main() {
        let mut im = image::open(&Path::new("source.jpeg")).unwrap();
        let borders = im.enimda(2048, 0.25, 0.5, 1.0, 2048, true).unwrap();
        println!("{:?}", borders);

        let (w, h) = im.dimensions();
        let cropped = im.sub_image(borders[3] + 1,
                       borders[0] + 1,
                       w - (borders[1] + borders[3] + 2),
                       h - (borders[0] + borders[2] + 2))
            .to_image();
        cropped.save("cropped.jpeg").unwrap();
    }

Demo
----

For demo please refer to `ENIMDA Demo <https://github.com/embali/enimda-demo/>`_

Also it lives at `Picture Instruments <http://picinst.com/>`_ as 'Remove borders' instrument
