use std::env;
use std::fs::{create_dir_all, File};
use std::io::Write;
use dioxus::prelude::*;
use octocrab::models::repos::{Asset, Release};
use serde::{Deserialize, Serialize};
use anyhow::Result;
use chrono::{DateTime, Utc};
use reqwest::{Method, Request, Url};
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
					message_state.unwrap().write().done = false; // TODO: Add a checkbox for mono
					filter_download_links_and_download(downloads.clone(), version.version.clone(), true)
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

// GODOT DOWNLOAD CODE

// Note: cannot borrow downloads and version because it is handled by the async closure return
pub async fn filter_download_links_and_download(downloads: Vec<GodotVersionDownload>, version: String, is_mono: bool) {
	for download in downloads {
		if download.content_type == "application/zip" {
			let arch = env::consts::ARCH;

			#[cfg(target_os = "linux")]
			match arch {
				"x86_64" => {
					if download.name.contains("linux_x86_64") {
						filter_mono_and_download_godot(download.url, download.name, &version, is_mono).await.unwrap();
					}
				}
				"x86" => {
					if download.name.contains("linux_x86_32") {
						filter_mono_and_download_godot(download.url, download.name, &version, is_mono).await.unwrap();
					}
				}
				_ => { // TODO: Handle other architectures
					println!("Not implemented for this architecture")
				}
			}
			#[cfg(target_os = "windows")]
			match arch {
				"x86_64" => {
					if download.name.contains("win64") {
						filter_mono_and_download_godot(download.url, download.name, &version, is_mono).await.unwrap();
					}
				}
				"x86" => {
					if download.name.contains("win32") {
						filter_mono_and_download_godot(download.url, download.name, &version, is_mono).await.unwrap();
					}
				}
				_ => { // TODO: Handle other architectures
					println!("Not implemented for this architecture")
				}
			}
		}
	}
}

pub async fn filter_mono_and_download_godot(url: String, name: String, version: &String, is_mono: bool) -> Result<()> {
	if is_mono && name.contains("mono") {
		download_godot(url, name, version).await?;
	} else if !is_mono && !name.contains("mono") {
		download_godot(url, name, version).await?;
	}
	Ok(())
}

pub async fn download_godot(url: String, name: String, version: &String) -> Result<()> {
	#[cfg(unix)]
		let app_data = env::var("HOME").expect("No HOME directory");
	#[cfg(windows)]
		let app_data = env::var("APPDATA").expect("No APP_DATA directory");

	let downloaded_bytes = reqwest::get(&url).await?.bytes().await?; // TODO: Handle these unwraps

	let instance_dir: String = format!("{}/Godot Manager/Instances/{}", app_data, version);
	let downloads_dir: String = format!("{}/Godot Manager/Instances/{}", app_data, version);

	create_dir_all(&instance_dir)?;
	create_dir_all(&downloads_dir)?;

	File::create(format!("{}/GM_download_{}.tmp", env::temp_dir().to_str().unwrap_or(downloads_dir.as_str()), name))?.write_all(&downloaded_bytes)?;
	Ok(())
}