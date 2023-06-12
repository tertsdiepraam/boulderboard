//! Types that define a discpline
use crate::deserialize;
use dioxus::prelude::LazyNodes;

pub use boulder::*;
pub use lead::*;

pub trait Discipline {
    type Ascent: Ascent;
    type Score: Score<Ascent = Self::Ascent>;
}

pub trait Ascent: TryFrom<deserialize::Ascent> {
    fn render(&self) -> LazyNodes;
}

pub trait Score: Ord {
    type Ascent: Ascent;

    fn render(&self) -> LazyNodes;
    fn calculate(start_order: u64, ascents: &[Self::Ascent]) -> Self;
}

mod lead {
    use std::cmp::Ordering;

    use crate::deserialize;

    use super::{Ascent, Discipline, Score};
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

    impl TryFrom<deserialize::Ascent> for LeadAscent {
        type Error = ();

        fn try_from(value: deserialize::Ascent) -> Result<Self, Self::Error> {
            if let Some(deserialize::LeadAscent { score }) = value.lead {
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

    use crate::deserialize;

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
    }

    impl Ascent for BoulderAscent {
        fn render(&self) -> LazyNodes {
            if self.top {
                rsx! { div { class: "ascent ascent-full" } }
            } else if self.zone {
                rsx! { div { class: "ascent ascent-half" } }
            } else {
                rsx! { div { class: "ascent ascent-empty" } }
            }
        }
    }

    impl TryFrom<deserialize::Ascent> for BoulderAscent {
        type Error = ();

        fn try_from(value: deserialize::Ascent) -> Result<Self, Self::Error> {
            if let Some(deserialize::BoulderAscent {
                top,
                top_tries,
                zone,
                zone_tries,
            }) = value.boulder
            {
                Ok(Self {
                    top,
                    top_tries,
                    zone,
                    zone_tries,
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
