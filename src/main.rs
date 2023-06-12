#![allow(non_snake_case)]
mod deserialize;
mod discipline;

use std::{cmp::Reverse, io::Write};

use clap::Parser;
use deserialize::{DisciplineTag, Results};
use dioxus::prelude::*;
use dioxus_desktop::Config;
use discipline::Discipline;
use tokio::time::{sleep, Duration};

use crate::discipline::{Ascent, Boulder, Lead, Score};

#[derive(Parser, Debug, Props, PartialEq)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long, group = "input")]
    event: Option<u64>,

    /// Number of times to greet
    #[arg(short, long, group = "input")]
    file: Option<String>,
}

#[derive(PartialEq, Clone)]
enum Input {
    Api(u64),
    File(String),
}

fn main() {
    let args = Args::parse();

    let input = match args {
        Args { event: Some(x), .. } => Input::Api(x),
        Args { file: Some(x), .. } => Input::File(x),
        _ => unreachable!("should be caught by clap"),
    };

    // launch the dioxus app in a webview
    dioxus_desktop::launch_with_props(App, (input,), Config::default());
}

#[derive(PartialEq, Props, Clone)]
struct AthleteProps<D: Discipline> {
    id: u64,
    first_name: String,
    last_name: String,
    country: String,
    ascents: Vec<D::Ascent>,
    score: D::Score,
    active: bool,
    rank: usize,
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
                // Rank is computed later by us, because the API uses weird unstable sorting
                rank: 0,
            }
        })
        .collect()
}

fn Athlete<D: Discipline>(cx: Scope<AthleteProps<D>>) -> Element {
    let AthleteProps {
        id,
        first_name,
        last_name,
        ascents,
        score,
        country,
        active: _,
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
            key: "{id}",
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

const BASE_URL: &str = "https://components.ifsc-climbing.org/results-api.php?api=event_full_results&result_url=/api/v1";

async fn fetch_results(input: &Input) -> Option<Results> {
    let res = match input {
        Input::Api(x) => {
            let url = dbg!(format!("{BASE_URL}/category_rounds/{x}/results/"));
            let res = reqwest::get(url).await.ok()?.text().await.ok()?;
            let mut f = std::fs::File::create(format!("results_{x}.json")).unwrap();
            write!(f, "{}", &res).unwrap();
            res
        }
        Input::File(x) => std::fs::read_to_string(x).unwrap(),
    };

    let clean = clean_api_output(res);
    let parsed: Results = serde_json::from_str(&clean).unwrap();
    Some(parsed)
}

/// Remove random fucking PHP warnings from the output
fn clean_api_output(x: String) -> String {
    x.lines().filter(|line| !line.starts_with('<')).collect()
}

// define a component that renders a div with the text "Hello, world!"
fn App(cx: Scope<(Input,)>) -> Element {
    let results = use_state(cx, || None);
    let _ = use_coroutine(cx, |_: UnboundedReceiver<()>| {
        let results = results.to_owned();
        let input = cx.props.0.to_owned();
        async move {
            loop {
                dbg!("fetching!!");
                results.set(fetch_results(&input).await);
                sleep(Duration::from_millis(1000)).await;
            }
        }
    });

    let r = match results.get() {
        Some(r) => r,
        None => return cx.render(rsx! { "Nothing to show" }),
    };

    cx.render(rsx! {
        style { include_str!("../public/style.css") }
        if let Some(event) = &r.event {
            rsx! { div { "{event}" } }
        }
        div {
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
                        .map(|(a, i)| rsx! {Athlete { rank: i+1, ..a } })
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
                        .map(|(a, i)| rsx! {Athlete { rank: i+1, ..a } })
                        .collect();

                    rsx!{ rendered.into_iter() }
                }
            }
        }
    })
}
