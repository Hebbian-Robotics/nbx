---
name: generic-resource-actions
description: Shared CRUD pattern for NetBox resources whose entire CLI surface is generated from the OpenAPI schema.
---

# nbx Generic Resource Actions

## Purpose
A single skill covering the resources whose CLI surface is generated wholesale from the NetBox OpenAPI schema. They share one CRUD pattern (list / get / create / update / delete / bulk-update / bulk-delete) and one set of conventions for typed flags, FK resolution, and `--data` escape-hatching.

Covered resources:

- DCIM: `cables`, `device-roles`, `device-types`, `inventory-items`, `locations`, `manufacturers`, `platforms`, `racks`, `regions`, `site-groups`, `sites`
- Extras: `tags`
- IPAM: `aggregates`, `ip-addresses`, `rirs`, `roles`, `vlan-groups`, `vlans`, `vrfs`
- Tenancy: `tenant-groups`, `tenants`

`dcim devices`, `dcim interfaces`, and `ipam prefixes` use the same generated typed flags but add hand-written extensions on top — they have dedicated skills (`dcim-devices`, `dcim-interfaces`, `ipam-prefixes`).

## When to use which action
- `list` — find by typed filters (`--name`, `--tag`, etc.) or arbitrary `--query 'key=value'` for unsurfaced filters. `--page-all` streams every page as NDJSON.
- `get <id-or-lookup>` — fetch by numeric ID or by the resource's default lookup field. Common defaults: `slug` for named reference objects, `name` for racks/VLANs/VRFs, `address` for IP addresses, `prefix` for aggregates/prefixes.
- `create` — typed flags for every property in the resource's `Writable<X>Request` schema. Required schema fields are required clap flags.
- `update <id-or-lookup>` — same flag set as `create`, all optional. PATCH semantics.
- `delete <id-or-lookup> --confirm` — DELETE.
- `bulk-update --data <json-array>` / `bulk-delete --data <json-array> --confirm` — array payload by numeric `id`.

## Key behaviors
- **Foreign-key name resolution.** Typed FK flags accept slug or name (resolved at runtime via the NetBox lookup endpoint declared in `FK_RESOLVERS`) or a numeric ID directly.
- **`--data` / `--data-file` escape hatch.** Available on every mutation for fields the codegen does not surface (custom_fields, complex array shapes, tag `object_types`, cable terminations, etc.). Typed flags merge into and override `--data` when both supply the same field.
- **`--tags <csv>`.** Comma-separated tag slugs; nbx converts the CSV to NetBox's `[{"slug": "..."}]` shape.
- **`--dry-run`.** Prints the resolved request without sending.
- **Default lookups vary by resource.** Use `--lookup-field <field>` to override (e.g. on a resource where the default is `slug` but you want to look up by `name`).

## Discovering flags and output
Run `nbx <app> <resource> --help` and `nbx <app> <resource> <action> --help` for the authoritative flag list, enum values (`[possible values: ...]`), and required/optional markers. Each resource has its own typed flag set derived from its schema. Run mutations with `--dry-run` to preview.

## Errors
- exit 1 `general` — network failure or unexpected NetBox response.
- exit 2 `not_found` — resource or referenced FK not found.
- exit 3 `auth_failed` — token rejected.
- exit 4 `validation_error` — required field missing, FK name resolves to multiple IDs, or NetBox 4xx body.
