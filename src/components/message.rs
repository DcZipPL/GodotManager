use dioxus::prelude::*;

#[inline_props]
pub fn Message(cx: Scope, content: String) -> Element {
	render!(
		div {
			style: "padding: 8px;",
			content.to_owned()
		}
	)
}