#![allow(dead_code)]

use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_brands_icons::{
    FaAngular, FaCss3Alt, FaDocker, FaGitAlt, FaHtml5, FaJava, FaJsSquare, FaPython, FaRust,
};
use dioxus_free_icons::{Icon, IconShape};

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
fn SkillCategoryCard(categorie_name: String, skills: Vec<Skill>) -> Element {
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
struct Skill {
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
            icon: rsx!(Icon { icon }),
            progress,
        }
    }
}
