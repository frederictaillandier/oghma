use chrono::{DateTime, NaiveDate};
use reqwest::blocking;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::TrashType;

#[derive(Serialize, Deserialize, Debug)]
struct Event {
    date: DateTime<chrono::Utc>,
    waste_type: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct AdliswilWasteInfo {
    events: Vec<Event>,
}

#[derive(Serialize, Deserialize, Debug)]
struct AdliswilWaste {
    results: AdliswilWasteInfo,
}

pub fn get_trashes(from: NaiveDate, to: NaiveDate) -> HashMap<NaiveDate, Vec<TrashType>> {
    let client = blocking::Client::new();

    let url = format!(
        "https://adliswil.entsorglos.swiss/backend/widget/calendar-dates/{}/",
        from.format("%m-%Y")
    );

    let response = client.get(url).send();

    match response {
        Ok(r) => {
            let mut result: HashMap<NaiveDate, Vec<TrashType>> = HashMap::new();

            let wastes: Result<AdliswilWaste, serde_json::Error> =
                serde_json::from_str(&r.text().unwrap());

            match wastes {
                Ok(waste_info) => {
                    for event in waste_info.results.events {
                        let naive = event.date.date_naive();
                        if naive >= from && naive < to {
                            let trastype = match event.waste_type {
                                1 => super::TrashType::NORMAL,
                                2 => super::TrashType::BIO,
                                3 => super::TrashType::CARDBOARD,
                                4 => super::TrashType::PAPER,
                                _ => super::TrashType::UNKNOWN,
                            };
                            result
                                .entry(event.date.date_naive())
                                .or_insert_with(Vec::new)
                                .push(trastype);
                        }
                    }
                    return result;
                }
                Err(e) => {
                    println!("error {}", e);
                    return HashMap::new();
                }
            };
        }
        Err(e) => {
            println!("error {}", e);
            return HashMap::new();
        }
    }
}
