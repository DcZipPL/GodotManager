use dioxus::prelude::*;
use crate::MessageState;

pub fn Message(cx: Scope) -> Element {
	let message_state = use_shared_state::<MessageState>(cx);
	let message = message_state.unwrap().read();

	render!(
		div {
			id: "popup",
			class: "popup-container show",
			style: if message.done {
				"animation: fadeOut 0.5s forwards"
			} else {
				"animation: slideIn 0.5s forwards"
			},
			span {
				class: "popup-close",
			},
			p {
				class: "popup-message",
				message.message.clone()
			}
		}
	)
}