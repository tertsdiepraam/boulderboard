use std::fmt::Display;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub enum DisciplineTag {
    Lead,
    Boulder,
    Speed,
}

impl Display for DisciplineTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Lead => f.write_str("Lead"),
            Self::Boulder => f.write_str("Boulder"),
            Self::Speed => f.write_str("Speed")
        }
    }
}

#[derive(Clone, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Active,
    Pending,
    Locked,
    Confirmed,
}

#[derive(Deserialize, Debug)]
pub struct Results {
    // id: u64,
    pub discipline: DisciplineTag,
    // event_id: u64,
    // dcat_id: u64,
    pub event: Option<String>,
    // status: Option<Status>,
    // status_as_of: Option<DateTime<FixedOffset>>,
    pub category: String,
    pub round: String,
    // format: String,
    // routes: Vec<Route>,
    #[serde(default)]
    pub ranking: Vec<RankAthlete>,
    // startlist: Vec<StartAthlete>,
}

// #[derive(Deserialize, Debug)]
// pub struct Route {
//     id: u64,
//     name: String,
//     startlist: String,
// }

#[derive(Deserialize, Debug)]
pub struct Athlete {
    pub athlete_id: u64,
    // name: Option<String>,
    pub firstname: String,
    pub lastname: String,
    // bib: Option<String>,
    #[serde(flatten)]
    pub country: Country,
}

#[derive(Deserialize, Debug)]
pub struct Country {
    pub country: String,
    // flag_url: String,
    // federation_id: u64,
}

// #[derive(Deserialize, Debug)]
// pub struct StartAthlete {
//     #[serde(flatten)]
//     athlete: Athlete,
//     route_start_positions: Vec<RouteStartPosition>,
// }

// #[derive(Deserialize, Debug)]
// pub struct RouteStartPosition {
//     route_name: String,
//     route_id: u64,
//     position: u64,
// }

#[derive(Deserialize, Debug)]
pub struct RankAthlete {
    #[serde(flatten)]
    pub athlete: Athlete,
    // rank: u64,
    // score: String,
    // start_order: Option<u64>,
    pub ascents: Vec<Ascent>,
    pub active: bool,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Ascent {
    // route_id: u64,
    // route_name: String,
    // modified: Option<DateTime<FixedOffset>>,
    pub status: Status,
    #[serde(flatten)]
    pub boulder: Option<BoulderAscent>,
    #[serde(flatten)]
    pub lead: Option<LeadAscent>,
    #[serde(flatten)]
    pub speed: Option<SpeedAscent>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct BoulderAscent {
    pub top: bool,
    pub top_tries: Option<u64>,
    pub zone: bool,
    pub zone_tries: Option<u64>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct LeadAscent {
    pub score: String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct SpeedAscent {
    pub time_ms: u64,
}
