use clap::{Arg, ArgAction, Command};

mod config;
mod data_grabber;
mod image_generator;
mod telegram_writer;

fn get_args() -> clap::ArgMatches {
    // get version from Cargo.toml
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    let author = env!("CARGO_PKG_AUTHORS");
    let about = env!("CARGO_PKG_DESCRIPTION");

    let app = Command::new(name)
        .version(version)
        .author(author)
        .about(about)
        // a boolean option with a short and long flag
        .arg(
            Arg::new("weekly_mode")
                .short('w')
                .long("weekly")
                .help("Run in weekly mode")
                .action(ArgAction::SetTrue),
        )
        .get_matches();
    app
}

fn main() {
    let config_str = std::env::var("GSTALDERCONFIG").unwrap();
    let config = serde_json::from_str::<config::Config>(&config_str).unwrap();

    let today = chrono::Local::now().naive_local().date();
    let args = get_args();
    let weekly = args.get_flag("weekly_mode");

    let until_date = if weekly {
        println!("Running in weekly mode");
        today + chrono::Duration::days(7)
    } else {
        println!("Running in daily mode");
        today + chrono::Duration::days(1)
    };

    let trashes_schedule = data_grabber::get_trashes(&config, today, until_date);
    image_generator::generate(&trashes_schedule);
    telegram_writer::send_update(&config, &trashes_schedule, weekly);
}
