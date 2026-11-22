use dioxus::prelude::*;
use crate::components::icons::{IconClock, IconX};
use crate::types::Blog;

#[component]
pub fn BlogModal(blog: Option<Blog>, on_close: EventHandler<()>) -> Element {
    let Some(ref b) = blog else {
        return rsx! {};
    };

    let html_content = {
        let parser = pulldown_cmark::Parser::new(&b.content);
        let mut html_output = String::new();
        pulldown_cmark::html::push_html(&mut html_output, parser);
        html_output
    };

    rsx! {
        div {
            class: "fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm animate-in fade-in duration-200",
            onclick: move |_| on_close.call(()),
            div {
                class: "bg-white rounded-3xl max-w-3xl w-full max-h-[90vh] overflow-y-auto shadow-2xl border border-gray-100 relative animate-in zoom-in-95 duration-200",
                onclick: move |e| e.stop_propagation(),

                // Close Button
                button {
                    onclick: move |_| on_close.call(()),
                    aria_label: "Close Article",
                    class: "absolute top-4 right-4 z-10 p-2 rounded-full bg-black/40 hover:bg-black/70 text-white backdrop-blur-md transition-all",
                    IconX { size: 20, color: "white".to_string() }
                }

                // Header Banner Image
                div { class: "relative h-64 w-full bg-gray-900 overflow-hidden",
                    img {
                        src: "{b.thumbnail}",
                        alt: "{b.title}",
                        class: "w-full h-full object-cover",
                    }
                    div { class: "absolute inset-0 bg-gradient-to-t from-black/85 via-black/30 to-transparent" }
                    div { class: "absolute bottom-5 left-6 right-6",
                        span { class: "px-3 py-1 rounded-full text-xs font-bold bg-rose-700 text-white shadow-sm inline-block mb-2",
                            "{b.category}"
                        }
                        h2 { class: "text-2xl sm:text-3xl font-bold text-white leading-tight",
                            "{b.title}"
                        }
                    }
                }

                // Meta Row
                div { class: "px-6 sm:px-8 py-4 bg-gray-50 border-b border-gray-100 flex flex-wrap items-center justify-between text-xs text-gray-500 gap-2",
                    div { class: "flex items-center space-x-2",
                        span { "By" }
                        span { class: "font-bold text-gray-900", "{b.author}" }
                        span { "·" }
                        span { class: "text-rose-700 font-semibold", "{b.author_role}" }
                    }
                    div { class: "flex items-center space-x-4",
                        span { "{b.date}" }
                        div { class: "flex items-center space-x-1 font-medium",
                            IconClock { size: 14, color: "currentColor".to_string() }
                            span { "{b.read_time}" }
                        }
                    }
                }

                // Article Body
                div { class: "p-6 sm:p-8 space-y-6 text-gray-800 leading-relaxed",
                    p { class: "text-base font-semibold text-gray-700 italic border-l-4 border-rose-600 pl-4 py-2 bg-rose-50/50 rounded-r-lg",
                        "{b.excerpt}"
                    }
                    div {
                        class: "blog-content prose max-w-none text-gray-700 leading-relaxed text-sm sm:text-base",
                        dangerous_inner_html: "{html_content}"
                    }
                }

                // Footer
                div { class: "p-6 bg-gray-50 border-t border-gray-100 flex justify-end",
                    button {
                        onclick: move |_| on_close.call(()),
                        class: "px-6 py-2.5 rounded-full bg-gray-900 hover:bg-black text-white font-semibold text-sm transition-colors",
                        "Back to Blogs"
                    }
                }
            }
        }
    }
}
