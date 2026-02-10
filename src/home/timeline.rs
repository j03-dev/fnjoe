use dioxus::prelude::*;

#[component]
pub fn Timelines() -> Element {
    let timelines = [
        Timeline {
            time: "2019",
            title: "Started Bachelor Degree",
            description: "Started BSc in Science & Technology at ASJA University, focusing on programming and software fundamentals.",
        },
        Timeline {
            time: "2022",
            title: "Bachelor Graduation",
            description: "Graduated BSc and started specializing in backend development and systems programming using Python and Rust.",
        },
        Timeline {
            time: "2023",
            title: "Freelance & Community Involvement",
            description: "Worked as Freelance Backend Developer and Odoo Developer. Became organizer at GDG Antsirabe and started Master degree (M1 Software Engineering).",
        },
        Timeline {
            time: "2024",
            title: "Metatype Core Maintainer",
            description: "Worked on Metatype.dev contributing to runtimes, gRPC, KV storage, and WebAssembly integrations. Also created personal projects OxAPY and DoYou.",
        },
        Timeline {
            time: "2025",
            title: "Open Source & Framework Research",
            description: "Continued open source contributions, experimented with Rust web frameworks, ORM design, and developer tooling improvements.",
        },
        Timeline {
            time: "2026",
            title: "SaaS & Developer Tooling",
            description: "Currently building SaaS products, AI integrations, mobile applications with Flutter, and developer tooling using Rust, Python and Dioxus.",
        }
    ];

    rsx! {
        div { class: "flex-1 flex items-center justify-center mx-5 md:mx-0",
            ul { class: "timeline timeline-snap-icon max-md:timeline-compact timeline-vertical w-full max-w-4xl",
                for (i , timeline) in timelines.into_iter().enumerate() {
                    if i % 2 == 0 {
                        TimelineItem { timeline, pos: Pos::Start }
                    } else {
                        TimelineItem { timeline, pos: Pos::End }
                    }
                }
            }
        }
    }
}

#[component]
fn TimelineItem(timeline: Timeline, pos: Pos) -> Element {
    rsx! {
        li {
            div { class: "timeline-middle",
                svg {
                    xmlns: "https://www.w3.org/2000/svg",
                    view_box: "0 0 20 20",
                    fill: "currentColor",
                    class: "h-5 w-5",
                    path {
                        fill_rule: "evenodd",
                        d: "M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z",
                        clip_rule: "evenodd",
                    }
                }
            }
            div { class: "{pos}",
                time { class: "font-mono italic font-bold", {timeline.time} }
                div { class: "text-lg font-black text-secondary", {timeline.title} }
                {timeline.description}
            }
            hr {}
        }
    }
}

#[derive(Clone, PartialEq)]
struct Timeline {
    time: &'static str,
    title: &'static str,
    description: &'static str,
}

#[derive(Clone, PartialEq)]
enum Pos {
    Start,
    End,
}

impl std::fmt::Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pos = match self {
            Pos::Start => "timeline-start mb-10 md:text-end",
            Pos::End => "timeline-end md:mb-10",
        };
        write!(f, "{}", pos)
    }
}
