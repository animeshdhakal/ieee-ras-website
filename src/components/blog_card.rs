use dioxus::prelude::*;
use crate::components::icons::{IconArrowRight, IconClock};
use crate::types::Blog;

#[component]
pub fn BlogCard(blog: Blog, #[props(default = false)] featured: bool, on_select: EventHandler<Blog>) -> Element {
    if featured {
        return rsx! {
            div {
                onclick: {
                    let b = blog.clone();
                    move |_| on_select.call(b.clone())
                },
                class: "group relative grid grid-cols-1 md:grid-cols-2 gap-0 md:gap-8 bg-white rounded-2xl overflow-hidden shadow-sm border border-gray-100 hover:shadow-md transition-all duration-300 cursor-pointer",
                div { class: "h-64 md:h-full bg-gray-100 relative overflow-hidden",
                    img {
                        src: "{blog.thumbnail}",
                        alt: "{blog.title}",
                        class: "w-full h-full object-cover transition-transform duration-500 group-hover:scale-105",
                    }
                    div { class: "absolute top-4 left-4 bg-white/90 backdrop-blur-sm px-3 py-1 rounded-full text-xs font-bold text-ieee-ras uppercase tracking-wide",
                        "{blog.category}"
                    }
                }

                div { class: "p-8 flex flex-col justify-center",
                    div { class: "flex items-center text-sm text-gray-500 mb-4 space-x-4",
                        span { class: "flex items-center",
                            "{blog.author}"
                        }
                        span { class: "flex items-center",
                            IconClock { size: 14, color: "currentColor".to_string(), class: "mr-1.5" }
                            "{blog.read_time}"
                        }
                    }

                    h2 { class: "text-2xl md:text-3xl font-bold text-gray-900 mb-4 group-hover:text-ieee-ras transition-colors",
                        "{blog.title}"
                    }

                    p { class: "text-gray-600 mb-6 leading-relaxed line-clamp-3",
                        "{blog.excerpt}"
                    }

                    div {
                        span { class: "inline-flex items-center text-ieee-ras font-semibold hover:text-ieee-ras-dark transition-colors",
                            "Read Article "
                            IconArrowRight { size: 18, color: "currentColor".to_string(), class: "ml-2 group-hover:translate-x-1 transition-transform" }
                        }
                    }
                }
            }
        };
    }

    rsx! {
        div {
            onclick: {
                let b = blog.clone();
                move |_| on_select.call(b.clone())
            },
            class: "group bg-white rounded-xl overflow-hidden border border-gray-100 shadow-sm hover:shadow-lg transition-all duration-300 flex flex-col h-full cursor-pointer",
            div { class: "h-48 bg-gray-100 relative overflow-hidden",
                img {
                    src: "{blog.thumbnail}",
                    alt: "{blog.title}",
                    class: "w-full h-full object-cover transition-transform duration-500 group-hover:scale-105",
                }
                div { class: "absolute top-4 left-4 bg-white/90 backdrop-blur-sm px-2.5 py-1 rounded-md text-[10px] font-bold text-ieee-ras uppercase tracking-wide",
                    "{blog.category}"
                }
            }

            div { class: "p-6 flex flex-col flex-grow",
                div { class: "flex items-center justify-between text-xs text-gray-500 mb-3",
                    span { "{blog.date}" }
                    span { "{blog.read_time}" }
                }

                h3 { class: "text-xl font-bold text-gray-900 mb-2 leading-tight group-hover:text-ieee-ras transition-colors line-clamp-2",
                    "{blog.title}"
                }

                p { class: "text-gray-600 text-sm mb-4 flex-grow line-clamp-3",
                    "{blog.excerpt}"
                }

                div { class: "mt-auto pt-4 border-t border-gray-50 flex items-center justify-between text-xs",
                    span { class: "text-gray-500 font-medium", "{blog.author}" }
                    span { class: "text-ieee-ras font-semibold inline-flex items-center",
                        "Read "
                        IconArrowRight { size: 14, color: "currentColor".to_string(), class: "ml-1 group-hover:translate-x-0.5 transition-transform" }
                    }
                }
            }
        }
    }
}
