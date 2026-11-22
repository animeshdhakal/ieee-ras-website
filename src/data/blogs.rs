use crate::types::Blog;

pub fn get_all_blogs() -> Vec<Blog> {
    vec![
        Blog {
            id: "ras-website-launch".to_string(),
            title: "Official Launch of IEEE Robotics & Automation Society Pulchowk Website".to_string(),
            excerpt: "We are thrilled to unveil our dedicated static web portal built entirely with Dioxus and Rust for blazing performance!".to_string(),
            content: r#"# Launch of IEEE Robotics & Automation Society Pulchowk STBC Website

We are thrilled to celebrate a landmark milestone for our robotics community: the official launch of the **IEEE Robotics & Automation Society (RAS) Pulchowk Student Branch Chapter Website**!

### Engineering with Dioxus & WebAssembly

To mirror our commitment to systems engineering and high performance, this portal is crafted using **Dioxus (Rust WebAssembly)**:
- **Zero Heavy Runtime Overhead**: Lightning-fast initial load times and instant navigation across client-side routes.
- **Static Resilient Architecture**: No backend database failure points; fully pre-compiled and served directly to the browser.
- **Modern Robotics Aesthetics**: Tailored with the official IEEE RAS color palette—Burgundy Red (#862633), Royal Purple (#5F2167), and vibrant rose accents.

### What You Can Explore
- **Upcoming & Past Events**: Register for robotics bootcamps, workshops, hackathons, and guest lectures.
- **Roster & Executive Committee**: Meet the officers, mentors, and technical committee members behind the chapter.
- **Interactive Photo Gallery**: Explore milestones from workshops, robotics builds, and competitions.
- **Technical Articles & Research**: Stay updated with member publications, project breakdowns, and technical tutorials.

Stay tuned as we continue developing cutting-edge robotics projects right here at Pulchowk Campus!"#.to_string(),
            date: "Feb 14, 2026".to_string(),
            author: "Animesh Dhakal".to_string(),
            author_role: "Webmaster & Systems Lead".to_string(),
            category: "Technology".to_string(),
            read_time: "5 min read".to_string(),
            thumbnail: "/blogs/website-launch-logo.jpg".to_string(),
        },
    ]
}
