---
name: dcim-interfaces
description: NetBox DCIM interface CRUD with composite addressing, VLAN/LAG name resolution, and cable trace.
---

# nbx dcim interfaces

## Purpose
List, retrieve, create, update, delete, trace, and bulk-mutate NetBox DCIM interfaces. Interfaces are first-class alongside devices; this command supplies the composite addressing, VLAN/LAG resolution, and cable-path trace that schema-shaped CRUD does not cover.

## When to use which action
- `list` — find interfaces by `--device`, `--type`, `--mode`, `--enabled`, `--mgmt-only`, `--tag`, `--name`, or arbitrary `--query`. Use `--page-all` for full streams.
- `get <address>` — fetch one interface. Address is either a numeric ID or `device:interface` (e.g. `srv01:Ethernet1`).
- `create` — create one interface. Required NetBox fields are `device`, `name`, `type`.
- `update <address>` — PATCH; every mutation flag is optional.
- `delete <address> --confirm` — DELETE one interface.
- `trace <address>` — walk the cable path attached to the interface; returns each hop as an array of `(termination, cable, termination)` three-tuples.
- `bulk-update --data <json-array>` / `bulk-delete --data <json-array> --confirm` — array payload by numeric `id`.

## Key behaviors
- **Composite addressing.** Interface names are not globally unique (every device has its own `Ethernet1`), so single-interface actions take `device:interface` (gNMI-style). Bare names are rejected with a `validation_error`. Numeric IDs always work.
- **Name resolution for VLANs, LAGs, and devices.** `--device`, `--untagged-vlan`, `--lag`, and each entry in `--tagged-vlans <csv>` are looked up by name/slug and replaced with numeric IDs before the request is sent. `--lag` resolution requires `--device` to be set so the lookup is scoped to the same device.
- **Multi-match resolution → validation_error.** If a VLAN/LAG name resolves to multiple IDs, nbx errors with a hint to pass the numeric ID directly.
- **`--data` / `--data-file` precedence.** Typed flags merge into and override the JSON payload.
- **`--dry-run`.** Prints the resolved request (with all name resolution applied) without sending.
- **`-o name` shorthand on `get`.** Projects the `name` field only.

## Discovering flags and output
Run `nbx dcim interfaces --help` and `nbx dcim interfaces <action> --help` for the authoritative flag list, type enums, and switching-mode values. Run mutations with `--dry-run` to preview the resolved request.

## Errors
- exit 1 `general` — network failure or unexpected NetBox response.
- exit 2 `not_found` — device or interface not found at the composite address.
- exit 3 `auth_failed` — token rejected.
- exit 4 `validation_error` — malformed address (bare name, empty segment), composite that resolves to multiple interfaces, multi-match VLAN/LAG name, or NetBox 4xx body.
