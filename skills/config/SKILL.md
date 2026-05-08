---
name: config
description: Manage nbx NetBox URL, token, and named contexts in ~/.config/nbx/config.toml.
---

# nbx config

## Purpose
Manage `~/.config/nbx/config.toml` — nbx's persistent connection store. Supports multiple named contexts (e.g. `prod`, `staging`) and a default-context selector.

## When to use which action
- `init --url <url> --token <token> --context <name>` — write the initial context. Use this once when bringing up a fresh install.
- `set-url <url> [--context <name>]` — update the URL on a context (defaults to the active context).
- `set-token <token> [--context <name>]` — update the API token on a context.
- `use-context <name>` — switch the default context.
- `current-context` — print the currently selected context name.
- `get-contexts` — list all configured contexts.
- `show` — print the resolved config with tokens redacted; pass `--show-token` to include them.

## Key behaviors
- **File permissions.** `~/.config/nbx/config.toml` is written with mode `0600`. nbx refuses to read a config file that is world- or group-readable.
- **Selection precedence at runtime.** `--context <name>` > `NBX_CONTEXT` env var > `default_context` in the config file. Resource commands inherit this precedence; `config` itself uses `--context` to scope edits.
- **Tokens are never echoed by default.** `show` redacts; agents that need the raw token should use `--show-token` deliberately.

## Discovering flags and output
Run `nbx config --help` and `nbx config <action> --help` for the flag list and behavior of each subaction.

## Errors
- exit 1 `general` — filesystem failure (e.g. cannot create `~/.config/nbx/`).
- exit 2 `not_found` — context name does not exist.
- exit 4 `validation_error` — malformed URL, missing required field, or refusing to read a config with permissive permissions.
