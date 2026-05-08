#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
COMPOSE_FILE="$SCRIPT_DIR/docker-compose.yml"
NETBOX_URL="${NETBOX_URL:-http://127.0.0.1:8000}"
NETBOX_USERNAME="${NETBOX_USERNAME:-admin}"
NETBOX_PASSWORD="${NETBOX_PASSWORD:-admin}"
export NETBOX_VERSION="${NETBOX_VERSION:-v4.5.10}"
# Pin Cargo's target directory so the script's `target/debug/nbx` invocations
# resolve regardless of any global `~/.cargo/config.toml` `build.target-dir`.
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-target}"

echo "Running integration suite against NetBox ${NETBOX_VERSION}"

cleanup() {
  docker compose -f "$COMPOSE_FILE" down -v --remove-orphans >/dev/null
}

trap cleanup EXIT

docker compose -f "$COMPOSE_FILE" up -d

for _ in $(seq 1 90); do
  if curl -fsS "$NETBOX_URL/login/" >/dev/null; then
    break
  fi
  sleep 5
done

cargo build

provisioned_token_response="$(
  curl -fsS -X POST \
    -H "Content-Type: application/json" \
    "$NETBOX_URL/api/users/tokens/provision/" \
    --data "{\"username\":\"$NETBOX_USERNAME\",\"password\":\"$NETBOX_PASSWORD\",\"version\":2,\"write_enabled\":true,\"description\":\"nbx integration\"}"
)"
NETBOX_TOKEN="$(
  jq -r 'if (.key | startswith("nbt_")) then "\(.key).\(.token)" else "nbt_\(.key).\(.token)" end' \
    <<<"$provisioned_token_response"
)"
if [[ -z "$NETBOX_TOKEN" || "$NETBOX_TOKEN" == "null" ]]; then
  echo "failed to provision NetBox API token" >&2
  exit 1
fi

auth_header() {
  if [[ "$NETBOX_TOKEN" == nbt_*.* ]]; then
    printf 'Bearer %s' "$NETBOX_TOKEN"
  else
    printf 'Token %s' "$NETBOX_TOKEN"
  fi
}

api() {
  local method="$1"
  local path="$2"
  local data="${3:-}"
  if [[ -n "$data" ]]; then
    curl -fsS -X "$method" \
      -H "Authorization: $(auth_header)" \
      -H "Content-Type: application/json" \
      "$NETBOX_URL$path" \
      --data "$data" >/dev/null
  else
    curl -fsS -X "$method" \
      -H "Authorization: $(auth_header)" \
      "$NETBOX_URL$path" >/dev/null
  fi
}

api POST /api/dcim/sites/ '{"name":"DC1","slug":"dc1"}' || true
api POST /api/dcim/manufacturers/ '{"name":"Acme","slug":"acme"}' || true
api POST /api/dcim/device-types/ '{"manufacturer":{"name":"Acme","slug":"acme"},"model":"Server","slug":"server"}' || true
api POST /api/dcim/device-roles/ '{"name":"Server","slug":"server","color":"3366ff"}' || true
api POST /api/extras/tags/ '{"name":"integration","slug":"integration","color":"3366ff"}' || true
api POST /api/extras/tags/ '{"name":"smoke","slug":"smoke","color":"33aa55"}' || true

export NETBOX_URL NETBOX_TOKEN

target/debug/nbx status --output json | jq -e '.schemaVersion == 1 and .data."netbox-version" != null'
target/debug/nbx authentication-check --output json | jq -e '.schemaVersion == 1'

target/debug/nbx dcim sites list --output json --limit 1 | jq -e '.schemaVersion == 1'
target/debug/nbx sites get dc1 --output json | jq -e '.data.slug == "dc1"'

# Generated typed sites create: schema-derived flags, enum status.
target/debug/nbx dcim sites create --name DC2 --slug dc2 --status active --description 'second site' --output json | jq -e '.data.slug == "dc2" and .data.status.value == "active"'
target/debug/nbx dcim sites delete dc2 --confirm --output json | jq -e '.schemaVersion == 1'

# Generated reference resources used by agent bootstrapping.
target/debug/nbx dcim manufacturers create --name Globex --slug globex --output json | jq -e '.data.slug == "globex"'
target/debug/nbx dcim manufacturers delete globex --confirm --output json | jq -e '.schemaVersion == 1'
target/debug/nbx dcim device-roles create --name Router --slug router --color ff0000 --output json | jq -e '.data.slug == "router"'
target/debug/nbx dcim device-roles delete router --confirm --output json | jq -e '.schemaVersion == 1'
target/debug/nbx extras tags create --name cli-tag --slug cli-tag --color 3366ff --output json | jq -e '.data.slug == "cli-tag"'
target/debug/nbx extras tags delete cli-tag --confirm --output json | jq -e '.schemaVersion == 1'
target/debug/nbx tenancy tenant-groups create --name Customers --slug customers --output json | jq -e '.data.slug == "customers"'
target/debug/nbx tenancy tenants create --name Customer1 --slug customer1 --group customers --output json | jq -e '.data.slug == "customer1" and .data.group.slug == "customers"'
target/debug/nbx tenancy tenants delete customer1 --confirm --output json | jq -e '.schemaVersion == 1'
target/debug/nbx tenancy tenant-groups delete customers --confirm --output json | jq -e '.schemaVersion == 1'

