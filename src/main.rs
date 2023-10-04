#![allow(non_snake_case)]

pub mod components;

use dioxus::html::{br, span};
use dioxus::prelude::*;
use dioxus_desktop::{Config, LogicalSize, WindowBuilder};
use dioxus_desktop::tao::dpi::{Size};
use crate::components::message::Message;
use crate::components::sidebar::{Sidebar, SidebarItem};
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

	let sidebar_items = vec![
		SidebarItem {
			name: "Projects".to_string(),
			route: "/".to_string(),
			selected: false,
		},
		SidebarItem {
			name: "Version Control".to_string(),
			route: "/vcs".to_string(),
			selected: false,
		},
		SidebarItem {
			name: "Installs".to_string(),
			route: "/installs".to_string(),
			selected: true,
		},
		SidebarItem {
			name: "Settings".to_string(),
			route: "/settings".to_string(),
			selected: false,
		},
	];

	cx.render(rsx! {
		link { href: "./public/assets/style.css", rel:"stylesheet" },
		link { href: "./public/assets/list.css", rel:"stylesheet" },
		link { href: "./public/assets/message.css", rel:"stylesheet" },
		div {
			class: "notice",
			"Godot Manager is still in development. Please report any bugs to the ",
			a {
				href: "https://github.com/DcZipPL/GodotManager/issues",
				"issue tracker"
			},
			".",
			br {},
			"User interface is not final and will change in the future."
		}
		Message { },
		div {
			class: "topbar",
			"No default godot version selected",
			button {
				"Reset"
			},
			button {
				"Run"
			}
		},
		div {
			class: "container",
			Sidebar {
				items: sidebar_items
			}
			div {
				class: "content",
				VersionListing {}
			}
		}
	})
}

#[derive(Default)]
pub struct MessageState {
	pub message: String,
	pub done: bool,
}

pub enum State {
	Loading,
	Loaded,
	Error,
}