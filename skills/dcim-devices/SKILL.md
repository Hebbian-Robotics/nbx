---
name: dcim-devices
description: NetBox DCIM device CRUD with foreign-key name resolution and bulk operations.
---

# nbx dcim devices

## Purpose
List, retrieve, create, update, delete, and bulk-mutate NetBox DCIM devices. Devices are the primary resource in DCIM; this command exposes them with hand-written extensions on top of the schema-generated typed flags so that common foreign keys (site, role, device-type, rack) accept slugs or names.

## When to use which action
- `list` — find devices by site, role, status, tag, or arbitrary `--query key=value`. Stream with `--page-all` for inventory exports.
- `get <id-or-name>` — fetch one device by numeric ID or by name (default lookup field).
- `create` — register a new device. Required schema fields (e.g. `name`, `device_type`, `site`, `role`) are required clap flags.
- `update <id-or-name>` — PATCH semantics; every flag is optional, only supplied fields are sent.
- `delete <id-or-name> --confirm` — single-detail DELETE.
- `bulk-update --data <json-array>` / `bulk-delete --data <json-array> --confirm` — array-of-objects payload, one PATCH/DELETE per element.

## Key behaviors
- **Foreign-key name resolution.** `--site`, `--role`, `--device-type`, `--rack`, `--platform`, `--tenant`, `--location` accept either a slug/name (resolved to a numeric ID at runtime via the NetBox lookup endpoint) or a numeric ID directly.
- **Repeatable `--role` on `list`.** Pass `--role db --role app` to OR multiple device-role slugs in one query.
- **`--data` / `--data-file` precedence.** Typed flags merge into and override the JSON payload. Use `--data` for `custom_fields` and any field codegen does not surface.
- **`--dry-run`.** On any mutation, prints the resolved request (with FK resolution applied) instead of sending. The output envelope still carries `nbxVersion` and `schemaVersion`.
- **Bulk operations.** `bulk-update` / `bulk-delete` accept a JSON array via `--data` or `--data-file`; each array element must include the device's numeric `id`.

## Discovering flags and output
Run `nbx dcim devices --help` and `nbx dcim devices <action> --help` for the authoritative flag list, enum values (`[possible values: ...]`), and required/optional markers. Run any mutation with `--dry-run` to preview the resolved request body.

## Errors
- exit 1 `general` — network failure or unexpected NetBox response.
- exit 2 `not_found` — device or referenced FK (site, role, device-type) not found.
- exit 3 `auth_failed` — token rejected.
- exit 4 `validation_error` — required field missing, FK name resolves to multiple IDs, or NetBox 4xx body.
