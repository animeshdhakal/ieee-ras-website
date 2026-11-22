use dioxus::prelude::*;
use crate::components::icons::{IconArrowRight, IconCalendar, IconCpu, IconMapPin, IconUsers};
use crate::Route;

#[component]
pub fn HomeAbout() -> Element {
    rsx! {
        section { class: "py-24 bg-white relative z-10",
            div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                div { class: "grid grid-cols-1 md:grid-cols-2 gap-16 items-center",
                    // Left Column
                    div {
                        span { class: "text-ieee-ras font-semibold text-sm uppercase tracking-wider mb-2 block",
                            "New Student Branch Chapter"
                        }
                        h2 { class: "text-3xl font-bold text-gray-900 mb-6 tracking-tight",
                            "Who We Are"
                        }
                        p { class: "text-gray-600 text-lg leading-relaxed mb-6",
                            "IEEE Robotics & Automation Society (RAS) Pulchowk Student Branch Chapter is a newly chartered technical club at IOE Pulchowk Campus, established in 2026. We bring together passionate engineering students to build robotics projects, explore intelligent automation, and bridge academic theory with real-world engineering."
                        }
                        p { class: "text-gray-600 leading-relaxed mb-8",
                            "As a newly formed chapter under the global IEEE RAS umbrella, we are building our community from the ground up: organizing practical workshops, hands-on bootcamps, and preparing teams for national and international robotics challenges."
                        }
                        Link {
                            to: Route::Team {},
                            class: "text-ieee-ras font-semibold hover:underline inline-flex items-center group",
                            "Meet the 2026 Founding Team "
                            IconArrowRight { size: 16, color: "currentColor".to_string(), class: "ml-2 group-hover:translate-x-1 transition-transform" }
                        }
                    }

                    // Right Column: Realistic stats for the new chapter
                    div { class: "grid grid-cols-1 sm:grid-cols-2 gap-6",
                        div { class: "bg-gray-50 p-6 rounded-2xl border border-gray-100 text-center hover:border-ieee-ras/20 hover:shadow-sm transition-all",
                            div { class: "inline-flex items-center justify-center w-12 h-12 rounded-full bg-rose-100 text-ieee-ras mb-4",
                                IconCalendar { size: 24, color: "#862633".to_string() }
                            }
                            div { class: "text-3xl font-bold text-gray-900 mb-1", "2026" }
                            div { class: "text-sm text-gray-500 font-medium uppercase tracking-wide", "Founding Year" }
                        }

                        div { class: "bg-gray-50 p-6 rounded-2xl border border-gray-100 text-center hover:border-ieee-ras/20 hover:shadow-sm transition-all",
                            div { class: "inline-flex items-center justify-center w-12 h-12 rounded-full bg-rose-100 text-ieee-ras mb-4",
                                IconUsers { size: 24, color: "#862633".to_string() }
                            }
                            div { class: "text-3xl font-bold text-gray-900 mb-1", "21" }
                            div { class: "text-sm text-gray-500 font-medium uppercase tracking-wide", "Founding Committee" }
                        }

                        div { class: "bg-gray-50 p-6 rounded-2xl border border-gray-100 text-center hover:border-ieee-ras/20 hover:shadow-sm transition-all",
                            div { class: "inline-flex items-center justify-center w-12 h-12 rounded-full bg-rose-100 text-ieee-ras mb-4",
                                IconCpu { size: 24, color: "#862633".to_string() }
                            }
                            div { class: "text-3xl font-bold text-gray-900 mb-1", "5" }
                            div { class: "text-sm text-gray-500 font-medium uppercase tracking-wide", "Executive Teams" }
                        }

                        div { class: "bg-gray-50 p-6 rounded-2xl border border-gray-100 text-center hover:border-ieee-ras/20 hover:shadow-sm transition-all",
                            div { class: "inline-flex items-center justify-center w-12 h-12 rounded-full bg-rose-100 text-ieee-ras mb-4",
                                IconMapPin { size: 24, color: "#862633".to_string() }
                            }
                            div { class: "text-3xl font-bold text-gray-900 mb-1", "IOE" }
                            div { class: "text-sm text-gray-500 font-medium uppercase tracking-wide", "Pulchowk Campus" }
                        }
                    }
                }
            }
        }
    }
}
