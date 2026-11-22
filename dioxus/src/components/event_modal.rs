use dioxus::prelude::*;
use crate::components::icons::{IconCalendar, IconExternalLink, IconMapPin, IconX};
use crate::types::IeeeEvent;

#[component]
pub fn EventModal(event: Option<IeeeEvent>, on_close: EventHandler<()>) -> Element {
    let Some(ref ev) = event else {
        return rsx! {};
    };

    let category_badge_class = match ev.category.as_str() {
        "Competition" => "bg-amber-100 text-amber-900 border-amber-200",
        "Workshop" => "bg-rose-100 text-rose-900 border-rose-200",
        "Talk" => "bg-purple-100 text-purple-900 border-purple-200",
        _ => "bg-blue-100 text-blue-900 border-blue-200",
    };

    rsx! {
        div {
            class: "fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm animate-in fade-in duration-200",
            onclick: move |_| on_close.call(()),
            div {
                class: "bg-white rounded-3xl max-w-2xl w-full max-h-[90vh] overflow-y-auto shadow-2xl border border-gray-100 relative animate-in zoom-in-95 duration-200",
                onclick: move |e| e.stop_propagation(),

                // Close Button
                button {
                    onclick: move |_| on_close.call(()),
                    aria_label: "Close Modal",
                    class: "absolute top-4 right-4 z-10 p-2 rounded-full bg-black/40 hover:bg-black/70 text-white backdrop-blur-md transition-all",
                    IconX { size: 20, color: "white".to_string() }
                }

                // Modal Header Image
                div { class: "relative h-64 w-full bg-gray-900 overflow-hidden",
                    img {
                        src: "{ev.image_url}",
                        alt: "{ev.title}",
                        class: "w-full h-full object-cover",
                    }
                    div { class: "absolute inset-0 bg-gradient-to-t from-black/80 via-black/20 to-transparent" }
                    div { class: "absolute bottom-4 left-6 right-6",
                        span { class: "px-3 py-1 rounded-full text-xs font-bold border {category_badge_class} shadow-sm inline-block mb-2",
                            "{ev.category}"
                        }
                        h2 { class: "text-2xl font-bold text-white leading-tight",
                            "{ev.title}"
                        }
                    }
                }

                // Modal Body
                div { class: "p-6 sm:p-8 space-y-6",
                    // Date & Location Info Row
                    div { class: "grid grid-cols-1 sm:grid-cols-2 gap-4 p-4 rounded-xl bg-gray-50 border border-gray-100 text-sm",
                        div { class: "flex items-center space-x-3 text-gray-700",
                            IconCalendar { size: 18, color: "#be123c".to_string() }
                            div {
                                span { class: "text-xs text-gray-400 block", "Date & Duration" }
                                span { class: "font-semibold text-gray-900", "{ev.dates}" }
                            }
                        }
                        div { class: "flex items-center space-x-3 text-gray-700",
                            IconMapPin { size: 18, color: "#be123c".to_string() }
                            div {
                                span { class: "text-xs text-gray-400 block", "Venue" }
                                span { class: "font-semibold text-gray-900", "{ev.location}" }
                            }
                        }
                    }

                    // Full Content / Details
                    div { class: "prose max-w-none text-gray-700 leading-relaxed space-y-3",
                        p { class: "text-base font-medium text-gray-900", "{ev.description}" }
                        div { class: "whitespace-pre-line text-sm text-gray-600 border-t border-gray-100 pt-4",
                            "{ev.details}"
                        }
                    }

                    // Footer actions
                    div { class: "pt-4 border-t border-gray-100 flex flex-col sm:flex-row justify-between items-center gap-4",
                        if let Some(ref reg_url) = ev.registration_url {
                            a {
                                href: "{reg_url}",
                                target: "_blank",
                                rel: "noreferrer",
                                class: "w-full sm:w-auto px-6 py-3 rounded-full bg-gradient-to-r from-rose-700 to-purple-800 text-white font-bold text-sm shadow-lg hover:shadow-rose-500/25 transition-all flex items-center justify-center gap-2",
                                "Register for Event"
                                IconExternalLink { size: 16, color: "white".to_string() }
                            }
                        } else {
                            span { class: "text-xs text-gray-500 italic", "Event completed / archived" }
                        }

                        button {
                            onclick: move |_| on_close.call(()),
                            class: "w-full sm:w-auto px-6 py-3 rounded-full bg-gray-100 hover:bg-gray-200 text-gray-700 font-semibold text-sm transition-colors",
                            "Close Window"
                        }
                    }
                }
            }
        }
    }
}
