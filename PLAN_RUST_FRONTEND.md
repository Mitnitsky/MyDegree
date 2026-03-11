# Plan: Migrate MyDegree Frontend to Rust + WebAssembly (Leptos)

## Framework Choice: Leptos

**Leptos** is chosen over alternatives (Yew, Dioxus) because:
- Fine-grained reactivity — maps cleanly to the existing Vuex store pattern
- No virtual DOM diffing → better runtime performance
- Server-Side Rendering support available if needed later
- Active ecosystem; `leptos-use` covers many Vue composable equivalents

---

## Phase 1 — Project Setup & Toolchain

1. Install the Rust WASM toolchain: `rustup target add wasm32-unknown-unknown`
2. Install **`trunk`** as the build/dev server (equivalent to Vite — bundles, serves, hot-reloads WASM)
3. Install `wasm-pack` for any standalone logic crates
4. Set up a **Cargo workspace** monorepo:
   - `/degree-core` — pure Rust library (business logic, no WASM bindings needed)
   - `/frontend` — Leptos app crate (compiled to WASM via `trunk`)
   - Keep `/python_scripts` untouched
5. Configure `trunk` with `Trunk.toml` pointing to `frontend/index.html`
6. Firebase has no Rust SDK — use **Firebase REST API** via `gloo-net` for Firestore; use Firebase Auth REST API for authentication

---

## Phase 2 — Data Models (`degree-core`)

Define all shared data structures in `degree-core` as a pure Rust library:

1. **`Course`** — `exists_in_db`, `name`, `number`, `points`, `grade`, `course_type`, `binary`; implement `Default`
2. **`Semester`** — `name`, `average`, `points`, `courses: Vec<Course>`
3. **`CourseType`** — `name`, `points_required`, `total_points`, `points_done`, `points_left`, `average`
4. **`UserState`** — full degree state: semesters, course_types, degree stats, `english_exemption`, `active_semester`, `token`
5. **`CourseDB`** — represents entries from `courses.json` (static course database)
6. Derive `serde::{Serialize, Deserialize}` on all structs for Firebase/localStorage JSON interop

---

## Phase 3 — Business Logic (`degree-core`)

Port all JavaScript store/utility logic to Rust modules:

| Module | Ported logic |
|---|---|
| `course.rs` | `create_new_course()`, `create_course_from_db_entry()`, `course_is_empty()` |
| `semester.rs` | `calculate_average()`, `calculate_points()`, `add_course()`, `remove_course()`, `course_exist_in_semesters()`, `sort_courses_by_field()` |
| `degree.rs` | Degree-level recalculation: total average, points done/left/to-choose, category breakdowns |
| `sorting.rs` | Hebrew + Latin character sort (`CharCompare` equivalent using explicit `Vec<char>` ordering for `א–ת`) |
| `utils.rs` | `math_round10()`, JSON export/download helper |
| `converter.rs` | `find_course()` — search in `CourseDB` |

Unit tests for all calculation functions inline in each module.

---

## Phase 4 — State Management (Leptos Signals)

Replace Vuex with Leptos reactive signals:

| Vuex concept | Leptos equivalent |
|---|---|
| `state` | `create_rw_signal` |
| `computed` | `create_memo` |
| `mutation` | signal setter function |
| `action` (async) | `create_action` / `create_resource` |

1. Create a top-level `AppState` context using `provide_context` / `use_context`
2. Each semester is a `RwSignal<Semester>`; the semesters list is `RwSignal<Vec<Semester>>`
3. Degree-level stats are `Memo<f64>` derived from the semesters signal — recomputed automatically
4. Persist to `localStorage` via `gloo-storage` on every state change using `create_effect`
5. Firebase sync: use `create_resource` + `gloo-net` HTTP calls to Firestore REST API

---

## Phase 5 — UI Components (Leptos)

Migrate each Vue component to a Leptos component function:

