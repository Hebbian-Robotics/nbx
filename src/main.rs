use std::process::ExitCode;

use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{Shell, generate};

mod client;
mod commands;
mod config;
mod envelope;
mod error;
mod generated;
mod output;
#[cfg(test)]
mod output_snapshots;
mod projection;

use commands::{
    ConfigCommand, DcimCommand, DeviceAction, ExtrasCommand, GlobalOptions, InterfaceAction,
    IpamCommand, PrefixAction, RawArgs, TenancyCommand, run_authentication_check_command,
    run_config_command, run_device_action, run_interface_action, run_prefix_action,
    run_raw_command, run_status_command,
};
use error::{NbxError, NbxResult};
use generated::resources::{
    aggregates::{AggregateAction, run_aggregate_action},
    cables::{CableAction, run_cable_action},
    device_roles::{DeviceRoleAction, run_device_role_action},
    device_types::{DeviceTypeAction, run_device_type_action},
    inventory_items::{InventoryItemAction, run_inventory_item_action},
    ip_addresses::{IpAddressAction, run_ip_address_action},
    locations::{LocationAction, run_location_action},
    manufacturers::{ManufacturerAction, run_manufacturer_action},
    platforms::{PlatformAction, run_platform_action},
    racks::{RackAction, run_rack_action},
    regions::{RegionAction, run_region_action},
    rirs::{RirAction, run_rir_action},
    roles::{RoleAction, run_role_action},
    site_groups::{SiteGroupAction, run_site_group_action},
    sites::{SiteAction, run_site_action},
    tags::{TagAction, run_tag_action},
    tenant_groups::{TenantGroupAction, run_tenant_group_action},
    tenants::{TenantAction, run_tenant_action},
    vlan_groups::{VlanGroupAction, run_vlan_group_action},
    vlans::{VlanAction, run_vlan_action},
    vrfs::{VrfAction, run_vrf_action},
};

#[derive(Debug, Parser)]
#[command(
    name = "nbx",
    version,
    about = "A NetBox CLI built for humans and AI agents."
)]
struct CommandLineArguments {
    #[command(flatten)]
    global_options: GlobalOptions,

    #[command(subcommand)]
    command: Option<NbxCommand>,
}

