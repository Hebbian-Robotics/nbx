// AUTO-GENERATED. Run `cargo run -p nbx-codegen -- schema/netbox-4.6.0.json src/generated/resources/`.

#[rustfmt::skip]
pub mod sites;
#[rustfmt::skip]
pub mod site_groups;
#[rustfmt::skip]
pub mod regions;
#[rustfmt::skip]
pub mod locations;
#[rustfmt::skip]
pub mod manufacturers;
#[rustfmt::skip]
pub mod device_types;
#[rustfmt::skip]
pub mod device_roles;
#[rustfmt::skip]
pub mod platforms;
#[rustfmt::skip]
pub mod racks;
#[rustfmt::skip]
pub mod inventory_items;
#[rustfmt::skip]
pub mod ip_addresses;
#[rustfmt::skip]
pub mod prefixes;
#[rustfmt::skip]
pub mod vrfs;
#[rustfmt::skip]
pub mod roles;
#[rustfmt::skip]
pub mod vlan_groups;
#[rustfmt::skip]
pub mod rirs;
#[rustfmt::skip]
pub mod aggregates;
#[rustfmt::skip]
pub mod vlans;
#[rustfmt::skip]
pub mod tags;
#[rustfmt::skip]
pub mod tenant_groups;
#[rustfmt::skip]
pub mod tenants;
#[rustfmt::skip]
pub mod cables;
#[rustfmt::skip]
pub mod devices;
#[rustfmt::skip]
pub mod interfaces;

use crate::commands::ResourceSpec;

pub fn all_resource_specs() -> Vec<ResourceSpec> {
    vec![
        sites::RESOURCE,
        site_groups::RESOURCE,
        regions::RESOURCE,
        locations::RESOURCE,
        manufacturers::RESOURCE,
        device_types::RESOURCE,
        device_roles::RESOURCE,
        platforms::RESOURCE,
        racks::RESOURCE,
        inventory_items::RESOURCE,
        ip_addresses::RESOURCE,
        prefixes::RESOURCE,
        vrfs::RESOURCE,
        roles::RESOURCE,
        vlan_groups::RESOURCE,
        rirs::RESOURCE,
        aggregates::RESOURCE,
        vlans::RESOURCE,
        tags::RESOURCE,
        tenant_groups::RESOURCE,
        tenants::RESOURCE,
        cables::RESOURCE,
        devices::RESOURCE,
        interfaces::RESOURCE,
    ]
}
