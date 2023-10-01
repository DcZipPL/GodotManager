#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;
use anyhow::Result;
use octocrab::models::repos::Release;

fn main() {
    // launch the dioxus app in a webview
    dioxus_desktop::launch(App);
}

// define a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            Versions {}
        },
    })
}

fn Versions(cx: Scope) -> Element {
    let versions = use_future(cx, (), |_| get_godot_versions());

    match versions.value() {
        None => {
            render!("Nothing to show")
        }
        Some(Ok(vers)) => {
            render!(
                for ver in vers {
                    div {
                        format!("{} ({})", ver.version.clone(), ver.prerelease.clone())
                    }
                }
            )
        }
        Some(Err(e)) => {
            render!("Error occurred")
        }
    }
}

async fn get_godot_versions() -> Result<Vec<GodotVersion>> {
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

struct GodotVersion {
    version: String,
    prerelease: bool
}