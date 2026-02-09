use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_brands_icons::{
    FaBootstrap, FaCss3Alt, FaHtml5, FaJava, FaJs, FaPython, FaReact, FaRust,
};
use dioxus_free_icons::Icon;

#[component]
pub fn Projects() -> Element {
    let projects = vec![
        Project::new(
            "Tish",
            Url::Local(asset!( "/assets/project/project_2.png")),
            "Tish is an e-commerce website dedicated to fashion, built with Java Enterprise Edition (JEE), JSP, and Servlet technologies.",
            vec![
                rsx!(Icon { icon: FaJava }),
                rsx!(Icon { icon: FaHtml5 }),
                rsx!(Icon { icon: FaJs }),
                rsx!(Icon { icon: FaCss3Alt }),
            ],
        ),
        Project::new(
            "Slate",
            Url::Local(asset!("/assets/project/project_3.png")),
            "Slate is a QA-style web app developed during a hackathon, inspired by Stack Overflow. Built with Python, HTML5, React, and Bootstrap.",
            vec![
                rsx!(Icon { icon: FaPython }),
                rsx!(Icon { icon: FaHtml5 }),
                rsx!(Icon { icon: FaReact }),
                rsx!(Icon { icon: FaBootstrap }),
            ],
        ),
        Project::new(
            "Joe's blog",
            Url::Local(asset!("/assets/project/project_4.png")),
            "Joe's blog is a dynamic platform built with my custom Oxapy library, offering fast routing and middleware support, and powered by Jinja for sharing tech insights and personal experiences.",
            vec![rsx!(Icon { icon: FaPython }), rsx!(Icon { icon: FaHtml5 }), rsx!(Icon { icon: FaBootstrap})],
        ),
        Project::new(
            "Metatype",
            Url::Remote("https://opengraph.githubassets.com/1/metatypedev/metatype"),
            "Metatype is a declarative platform for API development using WebAssembly, TypeScript, and Python for modular backend components.",
            vec![rsx!(Icon { icon: FaPython }), rsx!(Icon { icon: FaRust }), rsx!(Icon {icon: FaJs})],
        ),
        Project::new(
            "Russenger",
            Url::Remote("https://opengraph.githubassets.com/1/j03-dev/russenger"),
            "Russenger is a Rust library to simplify Facebook Messenger webhook responses with an intuitive Rust API.",
            vec![rsx!(Icon { icon: FaRust })],
        ),
        Project::new(
            "Oxapy",
            Url::Remote("https://opengraph.githubassets.com/1/j03-dev/oxapy"),
            "Oxapy is a Python HTTP server library built in Rust, offering fast routing, middleware support, static file serving, and state management.",
            vec![rsx!(Icon { icon: FaPython }), rsx!(Icon { icon: FaRust })],
        ),
        Project::new(
            "RusqlAlchemy",
            Url::Remote("https://opengraph.githubassets.com/1/j03-dev/rusql-alchemy"),
            "RusqlAlchemy is an ORM-style Rust library inspired by Django, designed to simplify database interaction with Rust.",
            vec![rsx!(Icon { icon: FaRust })],
        ),
        Project::new(
            "DoYou",
            Url::Remote("https://opengraph.githubassets.com/1/j03-dev/doyou"),
            "DoYou is a Rust-based music player application focused on simplicity and performance.",
            vec![rsx!(Icon { icon: FaRust })],
        ),
        Project::new(
            "System Theme",
            Url::Remote("https://opengraph.githubassets.com/1/j03-dev/system-theme.hx"),
            "System Theme is a plugin for the Helix editor that auto-switches between light and dark themes.",
            vec![rsx!(Icon { icon: FaRust })],
        ),
        Project::new(
            "Antsirabe Bus API",
            Url::Remote("https://opengraph.githubassets.com/1/j03-dev/bus_antsirabe_api"),
            "Antsirabe Bus API is a REST API offering real-time bus schedule data for Antsirabe, built with Rust and Python.",
            vec![rsx!(Icon { icon: FaPython })],
        ),
        Project::new(
            "TimeTable",
            Url::Remote("https://opengraph.githubassets.com/1/tbgracy/timetable"),
            "TimeTable is a CLI tool helping educational institutions create balanced timetables with constraint-based scheduling.",
            vec![rsx!(Icon { icon: FaPython })],
        ),
        Project::new(
            "Osas-Player",
            Url::Remote("https://opengraph.githubassets.com/1/j03-dev/osas-player"),
            "Osas-Player is a Python audio player built with Tkinter, offering a user-friendly GUI for playback.",
            vec![rsx!(Icon { icon: FaPython })],
        ),
    ];

    rsx! {
        div { class: "mx-auto w-full max-w-7xl grid grid-cols-1 gap-5 md:grid-cols-3",
            for project in projects {
                ProjectCard { project }
            }
        }
    }
}

#[component]
fn ProjectCard(project: Project) -> Element {
    rsx! {
        div { class: "card bg-base-100 w-full h-full w-96 shadow-sm",
            figure {
                img {
                    src: match project.image {
                        Url::Local(a) => a.to_string(),
                        Url::Remote(u) => u.to_string(),
                    },
                    alt: project.title,
                }
            }
            div { class: "card-body",
                h2 { class: "card-title", {project.title} }
                p { {project.description} }
                div { class: "card-actions justify-end text-xl",
                    for tech_icon in project.techno_icons {
                        {tech_icon}
                    }
                }
            }
        }
    }
}

#[derive(PartialEq, Clone)]
enum Url {
    Local(Asset),
    Remote(&'static str),
}

#[derive(PartialEq, Clone)]
struct Project {
    title: &'static str,
    image: Url,
    description: &'static str,
    techno_icons: Vec<Element>,
}

impl Project {
    fn new(
        title: &'static str,
        image: Url,
        description: &'static str,
        techno_icons: Vec<Element>,
    ) -> Self {
        Self {
            title,
            image,
            description,
            techno_icons,
        }
    }
}
