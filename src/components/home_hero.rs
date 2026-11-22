use dioxus::prelude::*;
use crate::components::icons::IconChevronRight;
use crate::Route;

#[component]
pub fn HomeHero() -> Element {
    use_effect(move || {
        let _ = document::eval("setTimeout(function() { if (window.initHeroParticles) window.initHeroParticles(); }, 100);");
    });

    rsx! {
        section { class: "relative h-[100vh] flex items-center justify-center overflow-hidden bg-ieee-dark",
            // Aurora glows behind particles
            div { class: "absolute inset-0 z-0 pointer-events-none",
                div { class: "absolute -top-1/4 left-1/5 h-[38rem] w-[38rem] rounded-full bg-rose-700/30 blur-[130px] animate-aurora" }
                div { class: "absolute top-1/3 right-1/6 h-[34rem] w-[34rem] rounded-full bg-purple-700/25 blur-[130px] animate-aurora-slow [animation-delay:-6s]" }
                div { class: "absolute bottom-0 left-1/3 h-[30rem] w-[30rem] rounded-full bg-rose-900/25 blur-[130px] animate-aurora [animation-delay:-11s]" }
            }

            // Interactive Canvas Particle Field
            canvas {
                id: "hero-particles-canvas",
                class: "absolute inset-0 w-full h-full z-0 opacity-70 pointer-events-auto",
            }

            // Vignette overlay
            div { class: "absolute inset-0 bg-gradient-to-b from-transparent via-ieee-dark/20 to-ieee-dark/90 pointer-events-none" }

            // Content
            div { class: "relative z-10 max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 text-center",
                div { class: "inline-flex items-center gap-2.5 mb-6 text-rose-200/90",
                    span { class: "h-1.5 w-1.5 rounded-full bg-rose-400" }
                    span { class: "text-sm font-medium tracking-wide",
                        "IEEE Robotics & Automation Society · Pulchowk Campus, Lalitpur"
                    }
                }

                h1 { class: "text-5xl md:text-7xl font-bold text-white tracking-tight leading-tight mb-6",
                    "Advancing Technology"
                    br {}
                    span { class: "text-transparent bg-clip-text bg-gradient-to-r from-rose-200 via-rose-300 to-rose-400",
                        "for Humanity"
                    }
                }

                p { class: "text-lg md:text-xl text-gray-300 max-w-2xl mx-auto mb-10 leading-relaxed font-light",
                    "A student-run chapter at IOE Pulchowk Campus, where curious engineers come together to build robotics projects, run workshops, and learn by doing."
                }

                div { class: "flex flex-col sm:flex-row items-center justify-center gap-4",
                    a {
                        href: "https://www.ieee-ras.org/membership/join",
                        target: "_blank",
                        rel: "noreferrer",
                        class: "w-full sm:w-auto px-8 py-3.5 bg-ieee-ras text-white rounded-full font-semibold hover:bg-rose-800 transition-all duration-300 shadow-lg hover:shadow-rose-900/25 flex items-center justify-center",
                        "Join IEEE RAS"
                    }
                    Link {
                        to: Route::Events {},
                        class: "w-full sm:w-auto px-8 py-3.5 bg-white/10 backdrop-blur-sm border border-white/20 text-white rounded-full font-semibold hover:bg-white/20 transition-all duration-300 flex items-center justify-center group",
                        "Explore Events "
                        IconChevronRight {
                            size: 18,
                            color: "white".to_string(),
                            class: "ml-2 group-hover:translate-x-1 transition-transform",
                        }
                    }
                }
            }
        }
    }
}
