use dioxus::prelude::*;

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
                            a { href: "#timeline", "Timeline" }
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
                        a { href: "#timeline", "Timeline" }
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
    let themes = ["dracula", "halloween", "garden", "forest", "lofi", "night"];

    rsx! {
        div { class: "dropdown",
            div {
                tabindex: "0",
                role: "button",
                class: "btn m-1 text-primary",
                "Theme"
                svg {
                    width: "12px",
                    height: "12px",
                    class: "inline-block h-2 w-2 fill-current opacity-60",
                    xmlns: "http://www.w3.org/2000/svg",
                    view_box: "0 0 2048 2048",
                    path { d: "M1799 349l242 241-1017 1017L7 590l242-241 775 775 775-775z" }
                }
            }
            ul {
                tabindex: "-1",
                class: "dropdown-content bg-base-300 rounded-box w-52 p-2 shadow-2xl",
                for theme in themes {
                    ThemeItem { name: theme }
                }
            }
        }
    }
}

#[component]
fn ThemeItem(name: &'static str) -> Element {
    let value = name.to_lowercase();

    let set_theme = |theme: &str| {
        document::eval(&format!(
            r#"document.documentElement.setAttribute("data-theme", "{}");"#,
            theme
        ));
    };

    rsx! {
        li {
            input {
                r#type: "radio",
                name: "theme-dropdown",
                class: "theme-controller w-full btn btn-sm btn-block btn-ghost justify-start",
                aria_label: "{name}",
                value: "{value}",
                onchange: move |_| set_theme(&value),
            }
        }
    }
}
