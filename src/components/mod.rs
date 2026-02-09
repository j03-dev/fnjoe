use dioxus::prelude::*;

mod footer;
mod hero;
mod navbar;
mod project;
mod skill;
mod timeline;

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
        SectionContainer {
            id: "timeline",
            title: "Carer Timeline",
            description: "A journey through my academic background, professional experience and open source contributions.",
            class: "bg-base-200",
            timeline::Timelines {}
        }
        SectionContainer {
            id: "skills",
            title: "Technical Skill",
            description: "A showcase of my technical proficiencies and expertise.",
            class: "bg-base-100",
            skill::Skills {}
        }
        SectionContainer {
            id: "projects",
            title: "Featured Project",
            description: "Discover some of my notable works and creations.",
            class: "bg-base-200 px-5 md:px-0",
            project::Projects {}
        }
        footer::Footer {}
    }
}

#[component]
fn SectionContainer(
    #[props(default)] id: &'static str,
    title: &'static str,
    description: &'static str,
    #[props(default)] class: &'static str,
    #[props(default)] children: Element,
) -> Element {
    rsx! {
        section { id, class: "min-h-screen py-10 {class}",
            div { class: "text-center mb-10",
                h2 { class: "text-3xl font-bold",
                    for (i , t) in title.split(" ").enumerate() {
                        if i == 0 {
                            "{t} "
                        } else {
                            span { class: "text-primary", {t} }
                        }
                    }
                }
                p { class: "text-gray-500 mt-2", {description} }
            }
            {children}
        }
    }
}
