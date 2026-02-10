use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_brands_icons::{
    FaAngular, FaCss3Alt, FaDocker, FaGitAlt, FaJava, FaLinux, FaPython, FaReact, FaRust,
};
use dioxus_free_icons::Icon;

#[component]
pub fn Skills() -> Element {
    rsx! {
        div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 items-center",
            SkillCategoryCard { category: &FRONTEND }
            SkillCategoryCard { category: &BACKEND }
            SkillCategoryCard { category: &TOOLS }
        }
    }
}

#[component]
fn SkillCategoryCard(category: &'static SkillCategory) -> Element {
    rsx! {
        div { class: "card bg-base-200 shadow-xl m-4",
            div { class: "card-body",
                h3 { class: "card-title text-2xl mb-4", {category.name} }
                div { class: "space-y-4 justify-center",
                    for skill in category.skills {
                        div { class: "flex items-center space-x-4",
                            span { class: "text-3xl text-primary", {skill.icon()} }
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

impl Skill {
    fn icon(&self) -> Element {
        match self.icon {
            TechIcon::Angular => rsx!(
                Icon { icon: FaAngular }
            ),
            TechIcon::Css => rsx!(
                Icon { icon: FaCss3Alt }
            ),
            TechIcon::Docker => rsx!(
                Icon { icon: FaDocker }
            ),
            TechIcon::Git => rsx!(
                Icon { icon: FaGitAlt }
            ),
            TechIcon::Java => rsx!(
                Icon { icon: FaJava }
            ),
            TechIcon::Linux => rsx!(
                Icon { icon: FaLinux }
            ),
            TechIcon::Python => rsx!(
                Icon { icon: FaPython }
            ),
            TechIcon::React => rsx!(
                Icon { icon: FaReact }
            ),
            TechIcon::Rust => rsx!(
                Icon { icon: FaRust }
            ),
        }
    }
}

pub static FRONTEND: SkillCategory = SkillCategory {
    name: "Frontend",
    skills: &[
        Skill {
            name: "React/Next",
            icon: TechIcon::React,
            progress: 85,
        },
        Skill {
            name: "Angular",
            icon: TechIcon::Angular,
            progress: 75,
        },
        Skill {
            name: "Tailwindcss",
            icon: TechIcon::Css,
            progress: 80,
        },
    ],
};

pub static BACKEND: SkillCategory = SkillCategory {
    name: "Backend",
    skills: &[
        Skill {
            name: "Python",
            icon: TechIcon::Python,
            progress: 95,
        },
        Skill {
            name: "Rust",
            icon: TechIcon::Rust,
            progress: 95,
        },
        Skill {
            name: "Django",
            icon: TechIcon::Python,
            progress: 90,
        },
        Skill {
            name: "Java",
            icon: TechIcon::Java,
            progress: 75,
        },
    ],
};

pub static TOOLS: SkillCategory = SkillCategory {
    name: "Tools & DevOps",
    skills: &[
        Skill {
            name: "Git",
            icon: TechIcon::Git,
            progress: 90,
        },
        Skill {
            name: "Docker",
            icon: TechIcon::Docker,
            progress: 70,
        },
        Skill {
            name: "Linux",
            icon: TechIcon::Linux,
            progress: 80,
        },
    ],
};

#[derive(Clone, PartialEq)]
pub struct SkillCategory {
    pub name: &'static str,
    pub skills: &'static [Skill],
}

#[derive(Clone, PartialEq)]
pub struct Skill {
    pub name: &'static str,
    pub icon: TechIcon,
    pub progress: i32,
}

#[derive(Clone, PartialEq)]
pub enum TechIcon {
    Angular,
    Css,
    Docker,
    Git,
    Java,
    Linux,
    Python,
    React,
    Rust,
}
