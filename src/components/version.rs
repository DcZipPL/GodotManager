use dioxus::prelude::*;
use octocrab::models::repos::Release;
use serde::{Deserialize, Serialize};
use anyhow::Result;

pub fn VersionListing(cx: Scope) -> Element {
	let versions = use_future(cx, (), |_| get_godot_versions());

	match versions.value() {
		None => {
			render!("Fetching versions...")
		}
		Some(Ok(vers)) => {
			render!(
                for ver in vers {
                    div {
                        VersionComponent {
                            version: ver.clone()
                        }
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
	render!(
        version.version.clone(),
        span {
            style: "color: red;",
            if version.prerelease {
                " (prerelease)"
            } else {
                ""
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

impl From<Release> for GodotVersion {
	fn from(p: Release) -> Self {
		Self {
			version: p.name.unwrap_or("No version".to_owned()),
			prerelease: p.prerelease
		}
	}
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GodotVersion {
	pub version: String,
	pub prerelease: bool
}