#![allow(non_snake_case)]

pub mod components;

use dioxus::prelude::*;
use dioxus_desktop::{Config, LogicalSize, WindowBuilder};
use dioxus_desktop::tao::dpi::{Size};
use crate::components::version::VersionListing;

fn main() {
    // launch the dioxus app in a webview
    dioxus_desktop::launch_cfg(App, Config::default().with_window(
        WindowBuilder::new()
            .with_resizable(true)
            .with_title("Godot Manager")
            .with_inner_size(Size::Logical(LogicalSize::new(800.0, 600.0)))
    ));
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        link { href: "./public/assets/style.css", rel:"stylesheet" },
        link { href: "./public/assets/list.css", rel:"stylesheet" },
        div {
            class: "topbar",
            "No updates available"
        },
        div {
            class: "container",
            div {
                class: "sidebar",
                "Versions"
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