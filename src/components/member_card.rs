use dioxus::prelude::*;
use crate::components::icons::IconLinkedin;
use crate::types::TeamMember;

#[component]
pub fn MemberCard(member: TeamMember) -> Element {
    let scale = member.image_scale.unwrap_or(1.0);
    let offset = member.image_offset.unwrap_or(0);
    let img_style = format!("transform: scale({scale}) translateY({offset}px);");

    rsx! {
        div { class: "group relative bg-white rounded-xl border border-gray-100 shadow-sm hover:shadow-md hover:border-ieee-ras/30 transition-all duration-300 flex flex-col overflow-hidden h-full",
            // Image / Placeholder
            div { class: "relative w-full h-64 sm:h-72 bg-gray-50 overflow-hidden flex items-center justify-center",
                if let Some(ref img_src) = member.image_url {
                    div { class: "absolute inset-0 transition-transform duration-500 group-hover:scale-105",
                        img {
                            src: "{img_src}",
                            alt: "{member.name}",
                            style: "{img_style}",
                            class: "w-full h-full object-cover",
                        }
                    }
                } else {
                    div { class: "text-gray-300 flex flex-col items-center justify-center",
                        svg {
                            width: "48",
                            height: "48",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "1.5",
                            path { d: "M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2" }
                            circle { cx: "12", cy: "7", r: "4" }
                        }
                    }
                }
            }

            // Text Info
            div { class: "p-5 flex flex-col items-center text-center flex-grow",
                h3 { class: "font-semibold text-gray-900 text-lg leading-tight transition-colors duration-200 group-hover:text-ieee-ras",
                    "{member.name}"
                }
                p { class: "text-ieee-ras/80 text-sm font-medium mt-1 mb-3",
                    "{member.role}"
                }

                // Social links revealed on hover
                div { class: "mt-auto flex items-center gap-3 opacity-0 group-hover:opacity-100 transition-opacity duration-300 transform translate-y-2 group-hover:translate-y-0",
                    if let Some(ref linkedin_url) = member.linkedin {
                        a {
                            href: "{linkedin_url}",
                            target: "_blank",
                            rel: "noreferrer",
                            class: "inline-flex items-center text-gray-400 hover:text-ieee-ras transition-colors",
                            aria_label: "{member.name} LinkedIn",
                            IconLinkedin { size: 18, color: "currentColor".to_string() }
                        }
                    }
                }
            }
        }
    }
}
