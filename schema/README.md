# NetBox Schema Pin

Target NetBox release: `v4.5.10`

Pinned schema: `schema/netbox-4.5.10.json`

Observed OpenAPI version: `3.0.3`

Fetch command:

```sh
curl -fsSL 'https://raw.githubusercontent.com/netbox-community/netbox/v4.5.10/contrib/openapi.json' -o schema/netbox-4.5.10.json
```

SHA-256:

```text
2e1e2ac57e18bcf24869f8fac8997749c5b988d1ea83f0fcf423f576d19987c6
```

This file is the official OpenAPI artifact committed in the NetBox `v4.5.10` tag. Retargeting NetBox requires updating this schema, regenerating metadata, and reviewing generated diffs.

## Bumping to a new NetBox release

`nbx`'s crate version tracks NetBox's release version (e.g., `nbx 4.5.10` is built from `netbox-community/netbox` tag `v4.5.10`). To target a new NetBox release `vX.Y.Z`:

1. **Drop in the new schema and remove the old.**

   ```sh
   curl -fsSL "https://raw.githubusercontent.com/netbox-community/netbox/vX.Y.Z/contrib/openapi.json" \
     -o schema/netbox-X.Y.Z.json
   git rm schema/netbox-<old>.json
   ```

2. **Mass-replace the version string in tracked files.**

   ```sh
   fd -e rs -e toml -e md -e yml -e sh -e json -t f \
       --exclude target --exclude .git . \
     | xargs grep -l "<old>" \
     | xargs sed -i '' 's|<old>|X.Y.Z|g'
   ```

   This catches `Cargo.toml`, `xtask/codegen/Cargo.toml`, `tests/integration/{run.sh,docker-compose.yml,README.md}`, `.github/workflows/{ci.yml,schema-drift.yml}`, `README.md`, this file, and the auto-generated header comments in `src/generated/`.

3. **Refresh the schema SHA in this file.**

   ```sh
   shasum -a 256 schema/netbox-X.Y.Z.json
   ```

   Paste the hash into the SHA-256 block above.

4. **Regenerate the three artifacts under `src/generated/`.**

   ```sh
   cargo run -p nbx-codegen -- schema/netbox-X.Y.Z.json src/generated/endpoints.rs
   cargo run -p nbx-codegen -- schema/netbox-X.Y.Z.json src/generated/types.rs
   cargo run -p nbx-codegen -- schema/netbox-X.Y.Z.json src/generated/resources/
   ```

5. **Build, lint, test, integrate.**

   ```sh
   cargo build
   cargo clippy --all-targets --all-features -- -D warnings
   cargo test --workspace
   tests/integration/run.sh
   ```

   The drift tests in `xtask/codegen` verify the regenerated files are byte-identical to what `nbx-codegen` would produce; if you skipped step 4, `cargo test --workspace` fails with a clear "stale" error pointing at the file and the regen command. The `fk_resolvers_match_pinned_schema` test asserts every entry in the codegen's `FK_RESOLVERS` table maps to a real path with a real query parameter in the new schema. The integration suite spins up `netboxcommunity/netbox:vX.Y.Z` and exercises the typed surface end-to-end.

6. **Review the generated diff.**

   `src/generated/types.rs` is large (~7 MB) but most patch-release bumps produce small, mostly-additive diffs. `git diff src/generated/resources/` is the most useful view — it surfaces new typed flags, renamed enum values, and new required fields. Major.minor bumps need closer review: an enum value rename or a removed field can break downstream agents.

7. **Update `CHANGELOG.md`.** Add a new `## [X.Y.Z] — YYYY-MM-DD` section. To decide what goes in it, run:

   ```sh
   # User-visible signal: added/removed pub items in generated resources
   git diff src/generated/resources/ \
     | rg "^[+-](pub (struct|enum|fn|const)|pub [a-z_]+:)"

   # Hand-written code changes
   git diff src/commands/mod.rs src/client.rs src/output.rs src/error.rs

   # Whole-tree summary
   git diff --stat src/generated/ src/commands/ src/client.rs src/output.rs
   ```

   If both are empty, the entry honestly says `_No CLI surface changes from <prev>. Pin advanced for runtime version-probe parity._` plus a link to the upstream NetBox release notes — that's still worth recording. If anything is non-empty, the entry summarizes what changed (new flags, renamed enums, etc.).

8. **Commit and push.** A single commit titled `chore: target NetBox vX.Y.Z` with a body that calls out:
   - the new SHA,
   - any user-visible flag changes (added enum values, new required fields, renamed flags),
   - integration-suite results.

The `Schema drift` GitHub Actions workflow (`.github/workflows/schema-drift.yml`) opens an issue weekly when a newer NetBox 4.5.x release diverges from the pin, so you can usually time the bump to that nudge.
