# nbx JSON Schema Contract

Current schema version: `1`

Every JSON and NDJSON record includes:

```json
{
  "nbxVersion": "4.5.10",
  "schemaVersion": 1
}
```

Single-object success:

```json
{
  "nbxVersion": "4.5.10",
  "schemaVersion": 1,
  "data": {}
}
```

List success:

```json
{
  "nbxVersion": "4.5.10",
  "schemaVersion": 1,
  "count": 0,
  "next": null,
  "previous": null,
  "results": []
}
```

Error:

```json
{
  "nbxVersion": "4.5.10",
  "schemaVersion": 1,
  "error": {
    "code": "not_found",
    "message": "resource not found",
    "detail": {}
  }
}
```

Exit codes:

- `0`: success
- `1`: general error, network error, timeout, or stream error
- `2`: not found
- `3`: auth failure
- `4`: validation error

`schemaVersion` is the agent contract version and is independent of the nbx crate version. It bumps only on a breaking output-contract change; additive changes (new fields, new error codes) do not. Enforced by snapshot tests in `src/output_snapshots.rs`.
