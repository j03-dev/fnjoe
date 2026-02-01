use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_regular_icons::{FaMoon, FaSun};
use dioxus_free_icons::Icon;

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
fn ThemeController() -> Element {
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
