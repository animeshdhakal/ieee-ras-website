use dioxus::prelude::*;
use crate::components::event_card::EventCard;
use crate::components::event_modal::EventModal;
use crate::data::events::get_all_events;
use crate::types::IeeeEvent;

#[component]
pub fn Events() -> Element {
    let mut filter = use_signal(|| "All".to_string());
    let mut view = use_signal(|| "upcoming".to_string());
    let mut selected_event = use_signal(|| None::<IeeeEvent>);

    let events = get_all_events();
    let categories = vec!["All", "Workshop", "Competition", "Event", "Talk"];

    let current_view = view.read().clone();
    let current_filter = filter.read().clone();

    let filtered_events: Vec<IeeeEvent> = events
        .into_iter()
        .filter(|event| {
            let matches_category = current_filter == "All" || event.category == current_filter;
            let matches_view = if current_view == "upcoming" {
                event.is_upcoming
            } else {
                !event.is_upcoming
            };
            matches_category && matches_view
        })
        .collect();

    rsx! {
        div { class: "pt-24 pb-20 min-h-screen bg-gray-50",
            div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                // Header
                div { class: "text-center mb-12",
                    h1 { class: "text-4xl font-bold text-gray-900 mb-4",
                        "Events & Activities"
                    }
                    p { class: "text-gray-600 max-w-2xl mx-auto",
                        "Discover robotics bootcamps, workshops, seminars, and competitions designed to boost your engineering skills and technical growth."
                    }
                }

                // Controls Row
                div { class: "flex flex-col md:flex-row justify-between items-center mb-10 gap-6",
                    // View Toggle (Upcoming / Past Events)
                    div { class: "bg-white p-1 rounded-lg border border-gray-200 shadow-sm inline-flex",
                        button {
                            onclick: move |_| view.set("upcoming".to_string()),
                            class: {
                                if *view.read() == "upcoming" {
                                    "px-6 py-2 rounded-md text-sm font-medium transition-all bg-ieee-ras text-white shadow-sm"
                                } else {
                                    "px-6 py-2 rounded-md text-sm font-medium transition-all text-gray-600 hover:bg-gray-50"
                                }
                            },
                            "Upcoming"
                        }
                        button {
                            onclick: move |_| view.set("past".to_string()),
                            class: {
                                if *view.read() == "past" {
                                    "px-6 py-2 rounded-md text-sm font-medium transition-all bg-ieee-ras text-white shadow-sm"
                                } else {
                                    "px-6 py-2 rounded-md text-sm font-medium transition-all text-gray-600 hover:bg-gray-50"
                                }
                            },
                            "Past Events"
                        }
                    }

                    // Category Filter Pills
                    div { class: "flex items-center space-x-2 overflow-x-auto max-w-full pb-2 md:pb-0 no-scrollbar",
                        for cat in categories.iter() {
                            {
                                let is_active = *filter.read() == *cat;
                                let btn_class = if is_active {
                                    "bg-gray-900 text-white border-gray-900"
                                } else {
                                    "bg-white text-gray-600 border-gray-200 hover:border-gray-300"
                                };
                                let cat_str = cat.to_string();
                                rsx! {
                                    button {
                                        key: "{cat}",
                                        onclick: move |_| filter.set(cat_str.clone()),
                                        class: "px-4 py-1.5 rounded-full text-xs font-medium border transition-all whitespace-nowrap {btn_class}",
                                        "{cat}"
                                    }
                                }
                            }
                        }
                    }
                }

                // Grid
                if !filtered_events.is_empty() {
                    div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8",
                        for event in filtered_events.iter() {
                            div { key: "{event.id}",
                                EventCard {
                                    event: event.clone(),
                                    on_select: move |e| selected_event.set(Some(e)),
                                }
                            }
                        }
                    }
                } else {
                    div { class: "text-center py-20 px-6 max-w-lg mx-auto bg-white rounded-3xl border border-gray-100 shadow-sm",
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
                        h3 { class: "text-2xl font-bold text-gray-900 mb-2", "No Events Scheduled" }
                        p { class: "text-gray-500 text-sm leading-relaxed",
                            "New robotics bootcamps, technical workshops, and competitions will be scheduled here soon. Check back shortly!"
                        }
                    }
                }
            }

            // Event Modal
            EventModal {
                event: selected_event.read().clone(),
                on_close: move |_| selected_event.set(None),
            }
        }
    }
}
