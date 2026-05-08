# Docker Integration

This harness runs `nbx` against `netboxcommunity/netbox:v4.6.0` with Postgres and Redis.

```sh
tests/integration/run.sh
```

The script starts Docker Compose, waits for NetBox, seeds minimal fixture data through the REST API, and runs CLI smoke checks against a real NetBox 4.6.0:

- env-only auth (`NETBOX_URL` / `NETBOX_TOKEN`)
- `dcim sites` list/get
- `dcim devices` create/update/list/delete with `--query` and `--field`
- `dcim interfaces` create, get by composite `device:interface` address, update with VLAN name resolution, `trace`, delete
- `ipam vlans create` through the generic dispatcher
- not-found lookup returns the structured error envelope and exits non-zero
