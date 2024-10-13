use chrono::Datelike;
use image::{GenericImage, Rgba, RgbaImage};
use reqwest::blocking;
use rusttype::{point, Font, Scale};
use serde::Deserialize;

mod data_grabber;

#[derive(Deserialize, Debug)]
struct ChatResult {
    result: ChatInfo,
}

#[derive(Deserialize, Debug)]
struct ChatInfo {
    title: String,
}

fn grab_current_food_master_name() -> String {
    let client = blocking::Client::new();

    let bot_token = "***";
    let chat_id = "***";

    // url format "https://api.telegram.org/bot{}/getChat?chat_id={}"
    let url = format!(
        "https://api.telegram.org/bot{}/getChat?chat_id={}",
        bot_token, chat_id
    );

    let response = client.get(url).send().unwrap().json::<ChatResult>();

    match response {
        Ok(response) => {
            let mut chat_info = response.result;
            chat_info.title.split_off(17)
        }
        Err(_) => "Error".to_string(),
    }
}

fn write_at(img: &mut RgbaImage, x: i32, y: i32, text: &str) {
    let font_data = include_bytes!("../opensans.ttf");
    let font = Font::try_from_bytes(font_data as &[u8]).unwrap();
    let scale = Scale { x: 50.0, y: 50.0 };
    let position = point(x as f32, y as f32);
    let glyphs: Vec<_> = font.layout(text, scale, position).collect();
    for glyph in glyphs {
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            glyph.draw(|x, y, v| {
                let x = x as i32 + bounding_box.min.x;
                let y = y as i32 + bounding_box.min.y;
                if x >= 0 && x < img.width() as i32 && y >= 0 && y < img.height() as i32 {
                    let alpha = (v * 255.0) as u8;
                    img.blend_pixel(x as u32, y as u32, Rgba([0, 0, 0, alpha]));
                }
            });
        }
    }
}

fn main() {
    let today = chrono::Local::now().naive_local().date();
    let next_week = today + chrono::Duration::weeks(5);

    let trashes_schedule = data_grabber::get_trashes(today, next_week);

    println!("{:?}", trashes_schedule);

    let mut img = RgbaImage::new(800, 480);
    // Set background color (white)
    for pixel in img.pixels_mut() {
        *pixel = Rgba([255, 255, 255, 255]);
    }

    // Write text at position (100, 100)
    let master_name = trashes_schedule.master;
    write_at(&mut img, 100, 100, format!("{},", master_name).as_str());
    write_at(&mut img, 100, 200, "Don't forget to the trashes out!");

    // Save the image as "output.png"
    img.save("output.bmp").unwrap();
}
