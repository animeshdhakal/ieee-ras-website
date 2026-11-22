use dioxus::prelude::*;
use crate::components::event_modal::EventModal;
use crate::components::home_about::HomeAbout;
use crate::components::home_cta::HomeCTA;
use crate::components::home_events::HomeEvents;
use crate::components::home_hero::HomeHero;
use crate::data::events::{get_all_events, get_upcoming_events};
use crate::types::IeeeEvent;

#[component]
pub fn Home() -> Element {
    let mut selected_event = use_signal(|| None::<IeeeEvent>);
    let upcoming = get_upcoming_events();
    let featured_events: Vec<IeeeEvent> = if !upcoming.is_empty() {
        upcoming
    } else {
        get_all_events().into_iter().take(3).collect()
    };

    rsx! {
        div { class: "flex flex-col min-h-screen",
            HomeHero {}
            HomeAbout {}
            HomeEvents {
                events: featured_events,
                on_select: move |ev| selected_event.set(Some(ev)),
            }
            HomeCTA {}

            // Detail Modal
            EventModal {
                event: selected_event.read().clone(),
                on_close: move |_| selected_event.set(None),
            }
        }
    }
}
