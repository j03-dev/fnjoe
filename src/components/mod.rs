use dioxus::prelude::*;

mod footer;
mod hero;
mod navbar;
mod project;
mod skill;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[component]
pub fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        navbar::NavBar {}
        hero::Hero {}
        skill::Skills {}
        project::Projects {}
        footer::Footer {}
    }
}
