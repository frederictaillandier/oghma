use chrono::Datelike;

mod config;
mod data_grabber;
mod image_generator;
mod telegram_writer;

fn main() {
    let config_str = std::env::var("GSTALDERCONFIG").unwrap();
    let config = serde_json::from_str::<config::Config>(&config_str).unwrap();
    println!("{:?}", config);

    let today = chrono::Local::now().naive_local().date();
    let weekly = today.weekday() == chrono::Weekday::Sun;
    let until_date = if weekly {
        today + chrono::Duration::days(7)
    } else {
        today + chrono::Duration::days(1)
    };

    let trashes_schedule = data_grabber::get_trashes(&config, today, until_date);
    image_generator::generate(&trashes_schedule);
    telegram_writer::send_update(&config, &trashes_schedule);
}
