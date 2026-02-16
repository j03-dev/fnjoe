use dioxus::prelude::*;

mod hero;
mod project;
mod skill;
mod timeline;

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            section { id: "hero", class: "hero min-h-screen bg-base-100", hero::Hero {} }
            section { id: "timeline", class: "min-h-screen bg-base-200 py-5",
                div { class: "text-center mb-10",
                    h2 { class: "text-3xl font-bold",
                        "Carer "
                        span { class: "text-primary", "Timeline" }
                    }
                    p { class: "text-gray-500 mt-2",
                        "A journey through my academic background, professional experience and open source contributions."
                    }
                }
                timeline::Timelines {}
            }
            section { id: "skills", class: "h-auto bg-base-100 py-5",
                div { class: "text-center mb-10",
                    h2 { class: "text-3xl font-bold",
                        "Technical "
                        span { class: "text-primary", "Skill" }
                    }
                    p { class: "text-gray-500 mt-2",
                        "A showcase of my technical proficiencies and expertise."
                    }
                }
                skill::Skills {}
            }
            section { id: "projects", class: "min-h-screen bg-base-200 py-5 px-5",
                div { class: "text-center mb-10",
                    h2 { class: "text-3xl font-bold",
                        "Featured "
                        span { class: "text-primary", "Project" }
                    }
                    p { class: "text-gray-500 mt-2",
                        "Discover some of my notable works and creations."
                    }
                }
                project::Projects {}
            }
        }
    }
}
