use dioxus::prelude::*;
use crate::components::icons::{IconArrowRight, IconCalendar, IconMapPin};
use crate::types::IeeeEvent;

#[component]
pub fn EventCard(event: IeeeEvent, on_select: EventHandler<IeeeEvent>) -> Element {
    rsx! {
        div {
            onclick: {
                let e = event.clone();
                move |_| on_select.call(e.clone())
            },
            class: "flex flex-col h-full bg-white rounded-2xl overflow-hidden border border-gray-100 shadow-sm hover:shadow-lg transition-all duration-300 group cursor-pointer",
            // Image area
            div { class: "h-48 bg-gray-100 relative overflow-hidden",
                img {
                    src: "{event.image_url}",
                    alt: "{event.title}",
                    class: "w-full h-full object-cover transition-transform duration-500 group-hover:scale-105",
                }

                // Category Badge
                div { class: "absolute top-4 left-4 bg-white/90 backdrop-blur-sm px-3 py-1.5 rounded-lg shadow-sm border border-gray-100",
                    span { class: "text-xs font-bold text-gray-900 uppercase tracking-wide",
                        "{event.category}"
                    }
                }

                if event.is_upcoming {
                    div { class: "absolute top-4 right-4 bg-ieee-ras text-white px-2.5 py-1 rounded-md text-[10px] font-bold uppercase tracking-wider shadow-sm",
                        "Upcoming"
                    }
                }
            }

            div { class: "p-6 flex flex-col flex-grow",
                div { class: "flex items-center text-xs text-gray-500 mb-3 space-x-3",
                    div { class: "flex items-center",
                        IconCalendar { size: 14, color: "currentColor".to_string(), class: "mr-1.5 shrink-0" }
                        "{event.dates}"
                    }
                    div { class: "flex items-center",
                        IconMapPin { size: 14, color: "currentColor".to_string(), class: "mr-1.5 shrink-0" }
                        span { class: "truncate max-w-[140px]", "{event.location}" }
                    }
                }

                h3 { class: "text-xl font-bold text-gray-900 mb-2 group-hover:text-ieee-ras transition-colors duration-200 line-clamp-2",
                    "{event.title}"
                }

                p { class: "text-gray-600 text-sm leading-relaxed mb-6 line-clamp-3",
                    "{event.description}"
                }

                div { class: "mt-auto",
                    span { class: "inline-flex items-center text-sm font-semibold text-ieee-ras group-hover:text-ieee-ras-dark transition-colors duration-300",
                        "View Details "
                        IconArrowRight { size: 16, color: "currentColor".to_string(), class: "ml-1 group-hover:translate-x-1 transition-transform" }
                    }
                }
            }
        }
    }
}
