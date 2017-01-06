ENIMDA
======

ENtropy-based IMage border Detection Algorithm: detect border or whitespace offset for every side of image, including GIF (first frame only).

Algorithm (simplified)
----------------------

For each side of the image starting from top, rotating image counterclockwise
to keep side of interest on top:

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

Usage
-----

Find if image has any borders:

.. code-block:: rust

    use std::path::Path;

    extern crate image;
    use image::GenericImage;

    extern crate enimda;
    use enimda::Enimda;


    fn main() {
        let source = Enimda::new("source.jpeg");

        let borders = source.scan(2048, 0.25, 0.5, 1.0, 2048, true);
        println!("{:?}", borders);

        let mut im = image::open(&Path::new("source.jpeg")).unwrap();
        let (w, h) = im.dimensions();

        let cropped = im.sub_image(
                output.borders.left + 1,
                output.borders.top + 1,
                w - (output.borders.right + output.borders.left + 2),
                h - (output.borders.top + output.borders.bottom + 2))
            .to_image();
        cropped.save("cropped.jpeg").unwrap();
    }

Demo
----

For demo please refer to `ENIMDA Demo <https://github.com/embali/enimda-demo/>`_

Also it lives at `Picture Instruments <http://picinst.com/>`_ as 'Remove borders' instrument
