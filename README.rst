ENIMDA
======

Entropy-based image border detection algorithm: detect border or whitespace offset for every side of image,
supports animated GIFs.

|crates| |travisci|

.. |crates| image:: https://img.shields.io/crates/v/enimda.svg
    :target: https://crates.io/crates/enimda
    :alt: latest version at crates.io
.. |travisci| image:: https://travis-ci.org/embali/enimda-rs.svg?branch=master
    :target: https://travis-ci.org/embali/enimda-rs
    :alt: travis ci build status

Documentation
-------------

`https://docs.rs/enimda/ <https://docs.rs/enimda/>`_

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
-------

Find image borders:

.. code-block:: rust

    extern crate enimda;

    use std::path::Path;
    use enimda::enimda;

    fn main() {
        let path = Path::new("source.jpeg");
        let borders = enimda(&path, 1.0, 100, 2048, 0.25, 0.5, 1.0, 2048, true).unwrap();
        println!("{:?}", borders);
    }

Demo
----

For demo please refer to `ENIMDA Demo <https://github.com/embali/enimda-demo/>`_

Also it lives at `Picture Instruments <http://picinst.com/>`_ as 'Remove borders' instrument

Tests
-----

.. code-block:: bash

    cargo test
