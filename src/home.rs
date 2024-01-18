use crate::api::seasons::{
    Event as ApiEvent, Season, SeasonsResponse,
    ShortEvent, ShortSeason,
};
use crate::leaderboard::LeaderboardInput;
use crate::{api, Page};
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
        "Pending"
    } else if cx.props.ends_at > now {
        "Started"
    } else {
        "Finished"
    };
    cx.render(rsx! {
        div {
            div {
                class: "event {expanded_class}",
                onclick: move |_| expanded.modify(|b| !b),
                "{state} - {cx.props.event}"
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
                div {
                    class: "seasons-table",
                    seasons.iter().map(|s| rsx!{ Season { ..s.clone() }})
                }
            }
        }
        _ => rsx! { "Loading..." },
    })
}
