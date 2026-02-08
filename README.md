# News Service

A minimal, security-focused, **read-only news service** written in Rust.

This project is designed with a **deny-by-default** philosophy and a deliberately
restricted scope: **static content delivery only**, with no user interaction and
no dynamic behavior.

The service is intended to be suitable for **privacy-conscious deployments**,
including Tor onion services.

---

## Project Goals

- Provide a **read-only news feed**
- Serve **static HTML and CSS only**
- Enforce **strict HTTP parsing and limits**
- Avoid user input, state, or dynamic routing
- Minimize attack surface by design
- Remain predictable and auditable

This is **not**:
- a CMS
- a blog engine
- an API
- an interactive service

---

## Phased Development Model

This project follows a **phased, security-first development model**.

Each phase is:
- implemented
- tested
- documented
- frozen

Later phases may change *deployment or execution context*, but **do not alter
behavior defined in earlier phases**.

Simplicity and predictability are treated as security features.

---

## Phase A — Application Core (Completed)

Phase A establishes a hardened, deterministic HTTP service.

### Included
- Minimal HTTP/1.1 server (GET / HEAD only)
- Strict request parsing and validation
- Explicit size and structure limits
- Deterministic error handling
- Integration tests covering protocol boundaries
- Static allow-listed routing only:
  - `/`
  - `/static/style.css`
  - `/static/entries/*.html`

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

```bash
Client -> HTTP/1.1 (GET / HEAD only) -> Strict Request Validation -> +-- Reject malformed / oversized requests ->  Allow-listed Routing -> Static File Response
```

Routing occurs **only after validation**.

Malformed or oversized requests are rejected early and never reach routing logic.

---

## Static Content Model

Content is intentionally simple and flat:

- `static/index.html` — main news page
- `static/style.css` — minimal styling
- `static/entries/YYYY-MM-DD.html` — individual entries

Entries are embedded using **same-origin iframes**, which:
- avoid pagination logic
- avoid metadata aggregation
- keep each entry isolated
- simplify auditing and removal

---

## Security Design Principles

### 1. Deny by Default
Only explicitly allow-listed paths are served.
All other requests return `404`.

### 2. Strict Input Validation
- Maximum request size enforced
- Maximum request line length enforced
- Unsupported methods rejected (`405`)
- Malformed input rejected deterministically

### 3. Minimal Attack Surface
- No dynamic routing
- No user-controlled file paths
- No script execution
- No background tasks
- No external dependencies at runtime

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
- A strict `Content-Security-Policy`

These headers:
- prevent MIME confusion
- prevent clickjacking
- disable client-side caching
- reduce protocol ambiguity
- prevent script execution

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

Testing is deterministic and avoids mocks or async frameworks.

Run tests with:

```sh
cargo test
```

## Threat Model (Phase A)
### Assumptions

- Service binds locally

- TLS handled externally

- No trust in client input

### Non-Goals

- Content confidentiality

- Traffic analysis resistance

- Authenticated user security

These are addressed in later phases.

## Phase Status

Phase A — Application core: ✅ Complete

Phase B — Filesystem layout, non-root execution: ✅ Complete

Phase C — Tor onion service integration: ✅ Complete

Phase C+ — Tor client authentication: ⏳ In Progress

Phase D — Deployment & operations hardening: ⏳ Planned

## Directory Structure (Current)
```bash
├── news
│   ├── Cargo.lock
│   ├── Cargo.toml
│   ├── src
│   │   ├── config.rs
│   │   ├── http.rs
│   │   ├── lib.rs
│   │   ├── main.rs
│   │   └── server.rs
│   ├── static
│   │   ├── entries
│   │   │   └── 2026-02-04.html
│   │   ├── index.html
│   │   └── style.css
│   └── tests
│       └── http.rs
├── README.md
└── torrc

```

## Attribution

Content published by ***Th3 R0gu3 Kn!ghts***.

The service intentionally avoids embedding identifying metadata in responses or
served content.

## Contributions

This project is intentionally conservative.

Changes that:

- increase complexity

- introduce dynamic behavior

- expand the attack surface

will be rejected unless explicitly planned for in a future phase.