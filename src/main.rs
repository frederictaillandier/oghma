use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::create("foo.bmp")?;

    let buffer: Vec<u8> = vec![
        0x42, 0x4d, //magic start
        0x46, 00, 00, 00, // size of the file
        00, 00, 00, 00, // unused
        0x36, 00, 00, 00, // offset to start of image data
        0x28, 00, 00, 00, // size of the DIB header
        0x2, 00, 00, 00, // width of the image
        0x2, 00, 00, 00, // height of the image
        0x1, 00, // number of color planes
        0x18, 00, // number of bits per pixel
        0, 00, 00, 00, // compression method
        0x10, 00, 00, 00, // size of raw image data
        00, 00, 00, 00, // horizontal resolution
        00, 00, 00, 00, // vertical resolution
        00, 00, 00, 00, // number of colors in the palette
        00, 00, 00, 00, // number of important colors
        0x00, 0x00, 0xFF, // blue
        0x00, 0xFF, 0x00, // green
        0x00, 0x00, // padding
        0xFF, 0x00, 0x00, // red
        0xFF, 0xFF, 0xFF, // white
        0x00, 0x00, // padding
    ];

    file.write_all(&buffer)?;
    Ok(())
}
