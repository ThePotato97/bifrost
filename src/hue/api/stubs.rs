use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::hue::api::{DeviceArchetype, ResourceLink, SceneMetadata};
use crate::hue::{best_guess_timezone, date_format};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bridge {
    pub bridge_id: String,
    pub owner: ResourceLink,
    pub time_zone: TimeZone,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BridgeHome {
    pub children: Vec<ResourceLink>,
    pub services: Vec<ResourceLink>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Button {
    pub owner: ResourceLink,
    pub metadata: ButtonMetadata,
    pub button: ButtonData,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ButtonMetadata {
    pub control_id: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ButtonData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub button_report: Option<ButtonReport>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_event: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_interval: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_values: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ButtonReport {
    #[serde(with = "date_format::utc_ms")]
    pub updated: DateTime<Utc>,
    pub event: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DollarRef {
    #[serde(rename = "$ref", skip_serializing_if = "Option::is_none")]
    pub dref: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DevicePower {
    pub owner: ResourceLink,
    pub power_state: Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeviceSoftwareUpdate {
    pub owner: ResourceLink,
    pub state: Value,
    pub problems: Vec<Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BehaviorScript {
    pub configuration_schema: DollarRef,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_number_instances: Option<u32>,
    pub metadata: Value,
    pub state_schema: DollarRef,
    pub supported_features: Vec<String>,
    pub trigger_schema: DollarRef,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BehaviorInstance {
    pub configuration: Value,
    #[serde(default)]
    pub dependees: Vec<Value>,
    pub enabled: bool,
    pub last_error: String,
    pub metadata: BehaviorInstanceMetadata,
    pub script_id: Uuid,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migrated_from: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BehaviorInstanceMetadata {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Entertainment {
    pub equalizer: bool,
    pub owner: ResourceLink,
    pub proxy: bool,
    pub renderer: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_streams: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renderer_reference: Option<ResourceLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segments: Option<EntertainmentSegments>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EntertainmentConfiguration {
    pub name: String,
    pub configuration_type: String,
    pub metadata: EntertainmentConfigurationMetadata,
    pub status: String,
    pub stream_proxy: EntertainmentConfigurationStreamProxy,
    pub locations: EntertainmentConfigurationLocations,
    pub light_services: Vec<ResourceLink>,
    pub channels: Vec<EntertainmentConfigurationChannels>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_streamer: Option<ResourceLink>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EntertainmentConfigurationMetadata {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EntertainmentConfigurationStreamProxy {
    pub mode: String,
    pub node: ResourceLink,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EntertainmentConfigurationLocations {
    pub service_locations: Vec<EntertainmentConfigurationServiceLocations>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EntertainmentConfigurationServiceLocations {
    pub equalization_factor: f64,
    pub position: EntertainmentConfigurationPosition,
    pub positions: Vec<EntertainmentConfigurationPosition>,
    pub service: ResourceLink,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EntertainmentConfigurationChannels {
    pub channel_id: u32,
    pub position: EntertainmentConfigurationPosition,
    pub members: Vec<EntertainmentConfigurationStreamMembers>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EntertainmentConfigurationPosition {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EntertainmentConfigurationStreamMembers {
    pub service: ResourceLink,
    pub index: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EntertainmentSegments {
    pub configurable: bool,
    pub max_segments: u32,
    pub segments: Vec<EntertainmentSegment>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EntertainmentSegment {
    pub length: u32,
    pub start: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GeofenceClient {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Geolocation {
    pub is_configured: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sun_today: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupedMotion {
    pub owner: ResourceLink,
    pub enabled: bool,
    pub motion: Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupedLightLevel {
    pub owner: ResourceLink,
    pub enabled: bool,
    #[serde(default)]
    pub light: Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Homekit {
    pub status: String,
    pub status_values: Vec<String>,
}

impl Default for Homekit {
    fn default() -> Self {
        Self {
            status: "unpaired".to_string(),
            status_values: vec![
                "pairing".to_string(),
                "paired".to_string(),
                "unpaired".to_string(),
            ],
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LightLevel {
    pub enabled: bool,
    pub light: Value,
    pub owner: ResourceLink,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Matter {
    pub has_qr_code: bool,
    pub max_fabrics: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Motion {
    pub enabled: bool,
    pub owner: ResourceLink,
    pub motion: Value,
    #[serde(default)]
    pub sensitivity: Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PrivateGroup {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PublicImage {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RelativeRotary {
    pub owner: ResourceLink,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_rotary: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotary_report: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SmartScene {
    /* active_timeslot: { */
    /*     timeslot_id: 3, */
    /*     weekday: monday */
    /* }, */
    pub active_timeslot: Value,
    pub group: ResourceLink,
    pub metadata: SceneMetadata,
    pub state: String,
    pub transition_duration: u32,
    pub week_timeslots: Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Taurus {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ZigbeeConnectivityStatus {
    Connected,
    ConnectivityIssue,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ZigbeeConnectivity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_pan_id: Option<String>,
    pub mac_address: String,
    pub owner: ResourceLink,
    pub status: ZigbeeConnectivityStatus,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ZigbeeDeviceDiscovery {
    pub owner: ResourceLink,
    pub status: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Value::is_null")]
    pub action: Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Zone {
    pub metadata: Metadata,
    pub children: Vec<ResourceLink>,
    #[serde(default)]
    pub services: Vec<ResourceLink>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Temperature {
    pub enabled: bool,
    pub owner: ResourceLink,
    pub temperature: Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TimeZone {
    pub time_zone: String,
}

impl TimeZone {
    #[must_use]
    pub fn best_guess() -> Self {
        Self {
            time_zone: best_guess_timezone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Metadata {
    pub name: String,
    pub archetype: DeviceArchetype,
}

impl Metadata {
    #[must_use]
    pub fn new(archetype: DeviceArchetype, name: &str) -> Self {
        Self {
            archetype,
            name: name.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct MetadataUpdate {
    pub name: Option<String>,
    pub archetype: Option<DeviceArchetype>,
}