#[derive(Debug, Subcommand)]
enum NbxCommand {
    /// Print the nbx version.
    Version,
    /// Check `NetBox` API status.
    Status,
    /// Check `NetBox` authentication.
    #[command(name = "authentication-check")]
    AuthenticationCheck,
    /// Generate shell completions.
    Completions { shell: Shell },
    /// Make a raw `NetBox` API request.
    Raw(RawArgs),
    /// Manage nbx configuration.
    Config {
        #[command(subcommand)]
        command: ConfigCommand,
    },
    /// DCIM resources.
    Dcim {
        #[command(subcommand)]
        command: DcimCommand,
    },
    /// IPAM resources.
    Ipam {
        #[command(subcommand)]
        command: IpamCommand,
    },
    /// Extras resources.
    Extras {
        #[command(subcommand)]
        command: ExtrasCommand,
    },
    /// Tenancy resources.
    Tenancy {
        #[command(subcommand)]
        command: TenancyCommand,
    },
    /// Alias for `nbx dcim cables`.
    Cables {
        #[command(subcommand)]
        action: CableAction,
    },
    /// Alias for `nbx dcim device-roles`.
    #[command(name = "device-roles")]
    DeviceRoles {
        #[command(subcommand)]
        action: DeviceRoleAction,
    },
    /// Alias for `nbx dcim device-types`.
    #[command(name = "device-types")]
    DeviceTypes {
        #[command(subcommand)]
        action: DeviceTypeAction,
    },
    /// Alias for `nbx dcim devices`.
    Devices {
        #[command(subcommand)]
        action: DeviceAction,
    },
    /// Alias for `nbx dcim interfaces`.
    Interfaces {
        #[command(subcommand)]
        action: InterfaceAction,
    },
    /// Alias for `nbx dcim sites`.
    Sites {
        #[command(subcommand)]
        action: SiteAction,
    },
    /// Alias for `nbx dcim racks`.
    Racks {
        #[command(subcommand)]
        action: RackAction,
    },
    /// Alias for `nbx dcim locations`.
    Locations {
        #[command(subcommand)]
        action: LocationAction,
    },
    /// Alias for `nbx dcim manufacturers`.
    Manufacturers {
        #[command(subcommand)]
        action: ManufacturerAction,
    },
    /// Alias for `nbx dcim platforms`.
    Platforms {
        #[command(subcommand)]
        action: PlatformAction,
    },
    /// Alias for `nbx dcim regions`.
    Regions {
        #[command(subcommand)]
        action: RegionAction,
    },
    /// Alias for `nbx dcim site-groups`.
    #[command(name = "site-groups")]
    SiteGroups {
        #[command(subcommand)]
        action: SiteGroupAction,
    },
    /// Alias for `nbx dcim inventory-items`.
    #[command(name = "inventory-items")]
    InventoryItems {
        #[command(subcommand)]
        action: InventoryItemAction,
    },
    /// Alias for `nbx ipam aggregates`.
    Aggregates {
        #[command(subcommand)]
        action: AggregateAction,
    },
    /// Alias for `nbx ipam ip-addresses`.
    #[command(name = "ip-addresses")]
    IpAddresses {
        #[command(subcommand)]
        action: IpAddressAction,
    },
    /// Alias for `nbx ipam prefixes`.
    Prefixes {
        #[command(subcommand)]
        action: PrefixAction,
    },
    /// Alias for `nbx ipam rirs`.
    Rirs {
        #[command(subcommand)]
        action: RirAction,
    },
    /// Alias for `nbx ipam roles`.
    Roles {
        #[command(subcommand)]
        action: RoleAction,
    },
    /// Alias for `nbx ipam vlan-groups`.
    #[command(name = "vlan-groups")]
    VlanGroups {
        #[command(subcommand)]
        action: VlanGroupAction,
    },
    /// Alias for `nbx ipam vlans`.
    Vlans {
        #[command(subcommand)]
        action: VlanAction,
    },
    /// Alias for `nbx ipam vrfs`.
    Vrfs {
        #[command(subcommand)]
        action: VrfAction,
    },
    /// Alias for `nbx extras tags`.
    Tags {
        #[command(subcommand)]
        action: TagAction,
    },
    /// Alias for `nbx tenancy tenant-groups`.
    #[command(name = "tenant-groups")]
    TenantGroups {
        #[command(subcommand)]
        action: TenantGroupAction,
    },
    /// Alias for `nbx tenancy tenants`.
    Tenants {
        #[command(subcommand)]
        action: TenantAction,
    },
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> ExitCode {
    let command_line_arguments = CommandLineArguments::parse();

    if command_line_arguments.global_options.verbose {
        initialize_tracing();
    }

    match run(command_line_arguments).await {
        Ok(exit_code) => exit_code,
        Err(error) => {
            let error_json = envelope::error_envelope(&error);
            eprintln!(
                "{}",
                serde_json::to_string_pretty(&error_json)
                    .unwrap_or_else(|_| "{\"error\":{\"code\":\"general\"}}".to_owned())
            );
            error.exit_code()
        }
    }
}

async fn run(command_line_arguments: CommandLineArguments) -> NbxResult<ExitCode> {
    let global_options = &command_line_arguments.global_options;
    match command_line_arguments.command {
        Some(NbxCommand::Version) => {
            println!("{}", env!("CARGO_PKG_VERSION"));
        }
        Some(NbxCommand::Status) => {
            run_status_command(global_options).await?;
        }
        Some(NbxCommand::AuthenticationCheck) => {
            run_authentication_check_command(global_options).await?;
        }
        Some(NbxCommand::Completions { shell }) => {
            let mut command = CommandLineArguments::command();
            let command_name = command.get_name().to_owned();
            generate(shell, &mut command, command_name, &mut std::io::stdout());
        }
        Some(NbxCommand::Raw(arguments)) => {
            run_raw_command(arguments, global_options).await?;
        }
        Some(NbxCommand::Config { command }) => {
            run_config_command(command).await?;
        }
        Some(NbxCommand::Dcim { command }) => match command {
            DcimCommand::Cables { action } => run_cable_action(action, global_options).await?,
            DcimCommand::DeviceRoles { action } => {
                run_device_role_action(action, global_options).await?;
            }
            DcimCommand::DeviceTypes { action } => {
                run_device_type_action(action, global_options).await?;
            }
            DcimCommand::Devices { action } => run_device_action(action, global_options).await?,
            DcimCommand::Interfaces { action } => {
                run_interface_action(action, global_options).await?;
            }
            DcimCommand::Sites { action } => run_site_action(action, global_options).await?,
            DcimCommand::Racks { action } => run_rack_action(action, global_options).await?,
            DcimCommand::InventoryItems { action } => {
                run_inventory_item_action(action, global_options).await?;
            }
            DcimCommand::Locations { action } => {
                run_location_action(action, global_options).await?;
            }
            DcimCommand::Manufacturers { action } => {
                run_manufacturer_action(action, global_options).await?;
            }
            DcimCommand::Platforms { action } => {
                run_platform_action(action, global_options).await?;
            }
            DcimCommand::Regions { action } => run_region_action(action, global_options).await?,
            DcimCommand::SiteGroups { action } => {
                run_site_group_action(action, global_options).await?;
            }
        },
        Some(NbxCommand::Ipam { command }) => match command {
            IpamCommand::Aggregates { action } => {
                run_aggregate_action(action, global_options).await?;
            }
            IpamCommand::IpAddresses { action } => {
                run_ip_address_action(action, global_options).await?;
            }
            IpamCommand::Prefixes { action } => run_prefix_action(action, global_options).await?,
            IpamCommand::Rirs { action } => run_rir_action(action, global_options).await?,
            IpamCommand::Roles { action } => run_role_action(action, global_options).await?,
            IpamCommand::VlanGroups { action } => {
                run_vlan_group_action(action, global_options).await?;
            }
            IpamCommand::Vlans { action } => run_vlan_action(action, global_options).await?,
            IpamCommand::Vrfs { action } => run_vrf_action(action, global_options).await?,
        },
        Some(NbxCommand::Extras { command }) => match command {
            ExtrasCommand::Tags { action } => run_tag_action(action, global_options).await?,
        },
        Some(NbxCommand::Tenancy { command }) => match command {
            TenancyCommand::TenantGroups { action } => {
                run_tenant_group_action(action, global_options).await?;
            }
            TenancyCommand::Tenants { action } => run_tenant_action(action, global_options).await?,
        },
        Some(NbxCommand::Cables { action }) => run_cable_action(action, global_options).await?,
        Some(NbxCommand::DeviceRoles { action }) => {
            run_device_role_action(action, global_options).await?;
        }
        Some(NbxCommand::DeviceTypes { action }) => {
            run_device_type_action(action, global_options).await?;
        }
        Some(NbxCommand::Devices { action }) => run_device_action(action, global_options).await?,
        Some(NbxCommand::Interfaces { action }) => {
            run_interface_action(action, global_options).await?;
        }
        Some(NbxCommand::Sites { action }) => run_site_action(action, global_options).await?,
        Some(NbxCommand::Racks { action }) => run_rack_action(action, global_options).await?,
        Some(NbxCommand::Locations { action }) => {
            run_location_action(action, global_options).await?;
        }
        Some(NbxCommand::Manufacturers { action }) => {
            run_manufacturer_action(action, global_options).await?;
        }
        Some(NbxCommand::Platforms { action }) => {
            run_platform_action(action, global_options).await?;
        }
        Some(NbxCommand::Regions { action }) => run_region_action(action, global_options).await?,
        Some(NbxCommand::SiteGroups { action }) => {
            run_site_group_action(action, global_options).await?;
        }
        Some(NbxCommand::InventoryItems { action }) => {
            run_inventory_item_action(action, global_options).await?;
        }
        Some(NbxCommand::Aggregates { action }) => {
            run_aggregate_action(action, global_options).await?;
        }
        Some(NbxCommand::IpAddresses { action }) => {
            run_ip_address_action(action, global_options).await?;
        }
        Some(NbxCommand::Prefixes { action }) => run_prefix_action(action, global_options).await?,
        Some(NbxCommand::Rirs { action }) => run_rir_action(action, global_options).await?,
        Some(NbxCommand::Roles { action }) => run_role_action(action, global_options).await?,
        Some(NbxCommand::VlanGroups { action }) => {
            run_vlan_group_action(action, global_options).await?;
        }
        Some(NbxCommand::Vlans { action }) => run_vlan_action(action, global_options).await?,
        Some(NbxCommand::Vrfs { action }) => run_vrf_action(action, global_options).await?,
        Some(NbxCommand::Tags { action }) => run_tag_action(action, global_options).await?,
        Some(NbxCommand::TenantGroups { action }) => {
            run_tenant_group_action(action, global_options).await?;
        }
        Some(NbxCommand::Tenants { action }) => run_tenant_action(action, global_options).await?,
        None => {
            CommandLineArguments::command()
                .print_help()
                .map_err(|error| NbxError::general(format!("failed to print help: {error}")))?;
            println!();
        }
    }

    Ok(ExitCode::SUCCESS)
}

fn initialize_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("nbx=debug")),
        )
        .try_init();
}
