use chrono::DateTime;
use reqwest::blocking;
use serde::{Deserialize, Serialize};

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

pub fn get_trashes(from: DateTime<chrono::Utc>, to: DateTime<chrono::Utc>) -> Vec<super::Trash> {
    let client = blocking::Client::new();

    let url = format!(
        "https://adliswil.entsorglos.swiss/backend/widget/calendar-dates/{}/",
        from.format("%m-%Y")
    );

    let response = client.get(url).send();

    match response {
        Ok(r) => {
            let mut result = Vec::new();

            let wastes: Result<AdliswilWaste, serde_json::Error> =
                serde_json::from_str(&r.text().unwrap());

            match wastes {
                Ok(waste_info) => {
                    for event in waste_info.results.events {
                        if event.date >= from && event.date <= to {
                            let trastype = match event.waste_type {
                                1 => super::TrashType::NORMAL,
                                2 => super::TrashType::BIO,
                                3 => super::TrashType::CARDBOARD,
                                4 => super::TrashType::PAPER,
                                _ => super::TrashType::UNKNOWN,
                            };
                            result.push(super::Trash {
                                date: event.date,
                                trastype,
                            });
                        }
                    }
                    return result;
                }
                Err(e) => {
                    println!("error {}", e);
                    return Vec::new();
                }
            };
        }
        Err(e) => {
            println!("error {}", e);
            return Vec::new();
        }
    }
}
