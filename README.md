# [MyDegree](https://mydegree.co.il)

A web application for Technion students to plan and track their degree progress — courses, grades, averages, and graduation requirements — all in one place.

## Features

- **Semester & Degree Averages** — Track per-semester and cumulative GPA in real time.
- **Course Search** — Search and add courses from an up-to-date UG-based course database.
- **Pre-requisite & Parallel Course Checks** — Automatically validates course dependencies on addition.
- **Course Type Tracking** — Monitor point totals across required, elective, and other course categories.
- **Cloud Sync** — Synchronize your plan across devices with Google sign-in or email authentication (Firebase Auth).
- **Histogram Viewer** — View grade distribution histograms per course, semester, and lecturer.
- **Mobile Responsive** — Full-featured mobile layout with card-based courses, floating labels, and bottom sheets.
- **Crash Reporting** — Automatic error reporting via Sentry (including WASM panic capture).
- **Feedback** — Built-in feedback form powered by EmailJS.

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Language | Rust → WebAssembly |
| UI Framework | [Leptos 0.7](https://leptos.dev/) (CSR) |
| Build Tool | [Trunk](https://trunkrs.dev/) |
| Styling | Bootstrap 5 RTL (self-hosted, PurgeCSS'd) + custom CSS |
| Backend | Firebase (Auth, Firestore) |
| Hosting | Firebase Hosting |
| Crash Reporting | [Sentry](https://sentry.io/) |
| Feedback | [EmailJS](https://www.emailjs.com/) |
| Core Logic | `degree-core` Rust crate (shared course/degree models) |

## Architecture

```
frontend/
├── index.html                  # Entry point (Sentry, EmailJS, Bootstrap, Firebase)
├── src/
│   ├── main.rs                 # WASM entry, panic hook, Leptos mount
│   ├── app.rs                  # Root component (desktop + mobile layouts)
│   ├── state.rs                # Reactive app state (signals, Firebase sync)
│   ├── firebase.rs             # Firebase JS interop (auth, Firestore)
│   ├── histogram.rs            # Histogram data parsing
│   └── components/
│       ├── header.rs           # Desktop header (nav, import/export, categories)
│       ├── semesters_tab_view.rs
│       ├── semester_table.rs   # Desktop course table with drag-drop
│       ├── degree_summary.rs   # Desktop degree summary
│       ├── search_dialog.rs    # Course search modal
│       ├── histogram_viewer.rs # Grade histogram display
│       ├── footer.rs           # Footer + About modal + Toast
│       └── mobile.rs           # Full mobile layout (header, tabs, cards, summary)
├── style.css                   # All styling (icons, mobile responsive, components)
└── firebase_bridge_src.mjs     # Firebase v10 modular SDK (bundled by esbuild)

degree-core/                    # Shared Rust library
├── src/lib.rs                  # Course, Semester, Degree models + logic
```

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) with `wasm32-unknown-unknown` target
- [Trunk](https://trunkrs.dev/) (`cargo install trunk`)
- [Node.js](https://nodejs.org/) (for esbuild/PurgeCSS build steps)

### Setup

```bash
# Install WASM target
rustup target add wasm32-unknown-unknown

# Install JS dependencies
cd frontend && npm install

# Copy Firebase config
cp frontend/firebase_config.example.js frontend/firebase_config.js
# Edit firebase_config.js with your Firebase project credentials
```

### Development

```bash
cd frontend
trunk serve
```

Opens at `http://localhost:8080` with hot-reload.

### Production Build

```bash
cd frontend
trunk build --release
```

Output goes to `frontend/dist/`.

### Deploy

```bash
firebase deploy --only hosting
```

## Acknowledgements

- Grade histograms powered by data from [michael-maltsev/technion-histograms](https://github.com/michael-maltsev/technion-histograms)
- Course information sourced via [michael-maltsev/technion-sap-info-fetcher](https://github.com/michael-maltsev/technion-sap-info-fetcher)

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## License

[GNU GPL 3.0](LICENSE.MD)
