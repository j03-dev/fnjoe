use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_brands_icons::{
    FaCss3Alt, FaGithub, FaHtml5, FaJava, FaJs, FaPython, FaReact, FaRust,
};
use dioxus_free_icons::Icon;

const TISH: Asset = asset!("/assets/project/tish.png");
const SLATE: Asset = asset!("/assets/project/slate.png");
const BLOG: Asset = asset!("/assets/project/joesblog.png");
const DOYOU: Asset = asset!("/assets/project/doyou.png");
const TIMETABLE: Asset = asset!("/assets/project/timetable.png");
const OSAS: Asset = asset!("/assets/project/osas.png");

#[component]
pub fn Projects() -> Element {
    rsx! {
        div { class: "mx-auto w-full max-w-7xl grid grid-cols-1 gap-5 md:grid-cols-3",
            for project in PROJECTS {
                ProjectCard { project }
            }
        }
    }
}

#[component]
fn ProjectCard(project: &'static Project) -> Element {
    rsx! {
        div { class: "card bg-base-100 shadow-sm h-full",
            figure {
                img {
                    src: "{project.image}",
                    alt: project.title,
                    class: "max-h-42 w-full",
                    loading: "lazy",
                }
            }

            div { class: "card-body",
                h2 { class: "card-title", {project.title} }
                p { {project.description} }

                div { class: "card-actions justify-end text-xl items-center flex",
                    for tech in project.tech {
                        {tech.icon()}
                    }
                    a {
                        class: "btn btn-neutral link link-hover",
                        href: project.repo,
                        Icon { icon: FaGithub }
                        "repo"
                    }
                }
            }
        }
    }
}

impl Tech {
    fn icon(&self) -> Element {
        match self {
            Tech::Java => rsx!(
                Icon { icon: FaJava }
            ),
            Tech::Html => rsx!(
                Icon { icon: FaHtml5 }
            ),
            Tech::Js => rsx!(
                Icon { icon: FaJs }
            ),
            Tech::Css => rsx!(
                Icon { icon: FaCss3Alt }
            ),
            Tech::Python => rsx!(
                Icon { icon: FaPython }
            ),
            Tech::React => rsx!(
                Icon { icon: FaReact }
            ),
            Tech::Rust => rsx!(
                Icon { icon: FaRust }
            ),
        }
    }
}

static PROJECTS: &[Project] = &[
    Project {
        title: "DoYou",
        image: ImageSource::Local(DOYOU),
        description: "DoYou is Youtube Music alternative built-in Rust with Dioxus.",
        tech: &[Tech::Rust],
        repo: "https://github.com/j03-ev/doyou"
    },
    Project {
        title: "Tish",
        image: ImageSource::Local(TISH),
        description: "Tish is an e-commerce website dedicated to fashion, built with Java Enterprise Edition (JEE), JSP, and Servlet technologies.",
        tech: &[Tech::Java, Tech::Html, Tech::Js, Tech::Css],
        repo: "https://github.com/tbgracy/tish"
    },
    Project {
        title: "Slate",
        image: ImageSource::Local(SLATE),
        description: "Slate is a QA-style web app developed during a hackathon, inspired by Stack Overflow. Built with Python, HTML5, React, and Css.",
        tech: &[Tech::Python, Tech::Html, Tech::React, Tech::Css],
        repo: "https://github.com/j03-dev/slate"
    },
    Project {
        title: "Joe's blog",
        image: ImageSource::Local(BLOG),
        description: "Joe's blog is a dynamic platform built with my custom Oxapy library, offering fast routing and middleware support, and powered by Jinja for sharing tech insights and personal experiences.",
        tech: &[Tech::Python, Tech::Html, Tech::Css],
        repo: "https://github.com/j03-dev/blog"
    },
    Project {
        title: "TimeTable",
        image: ImageSource::Local(TIMETABLE),
        description: "TimeTable is a CLI tool helping educational institutions create balanced timetables with constraint-based scheduling.",
        tech: &[Tech::Python],
        repo: "https://github.com/tbgracy/timetable"
    },
    Project {
        title: "Osas-Player",
        image: ImageSource::Local(OSAS),
        description: "Osas-Player is a Python audio player built with Tkinter, offering a user-friendly GUI for playback.",
        tech: &[Tech::Python],
        repo: "https://github.com/j03-dev/osas"
    },
    Project {
        title: "Metatype",
        image: ImageSource::Remote("https://opengraph.githubassets.com/1/metatypedev/metatype"),
        description: "Metatype is a declarative platform for API development using WebAssembly, TypeScript, and Python for modular backend components.",
        tech: &[Tech::Python, Tech::Rust, Tech::Js],
        repo: "https://github.com/metatypedev/metatype"
    },
    Project {
        title: "Russenger",
        image: ImageSource::Remote("https://opengraph.githubassets.com/1/j03-dev/russenger"),
        description: "Russenger is a Rust library to simplify Facebook Messenger webhook responses with an intuitive Rust API.",
        tech: &[Tech::Rust],
        repo: "https://github.com/metatypedev/metatype"
    },
    Project {
        title: "Oxapy",
        image: ImageSource::Remote("https://opengraph.githubassets.com/1/j03-dev/oxapy"),
        description: "Oxapy is a Python HTTP server library built in Rust, offering fast routing, middleware support, static file serving, and state management.",
        tech: &[Tech::Python, Tech::Rust],
        repo: "https://github.com/j03-dev/oxapy"
    },
    Project {
        title: "RusqlAlchemy",
        image: ImageSource::Remote("https://opengraph.githubassets.com/1/j03-dev/rusql-alchemy"),
        description: "RusqlAlchemy is an ORM-style Rust library inspired by Django, designed to simplify database interaction with Rust.",
        tech: &[Tech::Rust],
        repo: "https://github.com/j03-dev/rusql-alchemy"
    },
    Project {
        title: "System Theme",
        image: ImageSource::Remote("https://opengraph.githubassets.com/1/j03-dev/system-theme.hx"),
        description: "System Theme is a plugin for the Helix editor that auto-switches between light and dark themes.",
        tech: &[Tech::Rust],
        repo: "https://github.com/j03-dev/system-theme.hx"
    },
    Project {
        title: "Antsirabe Bus API",
        image: ImageSource::Remote("https://opengraph.githubassets.com/1/j03-dev/bus_antsirabe_api"),
        description: "Antsirabe Bus API is a REST API offering real-time bus schedule data for Antsirabe, built with Rust and Python.",
        tech: &[Tech::Python],
        repo: "https://github.com/j03-dev/bus_antsirabe_api"
    },
];

#[derive(Clone, PartialEq)]
pub enum Tech {
    Java,
    Html,
    Js,
    Css,
    Python,
    React,
    Rust,
}

#[derive(Clone, PartialEq)]
pub enum ImageSource {
    Local(Asset),
    Remote(&'static str),
}

impl std::fmt::Display for ImageSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let src = match self {
            ImageSource::Local(asset) => asset.to_string(),
            ImageSource::Remote(url) => url.to_string(),
        };
        write!(f, "{}", src)
    }
}

#[derive(Clone, PartialEq)]
pub struct Project {
    pub title: &'static str,
    pub image: ImageSource,
    pub description: &'static str,
    pub tech: &'static [Tech],
    pub repo: &'static str,
}
