extern crate image;

use std::env;
use std::fs::File;
use std::path::Path;

use image::GenericImage;

fn main() {
    let file = if env::args().count() == 2 {
        env::args().nth(1).unwrap()
    } else {
        panic!("Please enter a file")
    };

    let im = image::open(&Path::new(&file)).unwrap();
    println!("dimensions {:?}", im.dimensions());
    println!("{:?}", im.color());
    let fout = &mut File::create(&Path::new(&format!("fix_{}", file))).unwrap();
    im.save(fout, image::BMP).unwrap();
}
