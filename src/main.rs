use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_brands_icons::{
    FaAngular, FaBootstrap, FaCss3Alt, FaDocker, FaFacebook, FaGitAlt, FaGithub, FaHtml5, FaJava,
    FaJs, FaJsSquare, FaLinkedin, FaPython, FaReact, FaRust,
};
use dioxus_free_icons::icons::fa_regular_icons::{FaEnvelope, FaFilePdf, FaMoon, FaSun};
use dioxus_free_icons::{Icon, IconShape};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const PROFILE: Asset = asset!("/assets/image.jpeg");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        NavBar {}
        Hero {}
        Skills {}
        Projects {}
    }
}

#[component]
pub fn NavBar() -> Element {
    rsx! {
        nav { class: "navbar bg-base-100 shadow-sm text-primary",
            div { class: "navbar-start",
                div { class: "dropdown",
                    div {
                        tabindex: "0",
                        role: "button",
                        class: "btn btn-ghost lg:hidden",
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            class: "h-5 w-5",
                            fill: "none",
                            view_box: "0 0 24 24",
                            stroke: "currentColor",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                d: "M4 6h16M4 12h8m-8 6h16",
                            }
                        }
                    }
                    ul {
                        tabindex: "-1",
                        class: "menu menu-sm dropdown-content bg-base-100 rounded-box z-1 mt-3 w-52 p-2 shadow",
                        li {
                            a { href: "#hero", "About" }
                        }
                        li {
                            a { href: "#skills", "Skills" }
                        }
                        li {
                            a { href: "#projects", "Projects" }
                        }
                    }
                }
                a { class: "btn btn-ghost text-xl", "FN.JOE" }
            }
            div { class: "navbar-center  hidden lg:flex",
                ul { tabindex: "-1", class: "menu menu-horizontal px-1",
                    li {
                        a { href: "#hero", "About" }
                    
                    }
                    li {
                        a { href: "#skills", "Skills" }
                    }
                    li {
                        a { href: "#projects", "Projects" }
                    }
                }
            
            }
            div { class: "navbar-end", ThemeController {} }
        }
    }
}

