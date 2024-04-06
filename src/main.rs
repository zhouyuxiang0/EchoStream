#![allow(non_snake_case)]

use dioxus::{
    desktop::{tao::platform::windows::WindowBuilderExtWindows, Config, WindowBuilder},
    prelude::*,
};
use log::LevelFilter;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    LaunchBuilder::desktop()
        .with_cfg(
            Config::new().with_window(
                WindowBuilder::new()
                    .with_decorations(true)
                    .with_title("EchoStream"),
            ),
        )
        .launch(App);
}

#[component]
fn App() -> Element {
    rsx! { Router::<Route> {} }
}

#[component]
fn Blog(id: i32) -> Element {
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        "Blog post {id}"
    }
}

#[component]
fn Home() -> Element {
    // let mut count = use_signal(|| 0);

    rsx! {
        div { class: "menu-button",
            svg {
                "xmlns": "http://www.w3.org/2000/svg",
                "stroke-linejoin": "round",
                "fill": "none",
                "stroke-width": "2",
                "stroke-linecap": "round",
                "stroke": "currentColor",
                "viewBox": "0 0 24 24",
                path { "d": "M13 2H6a2 2 0 0 0-2 2v16c0 1.1.9 2 2 2h12a2 2 0 0 0 2-2V9l-7-7z" }
                path { "d": "M13 3v6h6" }
            }
            div { class: "plus", "+" }
            span { "连接" }
        }
    }
}
