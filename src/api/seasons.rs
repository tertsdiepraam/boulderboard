use chrono::{DateTime, Utc};
use dioxus::prelude::Props;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SeasonsResponse {
    pub seasons: Vec<ShortSeason>,
}

#[derive(Clone, Deserialize, PartialEq, Props)]
pub struct ShortSeason {
    pub id: u64,
    pub name: String,
    // leagues: Vec<League>,
}

#[derive(Deserialize)]
pub struct Season {
    pub name: String,
    // leagues: Vec<League>,
    pub events: Vec<ShortEvent>,
}

// #[derive(Deserialize)]
// pub struct League {}

#[derive(Clone, Deserialize, PartialEq, Props)]
pub struct ShortEvent {
    pub event: String,
    // location: String,
    pub event_id: u64,
    pub starts_at: DateTime<Utc>,
    pub ends_at: DateTime<Utc>,
    // local_start_date: Date,
    // local_end_date: Date,
    // timezone: ...,
}

#[derive(Deserialize)]
pub struct Event {
    // id: u64,
    // name: String,
    // location: String,
    pub dcats: Vec<ShortCategory>,
    // starts_at: DateTime<Utc>,
    // ends_at: DateTime<Utc>,
    // local_start_date: Date,
    // local_end_date: Date,
    // timezone: ...,
}

#[derive(Clone, Deserialize, PartialEq, Props)]
pub struct ShortCategory {
    pub dcat_name: String,
    category_name: String,
    // discipline_kind: String,
    // status: String,
    pub category_rounds: Vec<CategoryRound>,
}

#[derive(Clone, Deserialize, PartialEq, Props)]
pub struct CategoryRound {
    pub category_round_id: u64,
    pub name: String,
}
