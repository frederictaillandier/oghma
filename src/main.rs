mod config;
mod data_grabber;
mod image_generator;

fn main() {
    let config_str = std::env::var("GSTALDERCONFIG").unwrap();
    let config = serde_json::from_str::<config::Config>(&config_str).unwrap();
    println!("{:?}", config);

    let today = chrono::Local::now().naive_local().date();
    let tomorrow = today + chrono::Duration::days(1);

    let trashes_schedule = data_grabber::get_trashes(&config, today, tomorrow);
    println!("{:?}", trashes_schedule);
    image_generator::generate(trashes_schedule);
}
