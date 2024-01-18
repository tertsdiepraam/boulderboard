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
    Leaderboard(LeaderboardInput),
}

fn main() {
    let args = Args::parse();

    let page = match args.file {
        Some(x) => Page::Leaderboard(LeaderboardInput::File(x)),
        _ => Page::Home,
    };

    // launch the dioxus app in a webview
    #[cfg(feature = "web")]
    dioxus_web::launch_with_props(App, AppState { page }, dioxus_web::Config::default());
    #[cfg(feature = "desktop")]
    dioxus_desktop::launch_with_props(App, AppState { page }, dioxus_desktop::Config::default());
    #[cfg(all(feature = "desktop", feature = "web"))]
    compile_error!("Cannot enable both desktop and web");
    #[cfg(not(any(feature = "desktop", feature = "web")))]
    compile_error!("You have to enable either desktop or web");
}

const FONT: &str = r#"
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    <link href="https://fonts.googleapis.com/css2?family=Oswald:wght@200;300;400;500;600;700&display=swap" rel="stylesheet">
"#;

fn App(cx: Scope<AppState>) -> Element {
    use_shared_state_provider(cx, || cx.props.page.clone());
    let page = use_shared_state::<Page>(cx).unwrap();
    cx.render(rsx! {
        head { dangerous_inner_html: "{FONT}" }
        style { include_str!("../public/style.css") }
        div { onclick: move |_| *page.write() = Page::Home, class: "header", "Boulderboard"}
        match page.read().clone() {
            Page::Home => rsx! { Home {} },
            Page::Leaderboard(input) => rsx! { Leaderboard { input: input.clone() } },
        }
    })
}