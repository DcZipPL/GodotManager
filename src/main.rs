#![allow(non_snake_case)]

pub mod components;

use dioxus::html::button;
use dioxus::prelude::*;
use dioxus_desktop::{Config, LogicalSize, WindowBuilder};
use dioxus_desktop::tao::dpi::{Size};
use crate::components::message::Message;
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
    use_shared_state_provider(cx, || MessageState::default());
    let message_state = use_shared_state::<MessageState>(cx);

    cx.render(rsx! {
        link { href: "./public/assets/style.css", rel:"stylesheet" },
        link { href: "./public/assets/list.css", rel:"stylesheet" },
        link { href: "./public/assets/message.css", rel:"stylesheet" },
        Message { },
        div {
            class: "topbar",
            "No default godot version selected",
            button {
                onclick: move |_event| {
					message_state.unwrap().write().message = None;
				},
                "Reset"
            },
            button {
                "Run"
            }
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

#[derive(Default)]
pub struct MessageState {
    pub message: Option<String>,
}

pub enum State {
    Loading,
    Loaded,
    Error,
}