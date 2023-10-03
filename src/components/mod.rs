use dioxus::core::AttributeValue;
use dioxus::prelude::*;

pub mod version;
pub mod message;

#[inline_props]
pub fn Badge(cx: Scope, color: BadgeColor, text: String) -> Element {
	let color_hex: String = format!("border-color: {}", String::from(color)).to_string();
	render!(
		span {
			class: "badge",
			style: AttributeValue::Text(color_hex.leak()),
			text.to_string()
		}
	)
}

#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum BadgeColor {
	Green,
	Yellow,
	Red,
}

impl From<&BadgeColor> for String {
	fn from(color: &BadgeColor) -> Self {
		match color {
			BadgeColor::Green => "#00eb00".to_string(),
			BadgeColor::Yellow => "#ffa200".to_string(),
			BadgeColor::Red => "#ff0000".to_string(),
		}
	}
}