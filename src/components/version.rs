use dioxus::prelude::*;
use octocrab::models::repos::{Asset, Release};
use serde::{Deserialize, Serialize};
use anyhow::Result;
use chrono::{DateTime, Utc};
use crate::components::{Badge, BadgeColor};
use crate::MessageState;

pub fn VersionListing(cx: Scope) -> Element {
	let versions = use_future(cx, (), |_| get_godot_versions());

	match versions.value() {
		None => {
			render!("Fetching versions...")
		}
		Some(Ok(vers)) => {
			render!(
                for ver in vers {
					VersionComponent {
						version: ver.clone()
					}
                }
            )
		}
		Some(Err(e)) => {
			render!(e.to_string())
		}
	}
}

#[inline_props]
pub fn VersionComponent(cx: Scope, version: GodotVersion) -> Element {
	let downloads = get_godot_version_downloads(version);
	let message_state = use_shared_state::<MessageState>(cx);

	render!(
		div {
			class: "godot-version",
			span {
				class: "name",
				onclick: move |_event| {
					message_state.unwrap().write().message = format!("Downloading Godot {}", version.version);
					message_state.unwrap().write().done = false;
					handle_download_links(&downloads);
				},
				version.version.clone()
			},
			if !version.prerelease {
				rsx!(
					Badge {
						color: BadgeColor::Green,
						text: "Stable".to_string()
					}
				)
			}
		}
    )
}

pub fn handle_download_links(downloads: &Vec<GodotVersionDownload>) {
	for download in downloads {
		let os_type = std::env::consts::OS;
		match os_type {
			"windows" => {
				if download.name.contains("win") {
					println!("Download at {}", download.name)
				}
			}
			_ => { // TODO: Handle linux
				println!("Not implemented for this OS")
			}
		}
	}
}

pub async fn get_godot_versions() -> Result<Vec<GodotVersion>> {
	let github_access = octocrab::instance();
	let page = github_access.repos("godotengine", "godot-builds")
		.releases()
		.list()
		.per_page(10)
		.send()
		.await?;

	Ok(
		page.items.iter().map(|r| GodotVersion::from(r.clone())).collect()
	)
}

pub fn get_godot_version_downloads(version: &GodotVersion) -> Vec<GodotVersionDownload> {
	version.assets.iter().map(|a| GodotVersionDownload::from(a.clone())).collect()
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GodotVersion {
	pub version: String,
	pub uploaded_at: Option<DateTime<Utc>>,
	pub assets: Vec<Asset>,
	pub prerelease: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GodotVersionDownload {
	pub name: String,
	pub url: String,
	pub content_type: String,
}

impl From<Release> for GodotVersion {
	fn from(r: Release) -> Self {
		Self {
			version: if let Some(version_name) = r.name {
				if !version_name.is_empty() {
					version_name
				} else {
					r.tag_name
				}
			} else {
				r.tag_name
			},
			uploaded_at: r.published_at,
			assets: r.assets,
			prerelease: r.prerelease,
		}
	}
}

impl From<Asset> for GodotVersionDownload {
	fn from(p: Asset) -> Self {
		Self {
			url: p.browser_download_url.to_string(),
			name: p.name,
			content_type: p.content_type,
		}
	}
}