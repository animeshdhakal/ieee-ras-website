use dioxus::prelude::*;

#[component]
pub fn HomeCTA() -> Element {
    rsx! {
        section { class: "py-20 bg-ieee-ras relative overflow-hidden",
            // Decorative Circles
            div { class: "absolute top-0 right-0 w-64 h-64 bg-white/5 rounded-full blur-3xl -translate-y-1/2 translate-x-1/2" }
            div { class: "absolute bottom-0 left-0 w-96 h-96 bg-black/10 rounded-full blur-3xl translate-y-1/2 -translate-x-1/2" }

            div { class: "max-w-4xl mx-auto px-4 text-center relative z-10",
                h2 { class: "text-3xl md:text-4xl font-bold text-white mb-6",
                    "Ready to Shape the Future?"
                }
                p { class: "text-rose-100 text-lg mb-8 max-w-2xl mx-auto",
                    "Join a global network of professionals and students. Enhance your skills, expand your network, and contribute to technological advancement in robotics and automation."
                }
                a {
                    href: "https://www.ieee-ras.org/membership/join",
                    target: "_blank",
                    rel: "noreferrer",
                    class: "inline-block px-8 py-4 bg-white text-ieee-ras rounded-full font-bold shadow-lg hover:shadow-xl hover:bg-gray-50 transition-all duration-300 transform hover:-translate-y-1",
                    "Become a Member"
                }
            }
        }
    }
}
