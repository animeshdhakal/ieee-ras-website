use dioxus::prelude::*;

mod components;
mod data;
mod types;
mod views;

use components::footer::Footer;
use components::navbar::Navbar;
use views::blogs_view::Blogs;
use views::events_view::Events;
use views::gallery_view::Gallery;
use views::home::Home;
use views::team_view::Team;

#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[layout(RootLayout)]
    #[route("/")]
    Home {},
    #[route("/events")]
    Events {},
    #[route("/team")]
    Team {},
    #[route("/blogs")]
    Blogs {},
    #[route("/gallery")]
    Gallery {},
}

#[component]
fn RootLayout() -> Element {
    rsx! {
        div { class: "flex flex-col min-h-screen bg-white font-sans antialiased text-gray-900",
            Navbar {}
            main { class: "flex-1",
                Outlet::<Route> {}
            }
            Footer {}
        }
    }
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}
