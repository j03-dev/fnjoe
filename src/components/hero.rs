use dioxus::prelude::*;

use dioxus_free_icons::icons::fa_brands_icons::{FaFacebook, FaGithub, FaLinkedin};
use dioxus_free_icons::icons::fa_regular_icons::{FaEnvelope, FaFilePdf};
use dioxus_free_icons::Icon;

#[component]
pub fn Hero() -> Element {
    rsx! {
        section { id: "hero", class: "hero bg-base-100 min-h-screen",
            div { class: "hero-content flex-col lg:flex-row-reverse",
                img {
                    class: "max-w-sm  rounded-full shadow-2xl border-2 border-primary liquid-border",
                    src: asset!("/assets/image.jpeg"),
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
                        button {
                            class: "btn btn-primary text-base-100",
                            onclick: move |_| {
                                document::eval("contact_modal.showModa()");
                            },
                            Icon { icon: FaEnvelope }
                            "Contact Me"
                        }
                        button { class: "btn btn-secondary text-base-100",
                            Icon { icon: FaFilePdf }
                            "Resume"
                        }
                        ContactModal {}
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
fn ContactModal() -> Element {
    rsx! {
        dialog { id: "contact_modal", class: "modal",
            div { class: "modal-box",
                form {
                    action: "https://api.web3forms.com/submit",
                    method: "POST",
                    class: "fieldset",

                    legend { class: "fieldset-legend", "Contact" }

                    input {
                        r#type: "hidden",
                        name: "access_key",
                        value: "2451497f-175b-4715-b067-3cdb11550f1a",
                    }

                    label { class: "label", "Name" }
                    input {
                        class: "input",
                        name: "name",
                        placeholder: "Add your name here!",
                    }

                    label { class: "label", "Email" }
                    input {
                        r#type: "email",
                        name: "email",
                        class: "input",
                        placeholder: "Add your email here!",
                    }

                    label { class: "label", "Message" }
                    textarea {
                        class: "textarea",
                        name: "message",
                        placeholder: "Type your message here!",
                    }

                    button {
                        r#type: "submit",
                        class: "btn btn-success btn-outline",
                        "Submit"
                    }
                }
            }
        }
    }
}
