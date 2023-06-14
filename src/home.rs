use crate::api::seasons::{
    CategoryRound as ApiCategoryRound, Event as ApiEvent, Season, SeasonsResponse, ShortCategory,
    ShortEvent, ShortSeason,
};
use crate::leaderboard::LeaderboardInput;
use crate::{api, Page};
use dioxus::prelude::*;

fn Season(cx: Scope<ShortSeason>) -> Element {
    let expanded = use_state(cx, || false);

    cx.render(rsx! {
        div {
            class: "season",
            onclick: move |_| expanded.modify(|b| !b), "{cx.props.name}"
        }
        if *expanded.get() {
            rsx!{ EventList { ..cx.props.clone() } }
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
        _ => rsx! { "Loading..." },
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

    cx.render(rsx! {
        div {
            class: "event",
            onclick: move |_| expanded.modify(|b| !b), "{cx.props.event}"
        }
        if *expanded.get() {
            rsx!{ CategoryList { ..cx.props.clone() } }
        }
    })
}

fn CategoryList(cx: Scope<ShortEvent>) -> Element {
    let url = format!("events/{}", cx.props.event_id);
    let future = use_future(cx, (&cx.props.event_id,), |_| api::request::<ApiEvent>(url));

    let categories = match future.value() {
        Some(Some(event)) => {
            let categories = event.dcats.clone();
            rsx! { categories.into_iter().map(|c| rsx!{ Category { ..c } })}
        }
        _ => rsx! { "Loading..." },
    };

    cx.render(rsx! {
        div {
            class: "nested",
            categories
        }
    })
}

fn Category(cx: Scope<ShortCategory>) -> Element {
    cx.render(rsx! {
        div {
            "{cx.props.dcat_name}"
            div {
                cx.props.category_rounds.clone().into_iter().map(|r| rsx! { CategoryRound { ..r } })
            }
        }
    })
}

fn CategoryRound(cx: Scope<ApiCategoryRound>) -> Element {
    let page = use_shared_state::<Page>(cx).unwrap();
    let f = move |_| {
        *page.write() = Page::Leaderboard(LeaderboardInput::Api(cx.props.category_round_id));
    };
    cx.render(rsx! {
        div { onclick: f, "{cx.props.name}" }
    })
}

pub fn Home(cx: Scope) -> Element {
    let future = use_future(cx, (), |_| api::request::<SeasonsResponse>(String::new()));

    cx.render(match future.value() {
        Some(Some(SeasonsResponse { seasons })) => {
            rsx! { seasons.iter().map(|s| rsx!{ Season { ..s.clone() }}) }
        }
        _ => rsx! { "Loading..." },
    })
}
