use super::data_grabber::TrashesSchedule;
use image::{Pixel, Rgba, RgbaImage};
use rusttype::{point, Font, Scale};

fn write_at(img: &mut RgbaImage, x: i32, y: i32, text: &str) {
    let font_data = include_bytes!("../opensans.ttf");
    let font = Font::try_from_bytes(font_data as &[u8]).unwrap();
    let scale = Scale { x: 32.0, y: 32.0 };
    let position = point(x as f32, y as f32);
    let glyphs: Vec<_> = font.layout(text, scale, position).collect();
    for glyph in glyphs {
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            glyph.draw(|x, y, v| {
                let x = x + bounding_box.min.x as u32;
                let y = y + bounding_box.min.y as u32;
                if x < img.width() && y < img.height() {
                    let alpha = (v * 255.0) as u8;
                    let pix = img.get_pixel_mut(x, y);

                    pix.blend(&Rgba([0, 0, 0, alpha]));
                }
            });
        }
    }
}

fn write_text(img: &mut RgbaImage, x: i32, y: i32, text: String) {
    //split the text in lines
    let lines = text.split("\n");
    for (i, line) in lines.enumerate() {
        write_at(img, x, y + i as i32 * 32, line);
    }
}

pub fn generate(schedule: &TrashesSchedule) {
    let tomorrow = chrono::Local::now().naive_local().date() + chrono::Duration::days(1);
    match schedule.dates.get(&tomorrow) {
        Some(tomorrow_trashes) => {
            let mut img = RgbaImage::new(400, 800);
            // Set background color (white)
            for pixel in img.pixels_mut() {
                *pixel = Rgba([255, 255, 255, 255]);
            }

            let trashes_text = tomorrow_trashes
                .iter()
                .fold(String::new(), |acc, trash| format!("{}{}", acc, trash));

            // Write text at position (100, 100)
            let master_name = &schedule.master;
            write_text(
                &mut img,
                10,
                100,
                format!(
                    "{},\nDon't forget to take out the\n{}\nbefore tomorrow 7am.",
                    master_name, trashes_text
                ),
            );
            img.save("output.bmp").unwrap();
        }
        None => {
            let img = image::open("assets/backgrounds/12/2024-10-19_20-22-10_3722.png")
                .unwrap()
                .resize(400, 800, image::imageops::FilterType::Nearest)
                .to_rgba8();
            img.save("output.bmp").unwrap();
        }
    }

    // Save the image as "output.png"
}
