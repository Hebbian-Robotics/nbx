---
name: preflight
description: Verify NetBox reachability and token authentication before running mutations.
---

# nbx preflight

## Purpose
Two pre-flight commands that confirm nbx can talk to NetBox before you run resource mutations:

- `nbx status` — fetches `/api/status/`. Confirms the URL is reachable and reports the NetBox version. No token required.
- `nbx authentication-check` — fetches `/api/authentication-check/`. Confirms the configured token is accepted.

## When to use which action
- `status` — first call when bringing up a new context or debugging connectivity. Does not need a token.
- `authentication-check` — second call once a token is configured, to confirm it works before attempting writes. Required token; returns the authenticated user identity on success.

## Key behaviors
- **Status works token-less.** Useful for verifying that a URL points at a real NetBox before bothering with credentials.
- **Output is the standard nbx envelope.** Both commands wrap the NetBox response in `data` and include `nbxVersion` / `schemaVersion`, so agents can branch on the same shape as resource commands.
- **Selection precedence.** `--context <name>` > `NBX_CONTEXT` env var > `default_context` in `~/.config/nbx/config.toml`. `--url` / `NETBOX_URL` and `--token` / `NETBOX_TOKEN` override the context's values.

## Discovering flags and output
Run `nbx status --help` and `nbx authentication-check --help` for the flag list (global options like `--url`, `--token`, `--context`, `--output`, `--field` apply).

## Errors
- exit 1 `general` — network failure, DNS error, or timeout.
- exit 2 `not_found` — the URL points at something other than a NetBox instance (404 on `/api/status/`).
- exit 3 `auth_failed` — token rejected on `authentication-check`.
- exit 4 `validation_error` — missing URL.
