# Generated Code Notes

Everything under `src/generated/` is produced from the pinned NetBox OpenAPI schema (`schema/netbox-<ver>.json`) and verified by drift tests in `xtask/codegen` (`cargo test --workspace` fails on stale files). Three artifacts are generated:

- `endpoints.rs` — endpoint metadata (path, method, parameters, request/response refs) for the initial target paths.
- `types.rs` — typify-generated Rust structs for every `components.schemas` entry. Used at runtime by `validate_response` for drift detection.
- `resources/<resource>.rs` — per-resource typed CLI modules (clap arg structs, `ValueEnum` types for status/role enums, action enums, dispatchers, body builders).

## Schema normalizations applied before typify

The pinned OpenAPI document goes through `normalize_openapi_schema` (`xtask/codegen/src/lib.rs`) before typify generates types. The transforms:

- **`nullable: true` → `anyOf [{type: T}, {type: null}]`.** typify expects JSON Schema's nullability shape, not OpenAPI 3.0's `nullable` flag.
- **`readOnly` and `nullable` are kept independent.** OpenAPI 3.0 treats them as orthogonal flags (read-only is a write-permission concern; nullable is a value concern). Earlier nbx releases conflated them as a workaround for NetBox schemas that marked server-computed fields like `Interface.cable_end` `readOnly: true` without `nullable: true`, despite returning null at runtime. NetBox v4.5.10 corrected those declarations upstream, so the conflation is gone. If a future NetBox release reintroduces the same bug for another field, the runtime drift detector (`validate_response`) will surface it as a stderr warning so it can be filed upstream.
- **`exclusiveMaximum` / `exclusiveMinimum` flag form → JSON Schema 7 boundary form.** OpenAPI 3.0 has them as booleans; JSON Schema expects them as numbers.
- **`enum` arrays containing `null`** become nullable, with the `null` entry dropped.
- **`discriminator`, `example`, `externalDocs`, `readOnly`, `writeOnly`, `xml`, `x-spec-enum-id`** are stripped — typify doesn't consume them and they would otherwise pollute the output.

## Variant ident handling for enum properties

Per-resource `Writable<X>Request` enum-string fields are emitted as `ValueEnum` types (e.g., `SiteStatus`, `DeviceAirflow`) so the CLI shows `[possible values: ...]` in `--help` and does compile-time validation of `--status active` etc.

NetBox occasionally has enum values that produce colliding Rust ident names after PascalCase conversion. The clearest example is `WritableInterfaceRequest.type`: both `2.5gbase-t` and `25gbase-t` pascal-case to `Two5gbaseT`. The codegen disambiguates colliding variants with a numeric suffix (`Two5gbaseT2` for the second), preserving the wire string in the `#[value(name = "...")]` and `#[serde(rename = ...)]` attributes so the CLI surface and request/response shape stay identical.

Variants whose first character would be a digit (e.g., `4-post-frame`) get an English-word prefix instead (`FourPostFrame`).

## Foreign-key resolution

`FK_RESOLVERS` in `xtask/codegen/src/lib.rs` maps each `Brief<X>Request` schema name to the NetBox endpoint and lookup field used to resolve a name/slug to a numeric ID. Properties that match a known resolver are exposed as `String`/`Option<String>` (accepting names) rather than numeric IDs; the body builder calls `crate::commands::resolve_reference_id` at runtime to swap the lookup value for the integer NetBox expects. A unit test asserts every entry in the table points at a real path with the declared lookup field as a query parameter on that path's GET operation.

## Tags

Tag-shaped properties (`array<NestedTagRequest>`) are exposed as `--tags <csv>`. The body builder converts the CSV to `[{"slug": "..."}]`, which NetBox accepts for existing tag references.

## Wire payloads at runtime

Request and response payloads still travel as `serde_json::Value` end-to-end so agents see the exact shape NetBox returns (no field renames, no shape coercions). The generated `types.rs` is used at runtime only for drift detection — `validate_response` and `validate_bulk_response` deserialize the response through the typed struct and warn on stderr if the shape doesn't match. `NBX_SKIP_RESPONSE_VALIDATION=1` silences the check.

## Escape hatches

- Every typed mutation supports `--data <json>` / `--data-file <path>` for fields the codegen doesn't surface (custom_fields, complex array shapes, future fields). Flag values take precedence over `--data`.
- For resources we don't generate a CLI for, the typed surface is empty but the generated metadata in `endpoints.rs` and `types.rs` still exists for future use.
