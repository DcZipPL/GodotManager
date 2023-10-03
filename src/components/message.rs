use dioxus::html::style;
use dioxus::prelude::*;
use crate::MessageState;

pub fn Message(cx: Scope) -> Element {
	let message_state = use_shared_state::<MessageState>(cx);
	let message = message_state.unwrap().read().message.clone();

	render!(
		div {
			id: "popup",
			class: "popup-container show",
			span {
				class: "popup-close",
			},
			p {
				class: "popup-message",
				style: if message.clone().is_some() {
					"slideIn 0.5s forwards"
				} else {
					"fadeOut 0.5s forwards"
				},
				message.clone().unwrap_or("".to_string())
			}
		}
	)
}