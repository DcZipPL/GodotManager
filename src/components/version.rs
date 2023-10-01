use dioxus::prelude::*;
use octocrab::models::repos::{Asset, Release};
use serde::{Deserialize, Serialize};
use anyhow::Result;
use chrono::{DateTime, Utc};

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
	render!(
		div {
			style: "padding: 8px;",
			onclick: move |_event| {
				handle_download_links(&downloads);
			},
			version.version.clone(),
			if version.prerelease {
				rsx!(
					Badge {
						text: "Pre-release".to_string()
					}
				)
			}
		}
    )
}

#[inline_props]
pub fn Badge(cx: Scope, text: String) -> Element {
	render!(
		span {
			style: "margin-left: 8px; border: 1.5px #ffa200 solid; padding: 2px 4px; border-radius: 5px;",
			text.to_string()
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
	fn from(p: Release) -> Self {
		Self {
			version: p.name.unwrap_or("No version".to_owned()),
			uploaded_at: p.published_at,
			assets: p.assets,
			prerelease: p.prerelease,
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