use crate::api::seasons::{
    Event as ApiEvent, Season, SeasonsResponse,
    ShortEvent, ShortSeason,
};
use crate::leaderboard::LeaderboardInput;
use crate::{api, Page};
use chrono::{DateTime, Local};
use dioxus::prelude::*;

fn Season(cx: Scope<ShortSeason>) -> Element {
    let expanded = use_state(cx, || false);

    let expanded_class = if *expanded.get() {
        "expanded"
    } else {
        ""
    };

    cx.render(rsx! {
        div {
            div {
                class: "season {expanded_class}",
                onclick: move |_| expanded.modify(|b| !b), "{cx.props.name}"
            }
            if *expanded.get() {
                rsx!{ EventList { ..cx.props.clone() } }
            }
        }
    })
}

fn EventList(cx: Scope<ShortSeason>) -> Element {
    let url = format!("seasons/{}", cx.props.id);
    let future = use_future(cx, (&cx.props.id,), |_| api::request::<Season>(url));

    let events = match future.value() {
        Some(Some(season)) => {
            let mut events = season.events.clone();
            events.sort_by_key(|e| e.starts_at);
            rsx! { events.into_iter().map(|e| rsx!{ Event { ..e } })}
        }
        _ => rsx! { div { class: "event", "Loading..." } },
    };

    cx.render(rsx! {
        div {
            class: "nested",
            events
        }
    })
}

fn Event(cx: Scope<ShortEvent>) -> Element {
    let expanded = use_state(cx, || false);
    let expanded_class = if *expanded.get() {
        "expanded"
    } else {
        ""
    };
    let now = chrono::offset::Utc::now();
    
    let state = if cx.props.starts_at > now {
        "Upcoming"
    } else if cx.props.ends_at > now {
        "Started"
    } else {
        "Finished"
    };

    let starts_at: DateTime<Local> = cx.props.starts_at.into();
    let ends_at: DateTime<Local> = cx.props.ends_at.into();

    let date = starts_at.date_naive().format("%b %e");
    let start_time = starts_at.time().format("%H:%M");
    let end_time = ends_at.time().format("%H:%M");

    cx.render(rsx! {
        div {
            div {
                class: "event {expanded_class}",
                onclick: move |_| expanded.modify(|b| !b),
                div { "{cx.props.event}" }
                div {
                    class: "datetime",
                    "{state} | {date} | {start_time} - {end_time}"
                }
            }
            if *expanded.get() {
                rsx!{ CategoryList { ..cx.props.clone() } }
            }
        }
    })
}

fn CategoryList(cx: Scope<ShortEvent>) -> Element {
    let url = format!("events/{}", cx.props.event_id);
    let future = use_future(cx, (&cx.props.event_id,), |_| api::request::<ApiEvent>(url));
    let page = use_shared_state::<Page>(cx).unwrap();

    let categories = match future.value() {
        Some(Some(event)) => event.dcats.clone(),
        _ => return cx.render(rsx! { "Loading..." }),
    };

    let nodes = categories.into_iter().map(|c| rsx!{ 
        div {
            div { class: "event", "{c.dcat_name}" },
            div {
                class: "nested",
                c.category_rounds.iter().map(|r| {
                    let event_name = cx.props.event.clone();
                    let round_id = r.category_round_id;
                    let f = move |_| {
                        *page.write() = Page::Leaderboard(LeaderboardInput::Api(event_name.clone(),  round_id));
                    };
                    rsx! {
                        div {
                            onclick: f,
                            class: "event",
                            "{r.name}"
                        }
                    }
                })
            }
        }
    });
    
    cx.render(rsx! {
        div {
            class: "nested",
            nodes
        }
    })
}

pub fn Home(cx: Scope) -> Element {
    let future = use_future(cx, (), |_| api::request::<SeasonsResponse>(String::new()));

    cx.render(match future.value() {
        Some(Some(SeasonsResponse { seasons })) => {
            rsx! {
                RelevantEvents { id: seasons[0].id, name: seasons[0].name.clone() }
                h1 { "All seasons" }
                div {
                    class: "seasons-table",
                    seasons.iter().map(|s| rsx!{ Season { ..s.clone() }})
                }
            }
        }
        _ => rsx! { "Loading..." },
    })
}

pub fn RelevantEvents(cx: Scope<ShortSeason>) -> Element {
    let url = format!("seasons/{}", cx.props.id);
    let future = use_future(cx, (&cx.props.id,), |_| api::request::<Season>(url));
    
    let now = chrono::offset::Utc::now();

    let filter_events = move |n, f: Box<dyn Fn(_) -> _>| {
        match future.value() {
            Some(Some(season)) => {
                let events = season.events.clone();
                let mut events: Vec<_> = events.into_iter().filter(|e| f(e.clone())).collect();
                if let Some(n) = n {
                    events.truncate(n);
                }
                if events.is_empty() {
                    rsx! { "No events" }
                } else {
                    rsx! { events.into_iter().map(|e| rsx! { Event { ..e.clone() } }) }
                }
            }
            Some(None) => rsx! { "Could not load events" },
            None => rsx! { "Loading... " },
        }
    };
    let current_events = filter_events(None, Box::new(|e: ShortEvent| e.starts_at < now && e.ends_at > now));
    let recent_events = filter_events(Some(5), Box::new(|e: ShortEvent| e.ends_at < now));
    let upcoming_events = filter_events(Some(5), Box::new(|e: ShortEvent| e.starts_at > now));

    cx.render(rsx! {
        h1 { "Current season" }
        h2 { "Current events" }
        current_events
        h2 { "Recent events" }
        recent_events
        h2 { "Upcoming events" }
        upcoming_events
    })
}