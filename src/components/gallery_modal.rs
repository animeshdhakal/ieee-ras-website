use dioxus::prelude::*;
use crate::components::icons::IconX;
use crate::types::GalleryItem;

#[component]
pub fn GalleryModal(item: Option<GalleryItem>, on_close: EventHandler<()>) -> Element {
    let Some(ref photo) = item else {
        return rsx! {};
    };

    rsx! {
        div {
            class: "fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/85 backdrop-blur-md animate-in fade-in duration-200",
            onclick: move |_| on_close.call(()),
            div {
                class: "relative max-w-4xl w-full max-h-[92vh] flex flex-col items-center animate-in zoom-in-95 duration-200",
                onclick: move |e| e.stop_propagation(),

                // Close Button
                button {
                    onclick: move |_| on_close.call(()),
                    aria_label: "Close Lightbox",
                    class: "absolute -top-12 right-0 p-2 rounded-full bg-white/20 hover:bg-white/40 text-white transition-all",
                    IconX { size: 24, color: "white".to_string() }
                }

                // Image Container
                div { class: "rounded-2xl overflow-hidden shadow-2xl bg-black border border-white/10 w-full flex items-center justify-center max-h-[75vh]",
                    img {
                        src: "{photo.image_url}",
                        alt: "{photo.title}",
                        class: "max-h-[75vh] w-auto max-w-full object-contain",
                    }
                }

                // Caption
                div { class: "mt-4 text-center text-white space-y-1 px-4",
                    h3 { class: "text-lg font-bold text-gray-100", "{photo.title}" }
                    p { class: "text-xs font-semibold text-rose-400 uppercase tracking-wider",
                        "{photo.category} · {photo.date}"
                    }
                }
            }
        }
    }
}
