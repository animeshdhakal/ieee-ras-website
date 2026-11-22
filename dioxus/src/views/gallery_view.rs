use dioxus::prelude::*;
use crate::components::gallery_modal::GalleryModal;
use crate::data::gallery::get_all_gallery_items;
use crate::types::GalleryItem;

#[component]
pub fn Gallery() -> Element {
    let mut selected_image = use_signal(|| None::<GalleryItem>);
    let mut filter = use_signal(|| "All".to_string());

    let all_items = get_all_gallery_items();
    let categories = vec!["All", "Workshop", "Competition", "Event", "Meetup", "Talk"];

    let current_filter = filter.read().clone();
    let filtered_images: Vec<GalleryItem> = all_items
        .into_iter()
        .filter(|item| current_filter == "All" || item.category == current_filter)
        .collect();

    rsx! {
        div { class: "pt-24 pb-20 min-h-screen bg-white",
            div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                // Header
                div { class: "text-center mb-12",
                    h1 { class: "text-4xl font-bold text-gray-900 mb-4", "Gallery" }
                    p { class: "text-gray-600 max-w-2xl mx-auto",
                        "A visual journey through our robotics builds, events, workshops, and community gatherings."
                    }
                }

                // Filter Buttons
                div { class: "flex justify-center mb-12",
                    div { class: "flex flex-wrap justify-center gap-2",
                        for cat in categories.iter() {
                            {
                                let is_active = *filter.read() == *cat;
                                let btn_class = if is_active {
                                    "bg-ieee-ras text-white shadow-md"
                                } else {
                                    "bg-gray-100 text-gray-600 hover:bg-gray-200"
                                };
                                let cat_str = cat.to_string();
                                rsx! {
                                    button {
                                        key: "{cat}",
                                        onclick: move |_| filter.set(cat_str.clone()),
                                        class: "px-4 py-2 rounded-full text-sm font-medium transition-all {btn_class}",
                                        "{cat}"
                                    }
                                }
                            }
                        }
                    }
                }

                // Masonry Columns Grid (matching original columns-1 md:columns-2 lg:columns-3)
                if !filtered_images.is_empty() {
                    div { class: "columns-1 md:columns-2 lg:columns-3 gap-6 space-y-6",
                        for item in filtered_images.iter() {
                            div {
                                key: "{item.id}",
                                onclick: {
                                    let img_item = item.clone();
                                    move |_| selected_image.set(Some(img_item.clone()))
                                },
                                class: "break-inside-avoid relative group rounded-xl overflow-hidden cursor-pointer bg-gray-100",
                                img {
                                    src: "{item.image_url}",
                                    alt: "{item.title}",
                                    loading: "lazy",
                                    class: "w-full h-auto object-cover transform transition-transform duration-700 group-hover:scale-105",
                                }

                                // Overlay
                                div { class: "absolute inset-0 bg-gradient-to-t from-black/80 via-black/20 to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-300 flex flex-col justify-end p-6",
                                    span { class: "text-rose-300 text-xs font-bold uppercase tracking-wider mb-1",
                                        "{item.category} • {item.date}"
                                    }
                                    h3 { class: "text-white font-bold text-lg", "{item.title}" }

                                    div { class: "absolute top-4 right-4 bg-white/20 backdrop-blur-md p-2 rounded-full",
                                        svg {
                                            width: "18",
                                            height: "18",
                                            view_box: "0 0 24 24",
                                            fill: "none",
                                            stroke: "white",
                                            stroke_width: "2",
                                            stroke_linecap: "round",
                                            stroke_linejoin: "round",
                                            circle { cx: "11", cy: "11", r: "8" }
                                            line { x1: "21", y1: "21", x2: "16.65", y2: "16.65" }
                                            line { x1: "11", y1: "8", x2: "11", y2: "14" }
                                            line { x1: "8", y1: "11", x2: "14", y2: "11" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    div { class: "py-20 text-center max-w-md mx-auto",
                        div { class: "w-20 h-20 mx-auto mb-6 rounded-3xl bg-rose-50 text-ieee-ras flex items-center justify-center shadow-inner",
                            svg {
                                class: "w-10 h-10",
                                fill: "none",
                                stroke: "currentColor",
                                view_box: "0 0 24 24",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "1.5",
                                    d: "M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z",
                                }
                            }
                        }
                        h3 { class: "text-2xl font-bold text-gray-900 mb-2", "Gallery Is Empty" }
                        p { class: "text-gray-500 text-sm leading-relaxed",
                            "New event photos, workshop memories, and competition builds will be posted here soon. Check back shortly!"
                        }
                    }
                }
            }

            // Lightbox Modal
            GalleryModal {
                item: selected_image.read().clone(),
                on_close: move |_| selected_image.set(None),
            }
        }
    }
}
