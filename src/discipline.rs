//! Types that define a discpline
use crate::api;
use dioxus::prelude::LazyNodes;

pub use boulder::*;
pub use lead::*;
pub use speed::*;

pub trait Discipline {
    type Ascent: Ascent;
    type Score: Score<Ascent = Self::Ascent>;
}

pub trait Ascent: TryFrom<api::result::Ascent> {
    fn render(&self) -> LazyNodes;
}

pub trait Score: Ord {
    type Ascent: Ascent;

    fn render(&self) -> LazyNodes;
    fn calculate(start_order: u64, ascents: &[Self::Ascent]) -> Self;
}

mod lead {
    use std::cmp::Ordering;

    use super::{Ascent, Discipline, Score};
    use crate::api;
    use dioxus::prelude::{rsx, LazyNodes};
    use serde::Deserialize;

    #[derive(Debug)]
    pub struct Lead;

    impl Discipline for Lead {
        type Ascent = LeadAscent;
        type Score = LeadScore;
    }

    #[derive(Debug, Deserialize)]
    pub struct LeadAscent {
        score: String,
    }

    impl Ascent for LeadAscent {
        fn render(&self) -> LazyNodes {
            rsx! { "{self.score}" }
        }
    }

    impl TryFrom<api::result::Ascent> for LeadAscent {
        type Error = ();

        fn try_from(value: api::result::Ascent) -> Result<Self, Self::Error> {
            if let Some(api::result::LeadAscent { score }) = value.lead {
                Ok(Self { score })
            } else {
                Err(())
            }
        }
    }

    #[derive(PartialEq, Eq, Debug, Clone)]
    pub struct LeadScore(u64);

    impl Score for LeadScore {
        type Ascent = LeadAscent;
        fn render(&self) -> LazyNodes {
            rsx! { self.0.to_string() }
        }

        fn calculate(_: u64, _: &[Self::Ascent]) -> Self {
            // FIXME: Implement calculation
            LeadScore(0)
        }
    }

    impl PartialOrd for LeadScore {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for LeadScore {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.0.cmp(&other.0)
        }
    }
}

mod boulder {
    use dioxus::prelude::*;
    use serde::Deserialize;
    use std::cmp::Ordering;

    use crate::api::{self, result::Status};

    use super::{Ascent, Discipline, Score};

    #[derive(Debug)]
    pub struct Boulder;

    impl Discipline for Boulder {
        type Ascent = BoulderAscent;
        type Score = BoulderScore;
    }

    #[derive(PartialEq, Eq, Clone, Debug, Deserialize)]
    pub struct BoulderAscent {
        top: bool,
        top_tries: u64,
        zone: bool,
        zone_tries: u64,
        status: Status,
    }

    impl Ascent for BoulderAscent {
        fn render(&self) -> LazyNodes {
            let fill_class = if self.top && self.top_tries == 1 {
                "ascent-flash"
            } else if self.top {
                "ascent-full"
            } else if self.zone {
                "ascent-half"
            } else {
                "ascent-empty"
            };

            let status_class = if self.status == Status::Active {
                "ascent-active"
            } else if self.status == Status::Pending {
                "ascent-pending"
            } else {
                ""
            };

            rsx! { div { class: "ascent {fill_class} {status_class}" } }
        }
    }

    impl TryFrom<api::result::Ascent> for BoulderAscent {
        type Error = ();

        fn try_from(value: api::result::Ascent) -> Result<Self, Self::Error> {
            if let Some(api::result::BoulderAscent {
                top,
                top_tries,
                zone,
                zone_tries,
            }) = value.boulder
            {
                Ok(Self {
                    top,
                    top_tries: top_tries.unwrap_or_default(),
                    zone,
                    zone_tries: zone_tries.unwrap_or_default(),
                    status: value.status,
                })
            } else {
                Err(())
            }
        }
    }

    #[derive(PartialEq, Eq, Clone, Debug)]
    pub struct BoulderScore {
        ascents: Vec<BoulderAscent>,
        tops: u64,
        zones: u64,
        top_tries: u64,
        start_order: u64,
    }

    impl Score for BoulderScore {
        type Ascent = BoulderAscent;

        fn render(&self) -> LazyNodes {
            rsx! {
                div { class: "score", self.tops.to_string() }
                div { class: "score", self.zones.to_string() }
                div { class: "score", self.top_tries.to_string() }
            }
        }

        fn calculate(start_order: u64, ascents: &[Self::Ascent]) -> Self {
            Self {
                ascents: ascents.to_vec(),
                tops: ascents.iter().filter(|a| a.top).count() as u64,
                zones: ascents.iter().filter(|a| a.zone).count() as u64,
                top_tries: ascents
                    .iter()
                    .map(|a| if a.top { a.top_tries } else { 0 })
                    .sum(),
                start_order,
            }
        }
    }

    impl PartialOrd for BoulderScore {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for BoulderScore {
        fn cmp(&self, other: &Self) -> Ordering {
            Ordering::Equal
                .then(self.tops.cmp(&other.tops))
                .then(self.zones.cmp(&other.zones))
                .then(self.top_tries.cmp(&other.top_tries).reverse())
                .then(self.start_order.cmp(&other.start_order))
        }
    }

    #[derive(PartialEq, Eq, Clone, Debug)]
    pub struct Attempts {
        success: bool,
        tries: u64,
    }
}

mod speed {
    use super::{Ascent, Discipline, Score};
    use crate::api;
    use dioxus::prelude::*;

    #[derive(Debug)]
    pub struct Speed;

    impl Discipline for Speed {
        type Ascent = SpeedAscent;
        type Score = SpeedScore;
    }

    pub struct SpeedAscent {
        pub time_ms: u64,
    }

    impl TryFrom<api::result::Ascent> for SpeedAscent {
        type Error = ();

        fn try_from(value: api::result::Ascent) -> Result<Self, Self::Error> {
            if let Some(api::result::SpeedAscent { time_ms }) = value.speed {
                Ok(Self { time_ms })
            } else {
                Err(())
            }
        }
    }

    impl Ascent for SpeedAscent {
        fn render(&self) -> LazyNodes {
            rsx! { div { "{self.time_ms}" } }
        }
    }

    #[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub struct SpeedScore {
        time_ms: u64
    }

    impl Score for SpeedScore {
        type Ascent = SpeedAscent;

        fn render(&self) -> LazyNodes {
            rsx! { div { "{self.time_ms}" } }
        }

        fn calculate(_start_order: u64, ascents: &[Self::Ascent]) -> Self {
            let time_ms = ascents.iter().map(|a| a.time_ms).min().unwrap_or(0);
            Self { time_ms }
        }
    }
}