| Vue component | Leptos component |
|---|---|
| `App.vue` | `fn App()` — root, provides `AppState` context |
| `Header.vue` / `HeaderAuthentication.vue` | `fn Header()` — nav bar, login/logout |
| `SemestersTabView.vue` | `fn SemestersTabView()` — tab navigation |
| `SemesterTable.vue` | `fn SemesterTable(semester: RwSignal<Semester>)` |
| `SemesterTableRow.vue` | `fn SemesterTableRow(course: RwSignal<Course>)` — editable row |
| `SemesterTableHeader.vue` | `fn SemesterTableHeader()` — sortable column headers |
| `SemesterSummary.vue` | `fn SemesterSummary(semester: RwSignal<Semester>)` |
| `DegreeSummary.vue` | `fn DegreeSummary()` — degree-level stats panel |
| `SearchCourseDialog.vue` | `fn SearchCourseDialog()` — autocomplete modal |
| `MyInfoCard.vue` | `fn MyInfoCard()` — user profile card |
| `Footer.vue` | `fn Footer()` |

**Styling:** Keep existing Bootstrap 5 CSS (CDN link in `index.html`) — Leptos renders standard HTML so all Bootstrap classes work identically. Keep RTL layout with `<html dir="rtl">` and the Alef font via `@import` in CSS.

**Icons:** Use Font Awesome via CDN or the `icondata` crate.

**Modals:** Use `leptos-use` `use_toggle` + a conditional `<Show>` element instead of `vue-js-modal`.

**Autocomplete:** Implement a reactive `<input>` with a `create_memo`-filtered list from `CourseDB`.

---

## Phase 6 — Authentication & Firebase

Since Firebase has no official Rust SDK:

1. **Auth** — Call Firebase Auth REST API endpoints (`signInWithEmailAndPassword`, Google sign-in redirect) using `gloo-net::http::Request`; store the ID token in a signal and `localStorage`
2. **Firestore** — Use Firestore REST API (`https://firestore.googleapis.com/v1/...`) for reading/writing user degree data; serialize/deserialize with `serde_json`
3. **Alternative** — Keep a minimal JS shim (`firebase_bridge.js`) called via `wasm-bindgen` to use the official Firebase JS SDK for OAuth/Google sign-in (lower complexity for that flow)

---

## Phase 7 — Build & Deployment

1. **Development:** `trunk serve` inside `frontend/` — hot-reload, can proxy to Firebase emulator
2. **Production:** `trunk build --release` → outputs to `frontend/dist/` → deploy to Firebase Hosting (update `firebase.json` `public` field to `frontend/dist`)
3. **CI:** Replace `vue-cli-service build` in the build script with `trunk build --release`

---

## Key Challenges & Mitigations

| Challenge | Mitigation |
|---|---|
| No official Firebase Rust SDK | Use Firebase REST API via `gloo-net`; keep small JS shim for OAuth if needed |
| RTL + Hebrew text | Set `dir="rtl"` on `<html>`; Bootstrap 5 handles RTL — no framework changes needed |
| Hebrew alphabet sorting | Implement `CharCompare` in `sorting.rs` with explicit `Vec<char>` ordering |
| Bootstrap-Vue directives (tooltips, collapse) | Use Bootstrap 5 JS via CDN + `data-bs-*` attributes in Leptos `view!` macro |
| `courses.json` (1000s of entries) | Load via `include_str!` at compile time, deserialize once at startup into a signal |
| `vuex-map-fields` two-way binding | Use Leptos signal setters + `create_effect` writing back to `gloo-storage` |

---

## Final Directory Structure

```
MyDegree/
├── Cargo.toml                  # Workspace root
├── degree-core/                # Shared Rust business logic (no WASM)
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── course.rs
│       ├── semester.rs
│       ├── degree.rs
│       ├── sorting.rs
│       ├── converter.rs
│       └── utils.rs
├── frontend/                   # Leptos WASM app
│   ├── Cargo.toml
│   ├── Trunk.toml
│   ├── index.html              # Bootstrap 5 + Font Awesome CDN links
│   ├── style.css               # Alef font + existing custom styles
│   └── src/
│       ├── main.rs
│       ├── app.rs
│       ├── state.rs            # AppState signals & context
│       ├── firebase.rs         # REST API calls
│       └── components/
│           ├── header.rs
│           ├── semesters_tab_view.rs
│           ├── semester_table.rs
│           ├── degree_summary.rs
│           ├── search_dialog.rs
│           └── footer.rs
├── python_scripts/             # Unchanged
├── public/                     # Existing static assets (kept as-is)
└── src/                        # Existing Vue app (to be replaced)
```