#[component]
pub fn ThemeController() -> Element {
    let mut theme = use_signal(|| "light".to_string());

    use_effect(move || {
        let _ = document::eval(&format!(
            r#"document.documentElement.setAttribute('data-theme', '{}')"#,
            theme()
        ));
    });

    rsx! {
        label { class: "toggle text-primary text-2xl",
            input {
                r#type: "checkbox",
                value: "synthwave",
                class: "theme-controller",
                onclick: move |_| {
                    let new_theme = if theme() == "light" { "dark" } else { "light" };
                    theme.set(new_theme.to_string());
                },
            }
            Icon { icon: FaSun }
            Icon { icon: FaMoon }
        }
    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        section { id: "hero", class: "hero bg-base-100 min-h-screen",
            div { class: "hero-content flex-col lg:flex-row-reverse",
                img {
                    class: "max-w-sm  rounded-full shadow-2xl border-2 border-primary liquid-border",
                    src: PROFILE,
                }
                div { class: "mt-5 md:mt-0",
                    h1 { class: "text-5xl text-center md:text-left font-bold",
                        span { "FullStack" }
                        span { class: "text-primary", "Developer" }
                    }
                    p { class: "py-6",
                        span { "Salama! I'm " }
                        b { "FITAHIANA Nomeniavo Joe " }
                        span {
                            "a Fullstack Developer originating from the vibrant island of Madagascar.
                             With a fervent passion for technology, I navigate the digital realm with ease,
                             sculpting solutions with lines of code. I am committed to streamlining the intricacies of digital communication.
                             Join me on this exhilarating journey where innovation converges with code,
                             as together, we shape the future of technology, one byte at a time."
                        }
                    }
                    div { class: "flex justify-center md:justify-start gap-4",
                        button { class: "btn btn-primary text-base-100",
                            Icon { icon: FaEnvelope }
                            "Contact Me"
                        }
                        button { class: "btn btn-secondary text-base-100",
                            Icon { icon: FaFilePdf }
                            "Resume"
                        }
                    }
                    div { class: "flex mt-5 text-2xl text-gray-500 gap-2",
                        a {
                            href: "https://github.com/j03-dev",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            Icon { icon: FaGithub }
                        }
                        a {
                            href: "https://www.linkedin.com/in/nomeniavo-joe-fitahiana",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            Icon { icon: FaLinkedin }
                        }
                        a {
                            href: "https://www.facebook.com/profile.php?id=100006864466268",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            Icon { icon: FaFacebook }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn Skills() -> Element {
    let frontend_skills = vec![
        Skill::new("Angular", FaAngular, 85),
        Skill::new("HTML5", FaHtml5, 90),
        Skill::new("CSS3", FaCss3Alt, 85),
        Skill::new("TypeScript", FaJsSquare, 80),
    ];

    let backend_skills = vec![
        Skill::new("Python", FaPython, 95),
        Skill::new("Rust", FaRust, 95),
        Skill::new("Django", FaPython, 90),
        Skill::new("Java", FaJava, 75),
    ];

    let tools_dev_ops_skills = vec![
        Skill::new("Git", FaGitAlt, 90),
        Skill::new("Docker", FaDocker, 70),
    ];

    rsx! {
        div { id: "skills", class: "min-h-screen bg-base-200 py-8",
            div { class: "container mx-auto p-4",
                div { class: "text-center p-4",
                    h2 { class: "text-3xl font-bold",
                        span { "Technical" }
                        span { class: "text-primary", "Skill" }
                    }
                    p { class: "text-gray-500 mt-2",
                        "A showcase of my technical proficiencies and expertise."
                    }
                }
                div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 items-center pt-20",
                    SkillCategoryCard { categorie_name: "Frontend", skills: frontend_skills }
                    SkillCategoryCard { categorie_name: "Backend", skills: backend_skills }
                    SkillCategoryCard {
                        categorie_name: "Tools & DevOps",
                        skills: tools_dev_ops_skills,
                    }
                }
            }
        }
    }
}

#[component]
pub fn SkillCategoryCard(categorie_name: String, skills: Vec<Skill>) -> Element {
    rsx! {
        div { class: "card bg-base-200 shadow-xl m-4",
            div { class: "card-body",
                h3 { class: "card-title text-2xl mb-4", {categorie_name} }
                div { class: "space-y-4 justify-center",
                    for skill in skills {
                        div { class: "flex items-center space-x-4",
                            span { class: "text-3xl text-primary", {skill.icon} }
                            div { class: "flex-grow",
                                div { class: "flex justify-between",
                                    span { class: "font-semibold", {skill.name} }
                                    span { {skill.progress.to_string()} }
                                }
                                progress {
                                    class: "progress progress-primary w-full",
                                    value: skill.progress,
                                    max: "100",
                                }
                            }
                        }
                    }
                
                }
            }
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Skill {
    name: &'static str,
    icon: Element,
    progress: i32,
}

impl Skill {
    fn new<T: IconShape + PartialEq + Clone + 'static>(
        name: &'static str,
        icon: T,
        progress: i32,
    ) -> Self {
        Self {
            name,
            icon: rsx!(
                Icon { icon }
            ),
            progress,
        }
    }
}

#[component]
pub fn Projects() -> Element {
    let projects =  vec![
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
            "Joe's blog is a dynamic platform built with my custom Oxapy library, offering fast routing and middleware support, and powered by Django's templates for sharing tech insights and personal experiences.",
            vec![rsx!(Icon { icon: FaPython }), rsx!(Icon { icon: FaHtml5 }), rsx!(Icon { icon: FaBootstrap})],
        ),

        Project::new(
            "Metatype",
            Url::Remote("https://opengraph.githubassets.com/1/metatypedev/metatype"),
            "Metatype is a declarative platform for API development using WebAssembly, TypeScript, and Python for modular backend components.",
            vec![rsx!(Icon { icon: FaPython }), rsx!(Icon { icon: FaAngular })],
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
            vec![rsx!(Icon { icon: FaPython }), rsx!(Icon { icon: FaRust }),
            ],
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
            vec![rsx!(Icon { icon: FaRust }),
            ],
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
            vec![rsx!(Icon { icon: FaPython }), rsx!(Icon { icon: FaRust })],
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
        section { id: "projects", class: "min-h-screen bg-base-100 py-8",
            div { class: "text-center p-4",
                h2 { class: "text-3xl font-bold",
                    span { "Featured" }
                    span { class: "text-primary", "Project" }
                }
                p { class: "text-gray-500 mt-2", "Discover some of my notable works and creations." }
            }
            div { class: "mx-auto w-full max-w-7xl grid grid-cols-1 gap-5 md:grid-cols-3 md:mt-10 md:pt-10 md:pb-10",
                for project in projects {
                    ProjectCard { project }
                }
            }
        }
    }
}

#[component]
pub fn ProjectCard(project: Project) -> Element {
    let image: Element = match project.image {
        Url::Local(asset) => rsx!(
            img { src: asset }
        ),
        Url::Remote(url) => rsx!(
            img { src: url }
        ),
    };
    
    rsx! {
        div { class: "card bg-base-200 w-full h-full w-96 shadow-sm",
            figure { {image} }
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
pub enum Url {
    Local(Asset),
    Remote(&'static str),
}

#[derive(PartialEq, Clone)]
pub struct Project {
    pub title: &'static str,
    pub image: Url,
    pub description: &'static str,
    pub techno_icons: Vec<Element>,
}

impl Project {
    pub fn new(
        title: &'static str,
        image: Url,
        description: &'static str,
        techon_icons: Vec<Element>,
    ) -> Self {
        Self {
            title,
            image,
            description,
            techno_icons: techon_icons.iter().map(|elt| rsx!(
                {elt}
            )).collect(),
        }
    }
}
