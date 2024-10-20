use chrono::Datelike;
use reqwest::blocking;

use super::data_grabber::TrashesSchedule;

fn weekly_update(config: &super::config::Config, schedule: &TrashesSchedule) {
    let client = blocking::Client::new();

    let global_chat_update_txt = format!("The new food master is {}.", schedule.master);

    let group_update = format!(
        "https://api.telegram.org/bot{}/sendMessage?chat_id={}&text={}",
        &config.bot_token,
        &config.flatmates[0],
        global_chat_update_txt.replace("\n", "%0A")
    );
    let _response = client.get(&group_update).send().unwrap();

    let mut master_update_txt = String::new();
    for i in 1..8 {
        let date = chrono::Local::now().naive_local().date() + chrono::Duration::days(i);
        let trashes = schedule.dates.get(&date);
        match trashes {
            None => continue,
            Some(trashes) => {
                let trashes_str = trashes
                    .iter()
                    .fold(String::new(), |acc, trash| format!("{} {}", acc, trash));
                let day_update = format!("{} on {},\n", trashes_str, date.weekday().to_string());
                master_update_txt.push_str(&day_update);
            }
        }
    }
    let master_update_txt = format!(
        "Hello {}!\nYou are the new food master.\nThis week you need to put these trashes in front of the house before 7am.\nHere is the schedule:\n{}Have a nice evening!",
        schedule.master, master_update_txt
    );

    let master_update = format!(
        "https://api.telegram.org/bot{}/sendMessage?chat_id={}&text={}",
        &config.bot_token,
        &config.flatmates[1],
        master_update_txt.replace("\n", "%0A")
    );

    // send
    let _response = client.post(&master_update).send().unwrap();

    println!("url: {}", master_update);
}

fn daily_update(config: &super::config::Config, schedule: &TrashesSchedule) {
    let client = blocking::Client::new();

    let tomorrow = chrono::Local::now().naive_local().date() + chrono::Duration::days(1);
    let trashes = schedule.dates.get(&tomorrow);
    match trashes {
        None => return,
        Some(trashes) => {
            let trashes_str = trashes
                .iter()
                .fold(String::new(), |acc, trash| format!("{} {}", acc, trash));
            let daily_update_txt = format!(
                "Hello {} !\nDon't forget the{} trashes out before tomorrow morning! Have a nice evening!",
                schedule.master, trashes_str
            );

            let daily_update = format!(
                "https://api.telegram.org/bot{}/sendMessage?chat_id={}&text={}",
                &config.bot_token,
                &config.flatmates[1],
                daily_update_txt.replace("\n", "%0A")
            );
            println!("url: {}", daily_update);
            let _response = client.post(&daily_update).send().unwrap();
        }
    }
}

pub fn send_update(config: &super::config::Config, schedule: &TrashesSchedule) {
    if chrono::Local::now().naive_local().weekday() == chrono::Weekday::Sun {
        weekly_update(config, &schedule);
    }
    daily_update(config, schedule);
}
