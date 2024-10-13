mod adliswil;
mod we_recycle;
use std::collections::HashMap;

use chrono::NaiveDate;

#[derive(Debug)]
enum TrashType {
    WERECYLE,
    NORMAL,
    BIO,
    CARDBOARD,
    PAPER,
    UNKNOWN,
    UNKNOWN2,
    HAZARD,
}

#[derive(Debug)]
pub struct TrashesSchedule {
    pub dates: HashMap<NaiveDate, Vec<TrashType>>,
    pub master: String,
}

pub fn get_trashes(from: NaiveDate, to: NaiveDate) -> TrashesSchedule {
    let mut dates = adliswil::get_trashes(from, to);
    let mut we_recycle = we_recycle::get_trashes(from, to);

    for (date, trashes) in we_recycle.drain() {
        dates.entry(date).or_insert_with(Vec::new).extend(trashes);
    }

    TrashesSchedule {
        dates,
        master: "Casimir".to_string(),
    }
}
