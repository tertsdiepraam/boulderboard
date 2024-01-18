use crate::api;
use crate::api::result::{DisciplineTag, Results};
use crate::discipline::Discipline;
use crate::discipline::{Ascent, Boulder, Lead, Score, Speed};
use dioxus::prelude::*;
use std::cmp::Reverse;
use std::path::PathBuf;
use std::time::Duration;

#[derive(PartialEq, Clone)]
pub enum LeaderboardInput {
    Api(String, u64),
    File(PathBuf),
}

#[derive(PartialEq, Props)]
pub struct LeaderboardProps {
    input: LeaderboardInput,
}

#[derive(PartialEq, Props, Clone)]
struct AthleteProps<D: Discipline> {
    id: u64,
    first_name: String,
    last_name: String,
    country: String,
    flag: String,
    ascents: Vec<D::Ascent>,
    score: D::Score,
    active: bool,
    rank: usize,
}

fn Athlete<D: Discipline>(cx: Scope<AthleteProps<D>>) -> Element {
    let AthleteProps {
        id: _,
        first_name,
        last_name,
        ascents,
        score,
        country,
        flag: _flag,
        active,
        rank,
    } = cx.props;

    let initials = first_name
        .split(' ')
        .map(|n| format!("{}.", n.chars().next().unwrap()))
        .collect::<Vec<_>>()
        .join(" ");

    let order = rank - 1;
    cx.render(rsx! {
        div {
            class: if *active { "row-active" } else { "" },
            style: "--order: {order}",
            div { class: "rank", "{rank}" }
            div { class: "country-code", "{country}" }
            div { class: "athlete-name", "{initials} {last_name}" }
            div {
                class: "ascents",
                ascents.iter().map(|a| a.render())
            }
            score.render()
        }
    })
}

fn extract_athletes<D: Discipline>(results: &Results) -> Vec<AthleteProps<D>> {
    results
        .ranking
        .iter()
        .map(|rank_athlete| {
            let ascents = rank_athlete
                .ascents
                .iter()
                .map(|a| D::Ascent::try_from(a.clone()).ok())
                .collect::<Option<Vec<_>>>()
                .unwrap();
            // FIXME: Get the start order from the start list. The start order field on the RankAthlete might be
            // missing.
            let score = D::Score::calculate(0, &ascents);
            AthleteProps {
                id: rank_athlete.athlete.athlete_id,
                first_name: rank_athlete.athlete.firstname.clone(),
                last_name: rank_athlete.athlete.lastname.clone(),
                ascents,
                score,
                active: rank_athlete.active,
                country: rank_athlete.athlete.country.country.clone(),
                flag: rank_athlete.athlete.country.flag_url.clone(),
                // Rank is computed later by us, because the API uses weird unstable sorting
                rank: 0,
            }
        })
        .collect()
}

async fn fetch_results(input: &LeaderboardInput) -> Option<Results> {
    match input {
        LeaderboardInput::Api(_, x) => {
            api::request::<Results>(format!("category_rounds/{x}/results/")).await
        }
        LeaderboardInput::File(x) => {
            let res = std::fs::read_to_string(x).unwrap();
            serde_json::from_str(&res).map_err(|e| dbg!(e)).ok()
        }
    }
}

pub fn Leaderboard(cx: Scope<LeaderboardProps>) -> Element {
    let results = use_state(cx, || None);
    let _ = use_coroutine(cx, |_: UnboundedReceiver<()>| {
        let results = results.to_owned();
        let input = cx.props.input.to_owned();
        async move {
            loop {
                dbg!("fetching!!");
                results.set(fetch_results(&input).await);
                #[cfg(feature = "desktop")]
                tokio::time::sleep(Duration::from_millis(1000)).await;
                #[cfg(feature = "web")]
                gloo_timers::future::sleep(Duration::from_millis(1000)).await;
            }
        }
    });

    let r = match results.get() {
        Some(r) => r,
        None => return cx.render(rsx! { "Nothing to show" }),
    };

    let event = if let Some(event) = &r.event {
        Some(event)
    } else if let LeaderboardInput::Api(event, _) = &cx.props.input {
        Some(event)
    } else {
        None
    };

    cx.render(rsx! {
        if let Some(event) = event {
            rsx! { div { class: "leaderboard-event", "{event}" } }
        }
        div {
            class: "leaderboard-round",
            "{r.discipline} - {r.category} - {r.round}"
        }
        div {
            class: "table",
            match r.discipline {
                DisciplineTag::Lead => {
                    let athletes = extract_athletes::<Lead>(r);

                    // We have to keep the nodes in the same order in the DOM
                    // so we need a map from index to rank. Additionally, we
                    // need Reverse, because we want the highest score first.
                    let mut indices: Vec<_> = (0..athletes.len()).collect();
                    indices.sort_by_key(|&i| Reverse(athletes[i].score.clone()));

                    let mut ranking: Vec<_> = (0..athletes.len()).collect();
                    ranking.sort_by_key(|&i| indices[i]);

                    let rendered: Vec<_> = athletes
                        .into_iter()
                        .zip(ranking)
                        .map(|(a, i)| rsx! { Athlete { rank: i+1, ..a } })
                        .collect();

                    rsx!{ rendered.into_iter() }
                }
                DisciplineTag::Boulder => {
                    let athletes = extract_athletes::<Boulder>(r);

                    // We have to keep the nodes in the same order in the DOM
                    // so we need a map from index to rank. Additionally, we
                    // need Reverse, because we want the highest score first.
                    let mut indices: Vec<_> = (0..athletes.len()).collect();
                    indices.sort_by_key(|&i| Reverse(athletes[i].score.clone()));

                    let mut ranking: Vec<_> = (0..athletes.len()).collect();
                    ranking.sort_by_key(|&i| indices[i]);

                    let rendered: Vec<_> = athletes
                        .into_iter()
                        .zip(ranking)
                        .map(|(a, i)| rsx! {Athlete { key: "{a.id}", rank: i+1, ..a } })
                        .collect();

                    rsx!{ rendered.into_iter() }
                }
                DisciplineTag::Speed => {
                    let athletes = extract_athletes::<Speed>(r);

                    // We have to keep the nodes in the same order in the DOM
                    // so we need a map from index to rank. Additionally, we
                    // need Reverse, because we want the highest score first.
                    let mut indices: Vec<_> = (0..athletes.len()).collect();
                    indices.sort_by_key(|&i| Reverse(athletes[i].score.clone()));

                    let mut ranking: Vec<_> = (0..athletes.len()).collect();
                    ranking.sort_by_key(|&i| indices[i]);

                    let rendered: Vec<_> = athletes
                        .into_iter()
                        .zip(ranking)
                        .map(|(a, i)| rsx! {Athlete { key: "{a.id}", rank: i+1, ..a } })
                        .collect();

                    rsx!{ rendered.into_iter() }
                }
            }
        }
    })
}
