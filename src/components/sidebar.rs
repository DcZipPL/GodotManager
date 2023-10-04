use dioxus::core::AttributeValue;
use dioxus::prelude::*;

#[inline_props]
pub fn Sidebar(cx: Scope, items: Vec<SidebarItem>) -> Element {
	render!(
		div {
			class: "sidebar",
			for item in items {
				div {
					class: if item.selected {
						"item selected"
					} else {
						"item"
					},
					//href: AttributeValue::Text(item.route.clone().leak()), // TODO: Don't leak. Well here we are cloning and leaking it.
					item.name.to_owned()
				}
			}
		}
	)
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct SidebarItem {
	pub name: String,
	pub route: String,
	pub selected: bool,
}