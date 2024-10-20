mod adliswil;
mod we_recycle;
use chrono::{Datelike, NaiveDate};
use core::fmt;
use reqwest::blocking;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug)]
pub enum TrashType {
    WERECYLE,
    NORMAL,
    BIO,
    CARDBOARD,
    PAPER,
    UNKNOWN,
    UNKNOWN2,
    HAZARD,
}

#[derive(Deserialize, Debug)]
struct ChatResult {
    result: ChatInfo,
}

#[derive(Deserialize, Debug)]
struct ChatInfo {
    title: String,
}

impl fmt::Display for TrashType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TrashType::WERECYLE => write!(f, "WeRecycle"),
            TrashType::NORMAL => write!(f, "Normal"),
            TrashType::BIO => write!(f, "Bio"),
            TrashType::CARDBOARD => write!(f, "Cardboard"),
            TrashType::PAPER => write!(f, "Paper"),
            TrashType::UNKNOWN => write!(f, "Unknown"),
            TrashType::UNKNOWN2 => write!(f, "Unknown2"),
            TrashType::HAZARD => write!(f, "Hazard"),
        }
    }
}

#[derive(Debug)]
pub struct TrashesSchedule {
    pub dates: HashMap<NaiveDate, Vec<TrashType>>,
    pub master: String,
}

fn grab_current_food_master_name(config: &super::config::Config) -> String {
    let client = blocking::Client::new();
    // get bot token from env

    let bot_token = &config.bot_token;
    let chat_id = &config.flatmates
        [2 + chrono::Local::now().iso_week().week0() as usize % config.flatmates.len()];

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

pub fn get_trashes(
    config: &super::config::Config,
    from: NaiveDate,
    to: NaiveDate,
) -> TrashesSchedule {
    let mut dates = adliswil::get_trashes(from, to);
    let mut we_recycle = we_recycle::get_trashes(from, to);

    for (date, trashes) in we_recycle.drain() {
        dates.entry(date).or_insert_with(Vec::new).extend(trashes);
    }

    TrashesSchedule {
        dates,
        master: grab_current_food_master_name(config),
    }
}