target/debug/nbx dcim devices create \
  --name srv-integration-01 --site dc1 --device-type server --role server \
  --tags integration,smoke --output json | jq -e '.data.name == "srv-integration-01" and (.data.tags | length) == 2'
target/debug/nbx devices update srv-integration-01 --status active --output json | jq -e '.data.status.value == "active"'
target/debug/nbx devices list --query 'name=srv-integration-01' --field name,status --output json | jq -e '.results[0].name == "srv-integration-01"'

target/debug/nbx dcim interfaces create \
  --device srv-integration-01 --name Ethernet1 --type 1000base-t \
  --output json | jq -e '.schemaVersion == 1'
target/debug/nbx interfaces get srv-integration-01:Ethernet1 --output json | jq -e '.data.name == "Ethernet1"'

# Generated typed VLAN create: --vid + --name + --status enum.
target/debug/nbx ipam vlans create --vid 100 --name mgmt --status active --output json | jq -e '.data.vid == 100 and .data.status.value == "active"'
target/debug/nbx interfaces update srv-integration-01:Ethernet1 --mode access --untagged-vlan mgmt --output json | jq -e '.data.untagged_vlan.name == "mgmt"'
target/debug/nbx interfaces trace srv-integration-01:Ethernet1 --output json | jq -e '.schemaVersion == 1'
target/debug/nbx interfaces delete srv-integration-01:Ethernet1 --confirm --output json | jq -e '.schemaVersion == 1'

target/debug/nbx devices delete srv-integration-01 --confirm --output json | jq -e '.schemaVersion == 1'

target/debug/nbx ipam vlans create --vid 999 --name dry-run-only --dry-run --output json | jq -e '.data.dryRun == true'

vlan_bulk_id="$(target/debug/nbx ipam vlans create --vid 201 --name bulk-1 --output json | jq -r '.data.id')"
target/debug/nbx ipam vlans create --vid 202 --name bulk-2 --output json | jq -e '.schemaVersion == 1'
target/debug/nbx ipam vlans bulk-update --data "[{\"id\":$vlan_bulk_id,\"description\":\"bulk-updated\"}]" --output json | jq -e '.schemaVersion == 1'

# multi-page --page-all: with at least 3 vlans (mgmt, bulk-1, bulk-2) and --limit 1 we walk 3+ pages
page_all_lines="$(target/debug/nbx ipam vlans list --limit 1 --page-all | wc -l | tr -d ' ')"
[ "$page_all_lines" -ge 3 ]

target/debug/nbx ipam vlans bulk-delete --data "[{\"id\":$vlan_bulk_id}]" --confirm --output json | jq -e '.schemaVersion == 1'

# Generated typed prefixes create + CIDR identifier round-trip.
target/debug/nbx ipam prefixes create --prefix 192.0.2.0/24 --status active --output json | jq -e '.data.prefix == "192.0.2.0/24" and .data.status.value == "active"'
target/debug/nbx ipam prefixes get 192.0.2.0/24 --output json | jq -e '.data.prefix == "192.0.2.0/24"'
target/debug/nbx ipam prefixes available-ips 192.0.2.0/24 --output json | jq -e '.schemaVersion == 1 and (.results | length) > 0'
target/debug/nbx dcim cables create --data '{"label":"dry-run-cable"}' --dry-run --output json | jq -e '.data.dryRun == true'
target/debug/nbx ipam prefixes delete 192.0.2.0/24 --confirm --output json | jq -e '.schemaVersion == 1'

# Generated typed IP address create with role + status enums.
ip_id="$(target/debug/nbx ipam ip-addresses create --address 192.0.2.10/24 --status active --role loopback --dns-name lo0 --output json | jq -r '.data.id')"
target/debug/nbx ipam ip-addresses get "$ip_id" --output json | jq -e '.data.address == "192.0.2.10/24" and .data.role.value == "loopback"'
target/debug/nbx ipam ip-addresses delete "$ip_id" --confirm --output json | jq -e '.schemaVersion == 1'

# Real-NetBox 422: VLAN vid 4095 (NetBox accepts 1-4094) must surface as exit 4
# with field-level detail preserved under .error.detail.response.
set +e
validation_error_stderr="$(target/debug/nbx ipam vlans create --vid 4095 --name bad-vid --output json 2>&1 1>/dev/null)"
validation_error_status=$?
set -e
[ "$validation_error_status" -eq 4 ] || {
  echo "expected validation_error (exit 4), got $validation_error_status" >&2
  exit 1
}
jq -e '.error.code == "validation_error"' <<<"$validation_error_stderr" >/dev/null
jq -e '.error.detail.status == 400 or .error.detail.status == 422' <<<"$validation_error_stderr" >/dev/null
jq -e '.error.detail.response.vid != null' <<<"$validation_error_stderr" >/dev/null

if target/debug/nbx devices get does-not-exist --output json; then
  echo "expected not-found lookup to fail" >&2
  exit 1
fi

set +e
NETBOX_TOKEN=bogus target/debug/nbx devices list --output json >/dev/null 2>&1
auth_failed_status=$?
set -e
[ "$auth_failed_status" -eq 3 ] || {
  echo "expected auth_failed (exit 3), got $auth_failed_status" >&2
  exit 1
}
