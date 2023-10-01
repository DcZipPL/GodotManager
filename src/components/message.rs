use dioxus::prelude::*;

pub fn Message(cx: Scope) -> Element {
	render!(
		div {
			style: "padding: 8px;",
			"Hello!"
		}
	)
}