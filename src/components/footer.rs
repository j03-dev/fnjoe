use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::fa_brands_icons::{FaFacebook, FaGithub, FaLinkedin},
    Icon,
};

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer { class: "footer footer-horizontal footer-center bg-base-100 text-base-content rounded p-10",
            nav { class: "grid grid-flow-col gap-4",
                a { class: "link link-hover", href: "#about", "About" }
                a { class: "link link-hover", href: "#skills", "Skills" }
                a { class: "link link-hover", href: "#projects", "Projects" }
                a {
                    class: "link link-hover",
                    onclick: move |_| {
                        document::eval("contact_modal.showModal()");
                    },
                    "Contact Me"
                }
            }
            nav {
                div { class: "grid grid-flow-col gap-4",
                    a {
                        href: "https://github.com/j03-dev",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        Icon { icon: FaGithub, width: 28, height: 28 }
                    }
                    a {
                        href: "https://www.linkedin.com/in/nomeniavo-joe-fitahiana",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        Icon { icon: FaLinkedin, width: 28, height: 28 }
                    }
                    a {
                        href: "https://www.facebook.com/profile.php?id=100006864466268",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        Icon { icon: FaFacebook, width: 28, height: 28 }
                    }
                }
            }
            aside {
                p { "Copyright © 2026 - All right reserved by FITAHIANA Nomeniavo Joe" }
            }
        }
    }
}
