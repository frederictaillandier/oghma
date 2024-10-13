mod adliswil;
mod we_recycle;
use chrono::DateTime;

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
pub struct Trash {
    date: DateTime<chrono::Utc>,
    trastype: TrashType,
}

#[derive(Debug)]
pub struct TrashesSchedule {
    dates: Vec<Trash>,
    master: String,
}

pub fn get_trashes(from: DateTime<chrono::Utc>, to: DateTime<chrono::Utc>) -> TrashesSchedule {
    let mut dates = Vec::new();

    dates.append(&mut we_recycle::get_trashes(from, to));
    dates.append(&mut adliswil::get_trashes(from, to));

    TrashesSchedule {
        dates,
        master: "Casimir".to_string(),
    }
}
