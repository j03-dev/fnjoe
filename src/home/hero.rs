use dioxus::prelude::*;

use dioxus_free_icons::icons::fa_brands_icons::{FaFacebook, FaGithub, FaLinkedin};
use dioxus_free_icons::icons::fa_regular_icons::{FaCircleDown, FaCircleXmark, FaEnvelope};
use dioxus_free_icons::Icon;

#[component]
pub fn Hero() -> Element {
    rsx! {
        div { class: "hero-content flex-col lg:flex-row-reverse px-4 md:px-8",
            img {
                class: "w-[70%] md:max-w-sm rounded-full shadow-2xl border-2 border-primary liquid-border",
                src: asset!("/assets/image.jpeg"),
                loading: "lazy",
            }
            div { class: "mt-8 lg:mt-0 w-full",
                h1 { class: "text-3xl sm:text-4xl lg:text-5xl text-center lg:text-left font-bold",
                    "FullStack "
                    span { class: "text-primary", "Developer" }
                }
                p { class: "py-4 md:py-6 text-sm sm:text-base text-center lg:text-left",
                    "Salama! I'm "
                    b { "FITAHIANA Nomeniavo Joe " }
                    "a Fullstack Developer originating from the vibrant island of Madagascar.
                         With a fervent passion for technology, I navigate the digital realm with ease,
                         sculpting solutions with lines of code. I am committed to streamlining the intricacies of digital communication.
                         Join me on this exhilarating journey where innovation converges with code,
                         as together, we shape the future of technology, one byte at a time."
                }
                div { class: "flex flex-col sm:flex-row justify-center lg:justify-start gap-3 sm:gap-4 mt-4",
                    button {
                        class: "btn btn-primary text-base-100 w-full sm:w-auto",
                        onclick: move |_| {
                            document::eval("contact_modal.showModal()");
                        },
                        Icon { icon: FaEnvelope }
                        "Contact Me"
                    }
                    a { href: "https://drive.google.com/file/d/1UjeYxf2ds2yXZP5wAVVJiX9sXGyTcntf/view?usp=drive_link",
                        button { class: "btn btn-secondary text-base-100 w-full sm:w-auto",
                            Icon { icon: FaCircleDown }
                            "Resume"
                        }
                    }
                }
                div { class: "flex mt-5 text-2xl text-gray-500 gap-4 justify-center lg:justify-start",
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
                ContactModal {}
            }
        }
    }
}

#[component]
fn ContactModal() -> Element {
    rsx! {
        dialog { id: "contact_modal", class: "modal",
            div { class: "modal-box",
                form { method: "dialog",
                    button { class: "btn btn-sm btn-circle btn-ghost absolute right-4 top-7",
                        Icon { icon: FaCircleXmark }
                    }
                }

                form {
                    action: "https://api.web3forms.com/submit",
                    method: "POST",
                    class: "fieldset",

                    legend { class: "fieldset-legend", "Contact Me" }

                    input {
                        r#type: "hidden",
                        name: "access_key",
                        value: "2451497f-175b-4715-b067-3cdb11550f1a",
                    }

                    label { class: "label", "Name" }
                    input {
                        class: "input w-full",
                        name: "name",
                        placeholder: "Add your name here!",
                    }

                    label { class: "label", "Email" }
                    input {
                        r#type: "email",
                        name: "email",
                        class: "input w-full",
                        placeholder: "Add your email here!",
                    }

                    label { class: "label", "Message" }
                    textarea {
                        class: "textarea w-full",
                        name: "message",
                        placeholder: "Type your message here!",
                    }

                    button { r#type: "submit", class: "btn btn-accent mt-5", "Submit" }
                }
            }
        }
    }
}
