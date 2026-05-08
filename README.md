# nbx

A NetBox CLI built for humans at a terminal and agents in an automation loop.

## Why nbx exists

NetBox already has a broad REST API. `nbx` exists to make that API easier to use safely from a shell, a script, or an AI agent without writing one-off `curl` commands for every workflow.

- Single static binary, no Python or runtime to install on the target machine.
- Typed commands generated from NetBox's OpenAPI schema, so common resources get discoverable flags and enum validation.
- Name and slug resolution for common foreign keys, so humans can pass `--site dc1` instead of looking up numeric IDs first.
- Versioned JSON output envelope (`nbxVersion`, `schemaVersion`) so automation can depend on output shape.
- Meaningful exit codes (0/1/2/3/4) for branching without parsing stderr.
- Agent-readable `SKILL.md` files describing each command's purpose, parameters, output, and errors.

Target NetBox release: **v4.6.0**.

## Install

Prebuilt binaries for Linux (x86_64, aarch64), macOS (Intel, Apple Silicon), and Windows (x86_64-msvc) are attached to each GitHub Release by `.github/workflows/release.yml`.

### `cargo binstall` (recommended)

```sh
cargo install cargo-binstall   # one-time bootstrap
cargo binstall nbx
```

Resolves the right prebuilt archive from the [latest GitHub Release](https://github.com/Hebbian-Robotics/nbx/releases/latest) for your platform via `[package.metadata.binstall]` in `Cargo.toml`. No compilation. Sub-second install on a warm machine.

### Direct download

Grab the archive for your platform from the [latest release](https://github.com/Hebbian-Robotics/nbx/releases/latest), unpack it, and put `nbx` on your `$PATH`. Each release ships:

- `nbx-<version>-x86_64-unknown-linux-gnu.tar.gz`
- `nbx-<version>-aarch64-unknown-linux-gnu.tar.gz`
- `nbx-<version>-x86_64-apple-darwin.tar.gz`
- `nbx-<version>-aarch64-apple-darwin.tar.gz`
- `nbx-<version>-x86_64-pc-windows-msvc.zip`
- `SHA256SUMS` — verify with `shasum -a 256 -c SHA256SUMS`

### From crates.io source

```sh
cargo install nbx
```

Compiles the published source locally. Slower (minutes on a cold target dir) because typify-generated `src/generated/types.rs` is large and the dependency graph (reqwest, tokio, clap, …) gets built in release mode. Use this when no prebuilt suits your platform or you specifically want a from-source build.

### Local development build

```sh
cargo build --release
./target/release/nbx --help
```

## Quick start

```sh
# One-shot auth, JSON output, suitable for scripts and agents.
NETBOX_URL=https://netbox.example NETBOX_TOKEN=... \
  nbx dcim devices list --output json

# Persistent config for interactive terminal use.
nbx config init --url https://netbox.example --token <token>
nbx dcim devices list

# Fetch one resource by ID or default lookup value.
nbx devices get srv-db-07 --output json

# Create with typed flags and schema-derived enum validation.
nbx ipam vlans create --vid 100 --name mgmt --status active --output json
```

## Authentication

Resolution precedence (highest first):

1. `--url` / `--token` flags
2. `NETBOX_URL` / `NETBOX_TOKEN` env vars
3. Active context in `~/.config/nbx/config.toml`
4. Top-level `url` / `token` keys in the same config file (single-context legacy form)

NetBox v4.5 v2 tokens of the form `nbt_<key>.<token>` are sent as `Bearer`. Legacy v1 tokens use NetBox's `Token` scheme. The config file is written with mode `0600`; nbx warns when it reads a config with broader permissions.

### Contexts

For multiple NetBox instances (prod, staging, lab):

```sh
nbx config init --context prod --url https://netbox.corp.example --token ...
nbx config init --context staging --url https://netbox-staging.corp.example --token ...

nbx config use-context staging
nbx config current-context
nbx config get-contexts
```

Selection precedence: `--context <name>` > `NBX_CONTEXT` env var > `default_context` in the config file.

## How to Use It

Canonical form is `nbx <app> <resource> <action>`. Resource-level aliases (`nbx devices list`, `nbx prefixes get …`) are also accepted.

Preflight commands:

| Command | Purpose |
| --- | --- |
| `status` | Fetch `/api/status/` through the same output envelope; only a URL is required. |
| `authentication-check` | Verify the configured token against `/api/authentication-check/`. |

Devices, interfaces, and prefixes have hand-written extensions on top of the generated typed flags — composite addressing, name-resolved foreign keys, allocation helpers, and trace commands:

| Resource | Actions |
| --- | --- |
| `dcim devices` | `list`, `get`, `create`, `update`, `delete`, `bulk-update`, `bulk-delete` |
| `dcim interfaces` | `list`, `get`, `create`, `update`, `delete`, `trace`, `bulk-update`, `bulk-delete` |
| `ipam prefixes` | `list`, `get`, `create`, `update`, `delete`, `available-ips`, `available-prefixes`, `bulk-update`, `bulk-delete` |

`dcim devices list` accepts `--role` repeatedly to OR multiple device-role slugs in one query (e.g. `--role database-server --role application-server`).

`dcim inventory-items list` has typed `--device` (slug, name, or numeric ID; non-numeric values are resolved to a `device_id` via `/api/dcim/devices/`), `--role` (repeatable, OR), `--manufacturer`, and `--serial` filters in addition to the generated `--name` / `--tag`.

The other resources are fully generated from the schema. They take typed flags for scalar fields and supported foreign keys in their `Writable<X>Request` schema:

| Resource | Default lookup |
| --- | --- |
| `dcim cables` | `label` |
| `dcim device-roles` | `slug` |
| `dcim device-types` | `slug` |
| `dcim locations` | `slug` |
| `dcim manufacturers` | `slug` |
| `dcim platforms` | `slug` |
| `dcim regions` | `slug` |
| `dcim site-groups` | `slug` |
| `dcim sites` | `slug` |
| `dcim racks` | `name` |
| `dcim inventory-items` | `name` |
| `extras tags` | `slug` |
| `ipam aggregates` | `prefix` |
| `ipam ip-addresses` | `address` |
| `ipam rirs` | `slug` |
| `ipam roles` | `slug` |
| `ipam vlan-groups` | `slug` |
| `ipam vlans` | `name` |
| `ipam vrfs` | `name` |
| `tenancy tenant-groups` | `slug` |
| `tenancy tenants` | `slug` |

`get`, `update`, and `delete` accept either a numeric ID or the default lookup value (override with `--lookup-field`). Interfaces additionally accept the gNMI-style composite address `device:interface`, e.g. `srv01:Ethernet1`.

### Global flags

- `--output json|table|ndjson` — `table` on a TTY, `json` when piped, unless overridden.
- `--field a,b,c` — comma-separated projection with dot notation for nested fields.
- `-o name` — shorthand for projecting just `name`.
- `--query 'key=value&...'` — passthrough to NetBox filter parameters.
- `--limit <n>` / `--offset <n>` — pagination overrides for a single page.
- `--page-all` — stream every page as NDJSON (overrides `--output`).
- `--context <name>`, `--timeout <seconds>`, `--verbose`, `--dry-run`.

## Output contract

Every JSON or NDJSON record carries the envelope documented in [`SCHEMA.md`](./SCHEMA.md):

```json
{
  "nbxVersion": "4.6.0",
  "schemaVersion": 1,
  "data": { "id": 42, "name": "srv-db-07" }
}
```

List responses include the NetBox pagination envelope alongside `nbxVersion` / `schemaVersion`:

```json
{
  "nbxVersion": "4.6.0",
  "schemaVersion": 1,
  "count": 137,
  "next": "https://netbox.example/api/dcim/devices/?limit=50&offset=50",
  "previous": null,
  "results": [ { "id": 1, "name": "..." } ]
}
```

`schemaVersion` is the agent contract version and is independent of the nbx crate version. It bumps only on a breaking output-contract change; additive changes (new fields, new error codes) do not. The contract is enforced by snapshot tests in [`src/output_snapshots.rs`](./src/output_snapshots.rs) whose names embed the current `schemaVersion` — any breaking shape change forces a same-PR bump.

## Errors and exit codes

Errors are JSON objects with `code`, `message`, and `detail`:

```json
{
  "nbxVersion": "4.6.0",
  "schemaVersion": 1,
  "error": {
    "code": "not_found",
    "message": "NetBox resource not found",
    "detail": { "status": 404, "response": { "detail": "Not found." } }
  }
}
```

| Exit | Code | HTTP triggers |
| --- | --- | --- |
| 0 | success | 2xx |
| 1 | `general` / `stream_error` | 5xx, network, timeout, mid-stream NDJSON failure |
| 2 | `not_found` | 404 |
| 3 | `auth_failed` | 401, 403 |
| 4 | `validation_error` | 400, 422 |

429 responses are retried up to three times, honoring `Retry-After` (or 1s/2s/4s backoff if absent).

`--page-all` streams one NDJSON record per result. If a request fails mid-stream, nbx emits a final record with `error.code: "stream_error"` and exits non-zero — the stream is never silently truncated.

## Coverage, Caveats, and Escape Hatches

`nbx` is intentionally opinionated around the highest-value operational workflows first: DCIM devices/interfaces/cables/sites/racks/reference objects, IPAM prefixes/IP addresses/VLANs/VRFs/reference objects, Extras tags, and Tenancy tenants/groups. The schema-generated resource modules make this surface broad enough for day-to-day work while keeping the CLI predictable.

`nbx` is built against the OpenAPI schema from a single NetBox release (currently `v4.6.0`; see `schema/README.md`). On the first request of each session it probes `/api/status/`. If the running NetBox's `major.minor` doesn't match the pinned target, nbx prints a one-line warning to stderr and continues; behavior outside the pinned `major.minor` is best-effort.

Set `NBX_SKIP_VERSION_CHECK=1` to suppress the probe (useful in CI against many NetBox versions, or when calling `/api/status/` is restricted).

A weekly `Schema drift` GitHub Actions job (`.github/workflows/schema-drift.yml`) compares the pinned schema against the latest NetBox 4.6.x release and opens a tracking issue on diff. Bumping the pin is a deliberate PR documented in `schema/README.md`.

Use these escape hatches when the typed surface does not cover the exact request you need:

- `--query 'key=value&...'` passes NetBox filter parameters through unchanged.
- `--data <json>` / `--data-file <path>` supplies complex mutation fields that do not fit the initial typed-flag model, such as cable terminations, custom fields, and tag `object_types`. Typed flags take precedence when both set the same field.
- `raw` calls any NetBox API path under `/api/` with `GET`, `POST`, `PATCH`, or `DELETE`. It uses the same authentication, retry, version-probe, envelope, projection, pagination, and dry-run behavior as typed commands.

```sh
nbx raw GET /api/plugins/example/widgets/ --query 'name=leaf-1' --output json
nbx raw POST /api/plugins/example/widgets/ --data '{"name":"leaf-1"}' --output json
```

Prefer a typed command when one exists; use `raw` for plugin endpoints or NetBox endpoints that do not have a typed nbx command yet.

## Agent integration

Skill files live under `skills/<name>/SKILL.md`, the layout the [Vercel skills CLI](https://github.com/vercel-labs/skills) expects. Each is a Claude-format `SKILL.md` with frontmatter (`name`, `description`) and `## Purpose` / `## Parameters` / `## Example` / `## Errors` sections — readable by any agent that can parse Markdown.

```
skills/
├── README.md
├── config/SKILL.md
├── generic-resource-actions/SKILL.md
├── dcim-devices-list/SKILL.md
├── dcim-devices-get/SKILL.md
├── dcim-devices-create/SKILL.md
├── dcim-interfaces-list/SKILL.md
├── dcim-interfaces-get/SKILL.md
├── dcim-interfaces-create/SKILL.md
├── dcim-interfaces-update/SKILL.md
├── dcim-interfaces-trace/SKILL.md
├── ipam-prefix-availability/SKILL.md
├── raw/SKILL.md
└── preflight/SKILL.md
```

### Install via `npx skills`

```sh
# discover what's available
npx skills add Hebbian-Robotics/nbx --list

# install all nbx skills into Claude Code globally
npx skills add Hebbian-Robotics/nbx --all -g -a claude-code -y

# install specific skills
npx skills add Hebbian-Robotics/nbx \
  --skill dcim-interfaces-list \
  --skill dcim-interfaces-update \
  -a claude-code
```

Other supported agents (`-a <name>`): `amp`, `codex`, `cursor`, `gemini-cli`, `goose`, `opencode`, `windsurf`, and more — see the skills CLI docs.

### Agent flow

1. Read the relevant `SKILL.md` for command name, parameters, and expected output shape.
2. Run the command with `--output json` (or `--page-all` for NDJSON), passing `NETBOX_URL` / `NETBOX_TOKEN` via env.
3. Parse the response. Branch on the exit code; for non-zero, parse `error.code` for typed handling.

```sh
NETBOX_URL=https://netbox.example NETBOX_TOKEN=$TOKEN \
  nbx dcim devices get srv-db-07 --output json
# exit 0 + envelope on success
# exit 2 + { "error": { "code": "not_found", ... } } if missing
```

## Shell completions

```sh
nbx completions zsh  > ~/.zfunc/_nbx
nbx completions bash > /etc/bash_completion.d/nbx
nbx completions fish > ~/.config/fish/completions/nbx.fish
```

## Development

```sh
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```

### Codegen architecture

Three artifacts are generated from `schema/netbox-4.6.0.json`:

- `src/generated/types.rs` — every NetBox component schema, via `typify`. Compile-checked.
- `src/generated/endpoints.rs` — endpoint metadata (path, method, parameters, request/response refs) for the initial target paths.
- `src/generated/resources/<resource>.rs` — per-resource typed CLI modules: `<Resource>{Create,Update}Args` clap structs, `ValueEnum` types for status/role enums, `<Resource>Action` subcommand enum, `run_<resource>_action` dispatcher, and create/update body builders. Generated for the initial agent-focused surface: DCIM sites/site-groups/regions/locations/manufacturers/device-types/device-roles/platforms/racks/inventory-items/cables, Extras tags, IPAM IP addresses/prefixes/VRFs/roles/VLAN groups/RIRs/aggregates/VLANs, and Tenancy tenant groups/tenants.

Devices, interfaces, and prefixes use the same generated typed flags (as `<Resource>{Create,Update}Fields`) but add hand-written extensions in `src/commands/mod.rs`: composite `device:interface` addressing, VLAN/LAG name resolution, the `interfaces trace` endpoint, and prefix availability allocation — none of which reduce cleanly to schema-shaped CRUD. The codegen marks them with `extended: true` and skips emitting the dispatcher and `--data` flags so the hand-written wrapper supplies them.

#### Adding a new resource

Most of `src/` is generated, so exposing a new NetBox endpoint is small:

1. Add the endpoint path (e.g. `/api/dcim/new-things/`) to `V0_1_TARGET_ENDPOINT_PATHS` in [`xtask/codegen/src/lib.rs`](./xtask/codegen/src/lib.rs).
2. If the new resource references foreign keys reachable by name/slug, add an entry to `FK_RESOLVERS` in the same file.
3. Regenerate (`cargo run -p nbx-codegen -- ...` per the commands above).
4. Wire the generated module into `src/commands/mod.rs` so it's reachable from the CLI dispatcher.
5. (Optional) Add a `SKILL.md` under `skills/` so agents discover the new command.
6. `cargo test --workspace` — drift checks in `xtask/codegen` confirm the generated files match the pinned schema.

For endpoints not yet exposed as typed commands, agents can already call them through `nbx raw <method> <path>`, which inherits the same auth, output envelope, retry, projection, and dry-run behavior.

The runtime (HTTP client, auth, retries, output envelope, error model, projection) is hand-written by design — that's where the agent-facing opinions live.

Regenerate:

```sh
cargo run -p nbx-codegen -- schema/netbox-4.6.0.json src/generated/endpoints.rs
cargo run -p nbx-codegen -- schema/netbox-4.6.0.json src/generated/types.rs
cargo run -p nbx-codegen -- schema/netbox-4.6.0.json src/generated/resources/
```

`cargo test --workspace` fails if any committed file under `src/generated/` drifts from what the codegen would produce against the pinned schema. The drift checks live in `xtask/codegen` so a published `cargo install nbx` doesn't pull in the codegen toolchain. A NetBox version bump is a deliberate PR: update `schema/`, regenerate, review the diff, run tests and integration.

A `netbox-docker` integration job lives under `tests/integration/` and runs in CI.

## License

Apache-2.0. See [`LICENSE`](./LICENSE).
