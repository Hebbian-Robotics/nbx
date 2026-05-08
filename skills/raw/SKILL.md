---
name: raw
description: Authenticated passthrough to any NetBox /api/ path that does not yet have a typed nbx command.
---

# nbx raw

## Purpose
Call any NetBox API path under `/api/` (including plugin endpoints) through nbx's auth, retry, and output envelope. Use this when no typed `nbx <app> <resource>` command exists for the endpoint you need.

## When to use which action
Single invocation: `nbx raw <method> <path>`. `<method>` is `GET`, `POST`, `PATCH`, or `DELETE`; `<path>` must start with `/api/` or `api/`.

- Reach for `raw` when: the endpoint is a NetBox plugin route, or it's a core NetBox endpoint that does not yet have a typed nbx command.
- Prefer typed commands (`nbx dcim devices ...`, `nbx ipam prefixes ...`, etc.) when available — they give you flag-level help, enum validation, and FK name resolution that `raw` does not.

## Key behaviors
- **Inherits the full nbx runtime.** Connection resolution, authentication, retries, version probing, JSON envelopes, projection (`--field`), pagination (`--limit` / `--offset` / `--page-all`), and `--dry-run` all work identically to typed commands.
- **No generated metadata required.** `raw` does not consult `endpoints.rs`, so it works against any path NetBox accepts.
- **Inline path queries are accepted.** Both `nbx raw GET /api/foo/?name=bar` and `nbx raw GET /api/foo/ --query 'name=bar'` work.
- **`--page-all` for raw GET list endpoints** streams every page as NDJSON.
- **`--data` / `--data-file`** supplies the JSON request body for mutations.
- **`--dry-run`** prints the raw request (method, path, headers excl. token, body) without sending.

## Discovering flags and output
Run `nbx raw --help` for the flag list. Output shape mirrors what NetBox returns at the requested path, wrapped in the standard nbx envelope (`data` for object responses, `count`/`results` for paginated lists, `error` on failure).

## Errors
- exit 1 `general` — network failure, timeout, or unexpected response shape.
- exit 2 `not_found` — NetBox returned 404 for the requested path.
- exit 3 `auth_failed` — token rejected.
- exit 4 `validation_error` — malformed path, unsupported method, or NetBox 4xx body.
