# IEEE RAS Pulchowk Student Branch Chapter

The official static website for the **IEEE Robotics & Automation Society (RAS) Student Branch Chapter** at IOE Pulchowk Campus, Lalitpur, Nepal.

Built with [Dioxus](https://dioxuslabs.com/) (Rust) compiled to WebAssembly via [Trunk](https://trunkrs.dev/) and styled with Tailwind CSS.

---

## Why Dioxus?

We wanted a fast, completely static site that doesn't need a Node.js server, an active database, or complex hosting to run. Compiling Rust down to WASM gives us:
- Instant client-side route transitions
- A single static bundle we can drop on GitHub Pages, Cloudflare Pages, or any static host
- Type-safe components and data models for our team roster, blog updates, and events

---

## Getting Started

### Prerequisites

1. **Rust toolchain**: Make sure Rust is installed via `rustup`:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```
2. **Trunk**: The WASM web application bundler for Rust:
   ```bash
   # On macOS via Homebrew:
   brew install trunk

   # Or via cargo:
   cargo install --locked trunk
   ```

### Local Development

Start the local dev server with hot-reloading:

```bash
trunk serve
```

Then open [http://127.0.0.1:8080](http://127.0.0.1:8080) in your browser.

### Production Build

To create an optimized release build:

```bash
trunk build --release
```

All compiled assets (HTML, `.wasm`, JS glue, and static images from `public/`) will be generated inside the `dist/` directory, ready to deploy.

---

## Project Structure

Here is a quick overview of how the project is organized so future webmasters can easily update content:

```text
├── Cargo.toml               # Rust dependencies (Dioxus 0.6, pulldown-cmark, serde)
├── index.html               # Entry HTML shell, Tailwind config & canvas scripts
├── public/                  # Static assets served at root
│   ├── committee/ras/       # Member portraits (2026 tenure)
│   ├── blogs/               # Blog covers and graphics
│   └── logo-ras-*.png/svg   # Official IEEE RAS logos (color & white)
└── src/
    ├── main.rs              # App entry point, Router enum & layout
    ├── types.rs             # Data structs (TeamMember, Blog, IeeeEvent)
    ├── data/                # Static content files
    │   ├── team.rs          # Executive committee & officer rosters
    │   ├── blogs.rs         # Articles & Markdown posts
    │   ├── events.rs        # Chapter events and workshop details
    │   └── gallery.rs       # Gallery image registry
    ├── views/               # Page routes (Home, Events, Team, Blogs, Gallery)
    └── components/          # Reusable UI widgets (Navbar, Footer, Modals, Cards)
```

---

## How to Update Content

- **Adding or editing team members**: Edit `src/data/team.rs` and drop square photo portraits into `public/committee/ras/`.
- **Posting a blog article**: Add a new `Blog` item to `src/data/blogs.rs`. The `content` field supports standard Markdown (headers, lists, links, code blocks) and is parsed in the reader modal.
- **Publishing upcoming events**: Add entries to `src/data/events.rs` with `is_upcoming: true`. When there are no active events, the site automatically displays a fallback announcement card.

---

## Contact & Chapter Info

- **Chapter**: IEEE Robotics & Automation Society Pulchowk STBC
- **Institution**: Pulchowk Campus, Institute of Engineering (IOE), Tribhuvan University
- **Email**: [ieeeras@pcampus.edu.np](mailto:ieeeras@pcampus.edu.np)
- **Location**: Lalitpur, Bagmati, Nepal
