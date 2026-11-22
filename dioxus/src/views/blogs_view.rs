use dioxus::prelude::*;
use crate::components::blog_card::BlogCard;
use crate::components::blog_modal::BlogModal;
use crate::components::icons::IconSearch;
use crate::data::blogs::get_all_blogs;
use crate::types::Blog;

#[component]
pub fn Blogs() -> Element {
    let mut search_query = use_signal(|| String::new());
    let mut selected_category = use_signal(|| "All".to_string());
    let mut selected_blog = use_signal(|| None::<Blog>);

    let blogs = get_all_blogs();
    let categories = vec!["All", "Robotics", "Technology", "Events"];

    let query = search_query.read().to_lowercase();
    let cat = selected_category.read().clone();

    let filtered_posts: Vec<Blog> = blogs
        .into_iter()
        .filter(|post| {
            let matches_category = cat == "All" || post.category == cat;
            let matches_search = query.is_empty()
                || post.title.to_lowercase().contains(&query)
                || post.excerpt.to_lowercase().contains(&query);
            matches_category && matches_search
        })
        .collect();

    let featured_post = if !filtered_posts.is_empty() {
        Some(filtered_posts[0].clone())
    } else {
        None
    };

    let other_posts: Vec<Blog> = if filtered_posts.len() > 1 {
        filtered_posts.iter().skip(1).cloned().collect()
    } else {
        Vec::new()
    };

    rsx! {
        div { class: "pt-24 pb-20 min-h-screen bg-gray-50",
            div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                // Header
                div { class: "text-center mb-12",
                    h1 { class: "text-4xl font-bold text-gray-900 mb-4",
                        "Insights & Updates"
                    }
                    p { class: "text-gray-600 max-w-2xl mx-auto",
                        "Articles, research highlights, and technical tutorials from the IEEE RAS Pulchowk community."
                    }
                }

                // Search & Filter Bar
                div { class: "flex flex-col md:flex-row justify-between items-center mb-12 gap-6 bg-white p-4 rounded-xl shadow-sm border border-gray-100",
                    div { class: "relative w-full md:w-96",
                        IconSearch {
                            size: 18,
                            color: "#9ca3af".to_string(),
                            class: "absolute left-3 top-1/2 transform -translate-y-1/2 pointer-events-none",
                        }
                        input {
                            r#type: "text",
                            placeholder: "Search articles...",
                            value: "{search_query.read()}",
                            oninput: move |e| search_query.set(e.value()),
                            class: "w-full pl-10 pr-4 py-2 bg-gray-50 border border-gray-200 rounded-lg focus:outline-none focus:ring-2 focus:ring-ieee-ras/20 focus:border-ieee-ras transition-all text-sm",
                        }
                    }

                    div { class: "flex items-center gap-2 overflow-x-auto w-full md:w-auto pb-2 md:pb-0 no-scrollbar",
                        for category in categories.iter() {
                            {
                                let is_active = *selected_category.read() == *category;
                                let btn_class = if is_active {
                                    "bg-gray-900 text-white border-gray-900"
                                } else {
                                    "bg-white text-gray-600 border-gray-200 hover:border-gray-300 hover:bg-gray-50"
                                };
                                let c_str = category.to_string();
                                rsx! {
                                    button {
                                        key: "{category}",
                                        onclick: move |_| selected_category.set(c_str.clone()),
                                        class: "px-4 py-1.5 rounded-full text-xs font-medium border transition-all whitespace-nowrap {btn_class}",
                                        "{category}"
                                    }
                                }
                            }
                        }
                    }
                }

                // Featured Post (shown if on "All" category with no search query)
                if let Some(featured) = featured_post {
                    if *selected_category.read() == "All" && search_query.read().is_empty() {
                        div { class: "mb-12",
                            h2 { class: "text-lg font-semibold text-gray-900 mb-4 flex items-center",
                                span { class: "w-2 h-8 bg-ieee-ras rounded-full mr-3" }
                                "Featured Story"
                            }
                            BlogCard {
                                blog: featured,
                                featured: true,
                                on_select: move |b| selected_blog.set(Some(b)),
                            }
                        }
                    }
                }

                // Posts Grid
                div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8",
                    {
                        let display_posts = if *selected_category.read() == "All" && search_query.read().is_empty() {
                            other_posts
                        } else {
                            filtered_posts.clone()
                        };

                        rsx! {
                            for post in display_posts.iter() {
                                div { key: "{post.id}",
                                    BlogCard {
                                        blog: post.clone(),
                                        on_select: move |b| selected_blog.set(Some(b)),
                                    }
                                }
                            }
                        }
                    }
                }

                if filtered_posts.is_empty() {
                    div { class: "text-center py-20 bg-white rounded-xl border border-dashed border-gray-200",
                        p { class: "text-gray-500 text-lg",
                            "No articles found matching your criteria."
                        }
                        button {
                            onclick: move |_| {
                                search_query.set(String::new());
                                selected_category.set("All".to_string());
                            },
                            class: "mt-4 text-ieee-ras font-medium hover:underline",
                            "Clear filters"
                        }
                    }
                }
            }

            // Blog Reader Modal
            BlogModal {
                blog: selected_blog.read().clone(),
                on_close: move |_| selected_blog.set(None),
            }
        }
    }
}
