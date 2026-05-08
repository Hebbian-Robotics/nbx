---
name: ipam-prefixes
description: NetBox IPAM prefix CRUD plus available-ips and available-prefixes allocation helpers.
---

# nbx ipam prefixes

## Purpose
List, retrieve, create, update, delete, and bulk-mutate NetBox IPAM prefixes. Includes the two NetBox availability endpoints — `available-ips` and `available-prefixes` — for next-hop allocation inside an existing prefix.

## When to use which action
- `list` — find prefixes by site, VRF, role, status, tag, or arbitrary `--query`.
- `get <prefix-or-id>` — fetch one prefix by numeric ID or by CIDR (default lookup field is `prefix`, e.g. `192.0.2.0/24`).
- `create` — register a new prefix. Required NetBox fields surfaced as required clap flags.
- `update <prefix-or-id>` — PATCH semantics.
- `delete <prefix-or-id> --confirm` — DELETE.
- `available-ips <prefix-or-id>` — list free IP addresses inside the prefix. With `--data <json>`, allocates one (POST).
- `available-prefixes <prefix-or-id>` — list free child prefixes. With `--data <json>` (e.g. `{"prefix_length": 28}`), allocates one (POST).
- `bulk-update --data <json-array>` / `bulk-delete --data <json-array> --confirm` — array payload by numeric `id`.

## Key behaviors
- **CIDR identifier round-trip.** `get`, `update`, `delete`, `available-*` accept the CIDR (e.g. `192.0.2.0/24`) directly thanks to the `prefix` lookup field. Override with `--lookup-field <field>` if needed.
- **Foreign-key name resolution.** `--site`, `--vrf`, `--role`, `--tenant` accept slugs/names; resolved to numeric IDs before the request is sent.
- **Allocation by presence of `--data`.** On `available-ips` / `available-prefixes`, omitting `--data` lists availability (GET); supplying `--data` allocates (POST). `--data-file <path>` is equivalent.
- **`--data` / `--data-file` precedence.** Typed flags merge into and override the JSON payload.
- **`--dry-run`.** Prints the resolved request (with FK resolution applied) without sending.

## Discovering flags and output
Run `nbx ipam prefixes --help` and `nbx ipam prefixes <action> --help` for the authoritative flag list and enums. Run mutations with `--dry-run` to preview the resolved request.

## Errors
- exit 1 `general` — network failure or unexpected NetBox response.
- exit 2 `not_found` — prefix or referenced FK not found.
- exit 3 `auth_failed` — token rejected.
- exit 4 `validation_error` — required field missing, FK name resolves to multiple IDs, allocation payload rejected by NetBox, or NetBox 4xx body.
