#![allow(non_snake_case)]
mod api;
mod discipline;
mod home;
mod leaderboard;

use crate::{
    home::Home,
    leaderboard::{Leaderboard, LeaderboardInput},
};
use clap::Parser;
use dioxus::prelude::*;
use dioxus_desktop::Config;
use std::path::PathBuf;

#[derive(Parser, Debug, Props, PartialEq)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Read data from a local file instead of from the API
    #[arg(short, long, group = "input")]
    file: Option<PathBuf>,
}

#[derive(PartialEq, Props)]
struct AppState {
    page: Page,
}

#[derive(PartialEq, Clone)]
enum Page {
    Home,
    Event(u64),
    Leaderboard(LeaderboardInput),
}

fn main() {
    let args = Args::parse();

    let page = match args.file {
        Some(x) => Page::Leaderboard(LeaderboardInput::File(x)),
        _ => Page::Home,
    };

    // launch the dioxus app in a webview
    dioxus_desktop::launch_with_props(App, AppState { page }, Config::default());
}

fn App(cx: Scope<AppState>) -> Element {
    use_shared_state_provider(cx, || cx.props.page.clone());
    let page = use_shared_state::<Page>(cx).unwrap();
    cx.render(rsx! {
        style { include_str!("../public/style.css") }
        match page.read().clone() {
            Page::Home => rsx! { Home {} },
            Page::Leaderboard(input) => rsx! { Leaderboard { input: input.clone() } },
            _ => todo!(),
        }
    })
}
