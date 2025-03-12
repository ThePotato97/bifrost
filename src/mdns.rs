use std::net::Ipv4Addr;

use mac_address::MacAddress;
use mdns_sd::{ServiceDaemon, ServiceInfo};

use crate::error::ApiResult;

pub fn register_mdns(mac: MacAddress, ip: Ipv4Addr) -> ApiResult<ServiceDaemon> {
    /* Create a new mDNS daemon. */
    let mdns = ServiceDaemon::new()?;
    let service_type = "_hue._tcp.local.";

    let m = mac.bytes();
    let instance_name = format!(
        "bifrost-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        m[0], m[1], m[2], m[3], m[4], m[5]
    );

    let service_hostname = format!("{instance_name}.{service_type}");
    let service_addr = ip.to_string();
    let service_port = 80;

    let properties = [
        ("modelid", hue::HUE_BRIDGE_V2_MODEL_ID),
        ("bridgeid", &hue::bridge_id(mac)),
    ];

    let service_info = ServiceInfo::new(
        service_type,
        &instance_name,
        &service_hostname,
        service_addr,
        service_port,
        &properties[..],
    )?;

    mdns.register(service_info)?;

    log::info!("Registered service {}.{}", &instance_name, &service_type);

    Ok(mdns)
}
