# Changelog

All notable changes to nbx are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the
versioning aligns with the NetBox release nbx is generated from
(see `schema/README.md`). Output envelope `schemaVersion` is the
agent contract version and is independent of the crate version.

## [Unreleased]

No unreleased changes.

## [4.5.10] — 2026-05-06

Initial public release. nbx targets [NetBox v4.5.10](https://github.com/netbox-community/netbox/releases/tag/v4.5.10).

### Highlights

- **Schema-driven typed CLI.** Every flag, every enum, every body
  builder under `src/generated/` is regenerated from NetBox's pinned
  OpenAPI schema. The initial typed surface covers DCIM
  sites/site-groups/regions/locations/manufacturers/device-types/
  device-roles/platforms/racks/inventory-items/cables/devices/
  interfaces, Extras tags, IPAM IP addresses/prefixes/VRFs/roles/
  VLAN groups/RIRs/aggregates/VLANs, and Tenancy tenant groups/
  tenants. Drift checks live as workspace tests in `xtask/codegen`
  so a published `cargo install nbx` never pulls in the codegen
  toolchain.
- **Foreign-key name resolution.** 27 NetBox resources are reachable
  by slug or name on the typed flag (`--site dc1`,
  `--device-type acme-server`, etc.). A unit test asserts every
  resolver entry maps to a real path with the declared lookup field
  as a query parameter on that path.
- **Marquee extensions for devices and interfaces.** Hand-written on
  top of the generated flags: composite addressing
  (`nbx interfaces get srv01:Ethernet1`), VLAN/LAG name resolution,
  and the `interfaces trace` command.
- **Typed list filters for common workflows.** `dcim devices list
  --role` accepts repeated values to OR multiple device-role slugs in
  one query. `dcim inventory-items list` adds typed `--device`,
  `--role`, `--manufacturer`, and `--serial` filters on top of the
  generated `--name` / `--tag` flags.
- **Raw API fallback.** `nbx raw <method> <path>` calls any NetBox API
  path under `/api/` with the same authentication, retry, output
  envelope, projection, pagination, and dry-run behavior as typed
  commands. This covers plugin endpoints and NetBox endpoints that do
  not have typed nbx commands yet.
- **Tags via `--tags <csv>`.** Comma-separated tag slugs on every
  resource that has a `tags` field; body builder produces the
  NetBox-shaped `[{"slug": "..."}]` array.
- **Versioned JSON output envelope.** Every JSON / NDJSON record
  carries `nbxVersion` (currently `"4.5.10"`) and `schemaVersion`
  (`1`, the agent contract version). NDJSON streaming via
  `--page-all` emits one record per line.
- **Meaningful exit codes.** `0` success, `1` general,
  `2` not_found, `3` auth_failed, `4` validation_error.
- **Multi-context auth.** `~/.config/nbx/config.toml` supports
  named contexts; `--context` / `NBX_CONTEXT` / `default_context`
  with explicit precedence. Config files are written `0600`.
- **Runtime drift detection.** Each resource's response is
  deserialized through its typify-generated type after a successful
  call; mismatches emit a one-line stderr warning. Silenceable with
  `NBX_SKIP_RESPONSE_VALIDATION=1`.
- **Runtime version probe.** First request of each session probes
  `/api/status/`; if the running NetBox's `major.minor` doesn't
  match nbx's, a stderr warning fires.
- **Agent-format SKILL files.** `skills/<name>/SKILL.md` shipped for
  every command following the [Vercel skills](https://github.com/vercel-labs/skills)
  layout. `npx skills add Hebbian-Robotics/netbox-cli --all -g`
  installs them.
- **Schema-drift watcher.** Weekly GitHub Actions job opens a
  tracking issue when upstream NetBox 4.5.x diverges from the pin.
- **Multi-version integration matrix.** CI runs `tests/integration/`
  against `netboxcommunity/netbox:v4.5.10` and `v4.5.9` — both
  exercised live with all typed flag, FK resolution, and bulk-op
  paths.
- **Clean-install smoke test.** CI runs `cargo install --path .
  --locked` on a fresh runner and exercises the binary from `/tmp`.

### Removed

- The `readOnly → nullable` schema normalization workaround in
  `xtask/codegen`. NetBox v4.5.10 properly marks `Interface.cable_end`
  (and the rest of the `CabledObjectModel` family) `nullable: true`,
  fixing the upstream OpenAPI bug we were locally papering over. The
  codegen now follows OpenAPI semantics exactly: `readOnly: true` and
  `nullable: true` are independent flags. If a future NetBox release
  introduces another `readOnly` field that returns null without
  declaring `nullable: true`, the runtime drift detector will surface
  it as a stderr warning so it can be filed upstream.

### Upstream context

NetBox v4.5.10 is the first release where every Interface response
field declared in `Interface.required` is honest about its actual
nullability. See [NetBox release notes](https://github.com/netbox-community/netbox/blob/main/docs/release-notes/version-4.5.md)
for the wider list of upstream changes.
