# Technion Course Tools (Rust)

Rust binaries for fetching and managing Technion course data.

## Binaries

### `merge-courses`

Fetches course data from [michael-maltsev/technion-sap-info-fetcher](https://github.com/michael-maltsev/technion-sap-info-fetcher) (gh-pages branch), parses **all** available semester JSON files, converts them to the project's `CourseDB` format, and optionally merges with an existing database.

**Features:**
- Dynamically discovers all `courses_YYYY_SSS.json` files (skips `.unfiltered` and `.min` variants)
- Newer semesters override older entries for the same course number
- Converts old 5-6 digit course numbers to 8-digit format when merging with an old DB
- Generates `number_aliases` map for backward-compatible search
- Computes `content_hash` for cache invalidation

**Usage:**

```bash
# Build
cargo build --release --bin merge-courses

# Merge gh-pages data with an existing DB
merge-courses --old-db python_scripts/db/courses.json -o python_scripts/db/courses.json

# Fresh build from gh-pages only (no old DB)
merge-courses -o python_scripts/db/courses.json

# Keep the cloned repo for inspection
merge-courses -o /tmp/courses.json --keep-clone
```

**Output format (`courses.json`):**

```json
{
  "content_hash": "abc123...",
  "courses": [
    {
      "number": "02340123",
      "name": "מערכות הפעלה",
      "full_name": "02340123: מערכות הפעלה",
      "points": 4.5,
      "prerequisites": [["02340114"], ["02340118", "02340119"]],
      "linked": [],
      "identical": [],
      "overlapping": [],
      "inclusive": [],
      "including": [],
      "followed_by": ["02340125: ..."]
    }
  ],
  "number_aliases": {
    "234123": "02340123"
  }
}
```

### `course-scraper`

Direct SAP endpoint scraper with Tor proxy support. Fetches course data from Technion's SAP system.

> **Note:** The SAP endpoint is protected by a WAF that aggressively rate-limits. Prefer `merge-courses` which uses pre-fetched data.

**Usage:**

```bash
# Basic scraping (will be throttled quickly)
course-scraper --semester 202501 -o output.json

# With Tor proxy for IP rotation
course-scraper --semester 202501 -o output.json \
  --tor-proxy socks5://127.0.0.1:9050 \
  --tor-control-port 9051 \
  --concurrency 2
```

## Data Directory

- `../db/courses.json` — The canonical course database used by the frontend
