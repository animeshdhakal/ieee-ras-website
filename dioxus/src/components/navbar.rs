use dioxus::prelude::*;
use crate::components::icons::{IconMenu, IconX};
use crate::Route;

#[component]
pub fn Navbar() -> Element {
    let mut is_open = use_signal(|| false);
    let mut scrolled = use_signal(|| false);
    let route = use_route::<Route>();
    let is_home = matches!(route, Route::Home {});

    use_effect(move || {
        let mut eval = document::eval(r#"
            function updateScroll() {
                const isScrolled = window.scrollY > 20;
                dioxus.send(isScrolled);
                const nav = document.getElementById('main-navbar');
                if (nav && nav.getAttribute('data-home') === 'true') {
                    if (isScrolled) {
                        nav.classList.add('nav-scrolled');
                    } else {
                        nav.classList.remove('nav-scrolled');
                    }
                }
            }
            window.addEventListener('scroll', updateScroll, { passive: true });
            updateScroll();
        "#);

        spawn(async move {
            while let Ok(val) = eval.recv::<bool>().await {
                scrolled.set(val);
            }
        });
    });

    let is_solid = !is_home || *scrolled.read() || *is_open.read();

    let nav_class = if !is_solid {
        "fixed w-full z-50 bg-transparent py-5 transition-all duration-300"
    } else {
        "fixed w-full z-50 bg-white/95 backdrop-blur-md shadow-sm border-b border-gray-100 py-3 transition-all duration-300 nav-scrolled"
    };

    let text_class = if !is_solid {
        "text-white hover:text-rose-200"
    } else {
        "text-gray-800 hover:text-ieee-ras"
    };

    let btn_class = if !is_solid {
        "bg-white text-ieee-ras hover:bg-gray-100 shadow-sm"
    } else {
        "bg-ieee-ras text-white hover:bg-ieee-ras-dark shadow-md"
    };

    rsx! {
        nav {
            id: "main-navbar",
            "data-home": if is_home { "true" } else { "false" },
            class: "{nav_class}",
            div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                div { class: "flex justify-between items-center",
                    // Brand Logo
                    Link {
                        to: Route::Home {},
                        class: "flex items-center space-x-2 group",
                        div { class: "relative h-11 w-auto py-1",
                            img {
                                src: "/logo-ras-white.png?v=2026_02",
                                alt: "IEEE RAS Pulchowk Logo",
                                class: if !is_solid {
                                    "nav-logo-white block h-9 md:h-10 w-auto object-contain transition-opacity duration-300"
                                } else {
                                    "nav-logo-white hidden h-9 md:h-10 w-auto object-contain transition-opacity duration-300"
                                },
                            }
                            img {
                                src: "/logo-ras-color.png?v=2026_02",
                                alt: "IEEE RAS Pulchowk Logo",
                                class: if !is_solid {
                                    "nav-logo-color hidden h-9 md:h-10 w-auto object-contain transition-opacity duration-300"
                                } else {
                                    "nav-logo-color block h-9 md:h-10 w-auto object-contain transition-opacity duration-300"
                                },
                            }
                        }
                    }

                    // Desktop Menu
                    div { class: "hidden md:flex space-x-8 items-center",
                        Link {
                            to: Route::Home {},
                            class: "nav-text-color text-sm font-medium tracking-wide transition-colors duration-200 opacity-90 {text_class}",
                            "Home"
                        }
                        Link {
                            to: Route::Events {},
                            class: "nav-text-color text-sm font-medium tracking-wide transition-colors duration-200 opacity-90 {text_class}",
                            "Events"
                        }
                        Link {
                            to: Route::Team {},
                            class: "nav-text-color text-sm font-medium tracking-wide transition-colors duration-200 opacity-90 {text_class}",
                            "Team"
                        }
                        Link {
                            to: Route::Blogs {},
                            class: "nav-text-color text-sm font-medium tracking-wide transition-colors duration-200 opacity-90 {text_class}",
                            "Blogs"
                        }
                        Link {
                            to: Route::Gallery {},
                            class: "nav-text-color text-sm font-medium tracking-wide transition-colors duration-200 opacity-90 {text_class}",
                            "Gallery"
                        }
                        a {
                            href: "https://www.ieee-ras.org/membership/join",
                            target: "_blank",
                            rel: "noreferrer",
                            class: "nav-cta-btn px-5 py-2 rounded-full text-sm font-semibold transition-all duration-300 transform hover:scale-105 {btn_class}",
                            "Join IEEE RAS"
                        }
                    }

                    // Mobile Menu Button
                    div { class: "md:hidden flex items-center",
                        button {
                            onclick: move |_| is_open.toggle(),
                            aria_label: "Toggle Menu",
                            class: "nav-mobile-btn focus:outline-none {text_class}",
                            if *is_open.read() {
                                IconX { size: 24, color: "currentColor".to_string() }
                            } else {
                                IconMenu { size: 24, color: "currentColor".to_string() }
                            }
                        }
                    }
                }
            }

            // Mobile Menu Dropdown
            if *is_open.read() {
                div { class: "md:hidden absolute top-full left-0 w-full bg-white shadow-lg border-b border-gray-100 py-4 px-4 flex flex-col space-y-4",
                    Link {
                        to: Route::Home {},
                        onclick: move |_| is_open.set(false),
                        class: "block text-base font-medium text-gray-800 hover:text-ieee-ras",
                        "Home"
                    }
                    Link {
                        to: Route::Events {},
                        onclick: move |_| is_open.set(false),
                        class: "block text-base font-medium text-gray-800 hover:text-ieee-ras",
                        "Events"
                    }
                    Link {
                        to: Route::Team {},
                        onclick: move |_| is_open.set(false),
                        class: "block text-base font-medium text-gray-800 hover:text-ieee-ras",
                        "Team"
                    }
                    Link {
                        to: Route::Blogs {},
                        onclick: move |_| is_open.set(false),
                        class: "block text-base font-medium text-gray-800 hover:text-ieee-ras",
                        "Blogs"
                    }
                    Link {
                        to: Route::Gallery {},
                        onclick: move |_| is_open.set(false),
                        class: "block text-base font-medium text-gray-800 hover:text-ieee-ras",
                        "Gallery"
                    }
                    a {
                        href: "https://www.ieee-ras.org/membership/join",
                        target: "_blank",
                        rel: "noreferrer",
                        class: "block text-center w-full py-3 bg-ieee-ras text-white rounded-lg font-semibold",
                        "Join IEEE RAS"
                    }
                }
            }
        }
    }
}
