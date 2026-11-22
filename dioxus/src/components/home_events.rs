use dioxus::prelude::*;
use crate::components::event_card::EventCard;
use crate::components::icons::IconArrowRight;
use crate::types::IeeeEvent;
use crate::Route;

#[component]
pub fn HomeEvents(events: Vec<IeeeEvent>, on_select: EventHandler<IeeeEvent>) -> Element {
    rsx! {
        section { class: "py-24 bg-gray-50 border-t border-gray-200",
            div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                div { class: "flex justify-between items-end mb-12",
                    div {
                        span { class: "text-ieee-ras font-semibold text-sm uppercase tracking-wider mb-2 block",
                            "Mark Your Calendars"
                        }
                        h2 { class: "text-3xl font-bold text-gray-900",
                            "Upcoming Events"
                        }
                    }
                    Link {
                        to: Route::Events {},
                        class: "hidden sm:inline-flex items-center text-gray-600 hover:text-ieee-ras transition-colors font-medium",
                        "View All "
                        IconArrowRight { size: 18, color: "currentColor".to_string(), class: "ml-2" }
                    }
                }

                div { class: "grid grid-cols-1 md:grid-cols-3 gap-8",
                    if !events.is_empty() {
                        for event in events.iter() {
                            div { key: "{event.id}", class: "h-full",
                                EventCard {
                                    event: event.clone(),
                                    on_select: move |e| on_select.call(e),
                                }
                            }
                        }
                    } else {
                        div { class: "col-span-1 md:col-span-3 py-16 px-6 text-center max-w-lg mx-auto bg-white rounded-3xl border border-gray-100 shadow-sm",
                            div { class: "w-16 h-16 mx-auto mb-4 rounded-2xl bg-rose-50 text-ieee-ras flex items-center justify-center",
                                svg {
                                    class: "w-8 h-8",
                                    fill: "none",
                                    stroke: "currentColor",
                                    view_box: "0 0 24 24",
                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "1.5",
                                        d: "M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z",
                                    }
                                }
                            }
                            h3 { class: "text-xl font-bold text-gray-900 mb-2", "Stay Tuned for Upcoming Events" }
                            p { class: "text-gray-500 text-sm leading-relaxed",
                                "We are organizing hands-on robotics workshops, guest technical talks, and hackathons for the 2026 tenure year. Announcements will appear here soon!"
                            }
                        }
                    }
                }

                div { class: "mt-10 text-center sm:hidden",
                    Link {
                        to: Route::Events {},
                        class: "inline-flex items-center text-ieee-ras font-medium",
                        "View All Events "
                        IconArrowRight { size: 18, color: "currentColor".to_string(), class: "ml-2" }
                    }
                }
            }
        }
    }
}
