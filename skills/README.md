# nbx Agent Skills

Claude-format skill files describing the nbx command surface for AI agents. Every skill lives at `skills/<name>/SKILL.md`, the layout the [Vercel skills CLI](https://github.com/vercel-labs/skills) (`npx skills add …`) expects.

Skills are scoped one-per-top-level-resource. They cover *purpose, when to use which action, and behaviors that aren't visible in `--help`* (name resolution, composite addressing, `--data` precedence, dry-run semantics, multi-match errors). They intentionally do **not** enumerate flags or hardcode example output — `nbx <resource> [<action>] --help` is the authoritative source for flags, enum values, and required/optional markers, and `--dry-run` is the authoritative way to preview a request shape. This keeps the skill files short and stops them from drifting silently when the codegen adds or renames a flag.

Resources with hand-written extensions on top of the generated typed flags get dedicated skills:

- `dcim-devices` — list / get / create / update / delete / bulk-update / bulk-delete
- `dcim-interfaces` — list / get / create / update / delete / trace / bulk-update / bulk-delete (composite `device:interface` addressing, VLAN/LAG name resolution)
- `ipam-prefixes` — list / get / create / update / delete / available-ips / available-prefixes / bulk-update / bulk-delete

Resources whose CLI surface is fully generated share a single umbrella skill:

- `generic-resource-actions` — covers ~20 generated reference/core CRUD resources across `dcim`, `extras`, `ipam`, and `tenancy`

Utilities:

- `raw` — authenticated passthrough to any NetBox `/api/` path that does not yet have a typed nbx command
- `preflight` — `status` and `authentication-check` for connectivity and token verification
- `config` — nbx URL/token/context management

Each skill has YAML frontmatter (`name`, `description`) and `## Purpose`, `## When to use`, `## Key behaviors`, `## Discovering flags`, `## Errors` sections. `tests/skills_lint.rs` enforces that shape in CI.
