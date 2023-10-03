#![allow(non_snake_case)]

pub mod components;

use dioxus::prelude::*;
use crate::components::version::VersionListing;

fn main() {
    // launch the dioxus app in a webview
    dioxus_desktop::launch(App);
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        link { href: "./public/assets/style.css", rel:"stylesheet" },
        link { href: "./public/assets/list.css", rel:"stylesheet" },
        div {
            class: "topbar",
            "hi"
        },
        div {
            class: "container",
            div {
                class: "sidebar",
                "hi"
            },
            div {
                class: "content",
                VersionListing {}
            }
        }
    })
}

pub enum State {
    Loading,
    Loaded,
    Error,
}