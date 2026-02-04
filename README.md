# News Service (Phase A)

A minimal, security-focused, read-only news service written in Rust.

This project is designed with a **deny-by-default** philosophy and a strict scope:
static content delivery only, no user interaction, no dynamic behavior.

Phase A focuses exclusively on correctness, predictability, and attack-surface
reduction. Later phases will address deployment, filesystem hardening, and Tor
integration.

---

## Project Goals

- Provide a **read-only news feed**
- Serve **static HTML and CSS only**
- Enforce **strict HTTP parsing and limits**
- Avoid user input, state, or dynamic routing
- Remain suitable for privacy-conscious and onion-style deployments

This is **not** a blog engine, CMS, or interactive service.

---

## Phase A Scope (Completed)

Phase A establishes a hardened application core.

### Included
- Minimal HTTP/1.1 server
- Static allow-listed paths only:
  - `/`
  - `/style.css`
  - `/entries/*.html`
- Deterministic request handling
- Explicit request size and structure limits
- Integration tests for protocol boundaries

### Explicitly Excluded
- User input
- Forms
- Authentication
- JavaScript
- File uploads
- Directory listing
- Dynamic routing
- External network requests

---

## Architecture Overview

Client | |  HTTP/1.1 (GET / HEAD only) v Rust TCP Listener | v Strict Request Validation | +-- Reject malformed / oversized requests | +-- Route allow-listed static paths | v Static File Response
Routing occurs **only after validation**.

Malformed requests are rejected early and never reach routing logic.

---

## Static Content Model

Content is intentionally simple:

- `index.html` — main News page
- `style.css` — minimal styling
- `entries/YYYY-MM-DD.html` — individual news entries
- Entries are embedded via same-origin iframes

This structure:
- avoids pagination logic
- avoids metadata aggregation
- keeps each entry isolated and simple

---

## Security Design Principles

### 1. Deny by Default
Only known, allow-listed paths are served. Everything else returns `404`.

### 2. Strict Input Validation
- Maximum request size enforced
- Maximum request line length enforced
- Maximum header count enforced
- Unsupported methods rejected (`405`)

Malformed input is rejected deterministically.

### 3. Minimal Attack Surface
- No dynamic memory growth
- No user-controlled file paths
- No script execution
- No external assets
- No background tasks

### 4. Predictable HTTP Behavior
The server never guesses intent.
Invalid requests are rejected, not routed.

---

## Security Headers

All responses include:

- `X-Content-Type-Options: nosniff`
- `X-Frame-Options: SAMEORIGIN`
- `Cache-Control: no-store`
- `Connection: close`

These headers:
- prevent MIME confusion
- prevent clickjacking
- avoid client-side caching
- reduce protocol ambiguity

---

## Testing Strategy

Phase A includes **automated integration tests** that treat the service as an
external client would.

Tests verify:
- Valid requests return expected responses
- Unsupported methods are rejected
- Oversized request lines return `400`
- Excessive headers return `431`
- Unknown paths return `404`

Testing is deterministic and does not rely on mocks or async frameworks.

Run tests with:

```sh
cargo test --features test-support

---

## Threat Model (Phase A)

### Assumptions
- Service binds locally
- No TLS (handled externally)
- No trust in client input

### Non-Goals
- Resistance to authenticated attackers
- Content confidentiality
- Traffic analysis protection

## Phase Status
- Phase A: ✅ Complete
- Phase B: Filesystem layout, non-root execution, permissions
- Phase C: Tor onion service integration
- Phase D: Deployment hardening and operations


## License / Attribution

Content published by ***Th3 R0gu3 Kn!ghts***.

This project intentionally avoids embedding identifying metadata in served pages.

## Development Philosophy

This project follows a phased, security-first development model.

Each phase is completed, tested, documented, and frozen before moving forward.
Later phases may change deployment, execution context, or operating environment,
but do not retroactively change the behavior defined in earlier phases.

Simplicity and predictability are treated as security features.

## Directory Structure (Phase A)

```bash
bulletin-board
└── bulletin_board
    ├── Cargo.lock
    ├── Cargo.toml
    ├── README.md
    ├── src
    │   ├── config.rs
    │   ├── http.rs
    │   ├── lib.rs
    │   ├── main.rs
    │   └── server.rs
    ├── static
    │   ├── entries
    │   ├── index.html
    │   └── style.css
    ├── target
    │   ├── CACHEDIR.TAG
    │   ├── debug
    │   ├── release
    │   └── tmp
    └── tests
        └── http.rs
```

## Contributions

This project is intentionally conservative.

Changes that increase complexity, introduce dynamic behavior, or expand the
attack surface will be rejected unless explicitly planned for in a future phase.