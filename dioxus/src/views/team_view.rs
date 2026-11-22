use dioxus::prelude::*;
use crate::components::member_card::MemberCard;
use crate::data::team::get_ras_team_data;

#[component]
pub fn Team() -> Element {
    let team = get_ras_team_data();

    rsx! {
        div { class: "pt-24 pb-20 min-h-screen bg-white",
            div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                // Header
                div { class: "text-center mb-16",
                    h1 { class: "text-4xl md:text-5xl font-bold text-gray-900 mb-6",
                        "Our Team"
                    }
                    p { class: "text-xl text-gray-600 max-w-3xl mx-auto leading-relaxed",
                        "The dedicated individuals working behind the scenes to drive robotics innovation and foster a community of excellence."
                    }
                }

                div {
                    // Executive Officers Section
                    section { class: "mb-24",
                        div { class: "flex items-center mb-10",
                            h2 { class: "text-2xl font-bold text-gray-900 pr-4 bg-white z-10",
                                "Executive Officers"
                            }
                            div { class: "h-px bg-gray-200 flex-grow" }
                        }

                        // Chair & Vice Chair Highlight
                        div { class: "flex justify-center mb-10",
                            div { class: "grid grid-cols-1 md:grid-cols-2 gap-8 w-full md:w-2/3 lg:w-1/2",
                                for officer in team.officers.iter().take(2) {
                                    div { key: "{officer.id}", class: "h-full",
                                        MemberCard { member: officer.clone() }
                                    }
                                }
                            }
                        }

                        // Rest of Officers
                        div { class: "flex flex-wrap justify-center gap-6",
                            for officer in team.officers.iter().skip(2) {
                                div {
                                    key: "{officer.id}",
                                    class: "w-full sm:w-[calc(50%-12px)] lg:w-[calc(25%-18px)]",
                                    MemberCard { member: officer.clone() }
                                }
                            }
                        }
                    }

                    // Senior Executives Section
                    section { class: "mb-24",
                        div { class: "flex items-center mb-10",
                            h2 { class: "text-2xl font-bold text-gray-900 pr-4 bg-white z-10",
                                "Senior Executives"
                            }
                            div { class: "h-px bg-gray-200 flex-grow" }
                        }
                        div { class: "flex flex-wrap justify-center gap-6",
                            for member in team.senior_execs.iter() {
                                div {
                                    key: "{member.id}",
                                    class: "w-full sm:w-[calc(50%-12px)] lg:w-[calc(25%-18px)]",
                                    MemberCard { member: member.clone() }
                                }
                            }
                        }
                    }

                    // Committees
                    for committee in team.committees.iter() {
                        section { key: "{committee.title}", class: "mb-20 last:mb-0",
                            div { class: "flex items-center mb-8",
                                h2 { class: "text-2xl font-bold text-gray-900 pr-4 bg-white z-10",
                                    "{committee.title}"
                                }
                                div { class: "h-px bg-gray-200 flex-grow" }
                            }
                            div { class: "flex flex-wrap justify-center gap-6",
                                for member in committee.members.iter() {
                                    div {
                                        key: "{member.id}",
                                        class: "w-full sm:w-[calc(50%-12px)] lg:w-[calc(25%-18px)]",
                                        MemberCard { member: member.clone() }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
