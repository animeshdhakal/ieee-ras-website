use dioxus::prelude::*;
use crate::components::icons::{IconFacebook, IconInstagram, IconLinkedin, IconMail, IconMapPin};
use crate::Route;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer { class: "bg-gray-900 text-gray-300 py-12 border-t border-gray-800",
            div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                div { class: "grid grid-cols-1 md:grid-cols-3 gap-12",
                    // Brand Column
                    div { class: "space-y-4",
                        div { class: "mb-4",
                            img {
                                src: "/logo-ras-white.png?v=2026_02",
                                alt: "IEEE RAS Pulchowk Logo",
                                class: "h-11 w-auto object-contain opacity-95",
                            }
                        }
                        p { class: "text-sm leading-relaxed opacity-80 max-w-xs",
                            "Advancing Technology for Humanity. The student branch chapter of IEEE Robotics & Automation Society at Pulchowk Campus, dedicated to technical excellence and community growth."
                        }
                    }

                    // Quick Links
                    div { class: "space-y-4",
                        h4 { class: "text-white text-sm font-semibold uppercase tracking-wider",
                            "Quick Links"
                        }
                        ul { class: "space-y-2 text-sm",
                            li {
                                Link { to: Route::Home {}, class: "hover:text-white transition-colors", "Home" }
                            }
                            li {
                                Link { to: Route::Events {}, class: "hover:text-white transition-colors", "Events" }
                            }
                            li {
                                Link { to: Route::Team {}, class: "hover:text-white transition-colors", "Our Team" }
                            }
                            li {
                                Link { to: Route::Blogs {}, class: "hover:text-white transition-colors", "Blogs" }
                            }
                            li {
                                Link { to: Route::Gallery {}, class: "hover:text-white transition-colors", "Gallery" }
                            }
                            li {
                                a {
                                    href: "https://www.ieee.org",
                                    target: "_blank",
                                    rel: "noreferrer",
                                    class: "hover:text-white transition-colors",
                                    "IEEE.org"
                                }
                            }
                            li {
                                a {
                                    href: "https://www.ieee-ras.org",
                                    target: "_blank",
                                    rel: "noreferrer",
                                    class: "hover:text-white transition-colors",
                                    "IEEE-RAS.org"
                                }
                            }
                        }
                    }

                    // Contact
                    div { class: "space-y-4",
                        h4 { class: "text-white text-sm font-semibold uppercase tracking-wider",
                            "Contact Us"
                        }
                        div { class: "space-y-3 text-sm",
                            div { class: "flex items-start space-x-3",
                                IconMapPin { size: 18, color: "#862633".to_string(), class: "mt-0.5" }
                                span { "Pulchowk Campus, Lalitpur, Nepal" }
                            }
                            div { class: "flex items-center space-x-3",
                                IconMail { size: 18, color: "#862633".to_string() }
                                a {
                                    href: "mailto:ieeeras@pcampus.edu.np",
                                    class: "hover:text-white transition-colors",
                                    "ieeeras@pcampus.edu.np"
                                }
                            }
                        }

                        // Social Icons
                        div { class: "flex space-x-4 pt-2",
                            a {
                                href: "https://www.facebook.com/IEEEPulchowk",
                                target: "_blank",
                                rel: "noreferrer",
                                aria_label: "Facebook",
                                class: "text-gray-400 hover:text-white transition-colors",
                                IconFacebook { size: 20, color: "currentColor".to_string() }
                            }
                            a {
                                href: "https://www.linkedin.com/company/ieee-pulchowk",
                                target: "_blank",
                                rel: "noreferrer",
                                aria_label: "LinkedIn",
                                class: "text-gray-400 hover:text-white transition-colors",
                                IconLinkedin { size: 20, color: "currentColor".to_string() }
                            }
                            a {
                                href: "https://www.instagram.com/ieee.pul",
                                target: "_blank",
                                rel: "noreferrer",
                                aria_label: "Instagram",
                                class: "text-gray-400 hover:text-white transition-colors",
                                IconInstagram { size: 20, color: "currentColor".to_string() }
                            }
                        }
                    }
                }

                div { class: "mt-12 pt-8 border-t border-gray-800 flex flex-col md:flex-row justify-between items-center text-xs text-gray-500",
                    p {
                        "© 2026 IEEE Robotics & Automation Society Pulchowk STBC. All rights reserved."
                    }
                    div { class: "flex space-x-6 mt-4 md:mt-0",
                        a { href: "#", class: "hover:text-gray-300", "Privacy Policy" }
                        a { href: "#", class: "hover:text-gray-300", "Terms of Service" }
                    }
                }
            }
        }
    }
}
