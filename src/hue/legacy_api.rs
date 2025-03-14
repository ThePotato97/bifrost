use std::{collections::HashMap, net::Ipv4Addr};

use chrono::{DateTime, Local, Utc};
use mac_address::MacAddress;
use serde::{Deserialize, Serialize, Serializer};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::error::ApiResult;
use crate::hue::version::SwVersion;
use crate::hue::{self, api, best_guess_timezone};
use crate::resource::Resources;

use super::date_format;

#[derive(Debug, Serialize, Deserialize)]
pub struct HueError {
    #[serde(rename = "type")]
    typ: u32,
    address: String,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HueResult<T> {
    Success(T),
    Error(HueError),
}

pub fn serialize_lower_case_mac<S>(mac: &MacAddress, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let m = mac.bytes();
    let addr = format!(
        "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
        m[0], m[1], m[2], m[3], m[4], m[5]
    );
    serializer.serialize_str(&addr)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiShortConfig {
    pub apiversion: String,
    pub bridgeid: String,
    pub datastoreversion: String,
    pub factorynew: bool,
    #[serde(serialize_with = "serialize_lower_case_mac")]
    pub mac: MacAddress,
    pub modelid: String,
    pub name: String,
    pub replacesbridgeid: Option<String>,
    pub starterkitid: String,
    pub swversion: String,
}

impl Default for ApiShortConfig {
    fn default() -> Self {
        Self {
            apiversion: hue::HUE_BRIDGE_V2_DEFAULT_APIVERSION.to_string(),
            bridgeid: "0000000000000000".to_string(),
            datastoreversion: "163".to_string(),
            factorynew: false,
            mac: MacAddress::default(),
            modelid: hue::HUE_BRIDGE_V2_MODEL_ID.to_string(),
            name: "Bifrost Bridge".to_string(),
            replacesbridgeid: None,
            starterkitid: String::new(),
            swversion: hue::HUE_BRIDGE_V2_DEFAULT_SWVERSION.to_string(),
        }
    }
}

impl ApiShortConfig {
    #[must_use]
    pub fn from_mac_and_version(mac: MacAddress, version: &SwVersion) -> Self {
        Self {
            bridgeid: hue::bridge_id(mac),
            apiversion: version.get_legacy_apiversion(),
            swversion: version.get_legacy_swversion(),
            mac,
            ..Self::default()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ApiResourceType {
    Config,
    Groups,
    Lights,
    Resourcelinks,
    Rules,
    Scenes,
    Schedules,
    Sensors,
    Capabilities,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewUser {
    devicetype: String,
    generateclientkey: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewUserReply {
    pub username: Uuid,
    pub clientkey: Uuid,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConnectionState {
    Connected,
    Disconnected,
}

impl Default for ConnectionState {
    fn default() -> Self {
        Self::Disconnected
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiInternetServices {
    pub internet: ConnectionState,
    pub remoteaccess: ConnectionState,
    pub swupdate: ConnectionState,
    pub time: ConnectionState,
}

impl Default for ApiInternetServices {
    fn default() -> Self {
        Self {
            internet: ConnectionState::Connected,
            remoteaccess: ConnectionState::Connected,
            swupdate: ConnectionState::Connected,
            time: ConnectionState::Connected,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PortalState {
    communication: ConnectionState,
    incoming: bool,
    outgoing: bool,
    signedon: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiBackup {
    pub errorcode: u32,
    pub status: String,
}

impl Default for ApiBackup {
    fn default() -> Self {
        Self {
            errorcode: 0,
            status: "idle".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DeviceTypes {
    bridge: bool,
    lights: Vec<Value>,
    sensors: Vec<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SwUpdate {
    #[serde(with = "date_format::legacy_utc")]
    lastinstall: DateTime<Utc>,
    state: SwUpdateState,
}

impl Default for SwUpdate {
    fn default() -> Self {
        Self {
            lastinstall: Utc::now(),
            state: SwUpdateState::NoUpdates,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SwUpdateState {
    NoUpdates,
    Transferring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoftwareUpdate2 {
    autoinstall: Value,
    bridge: SwUpdate,
    checkforupdate: bool,
    #[serde(with = "date_format::legacy_utc")]
    lastchange: DateTime<Utc>,
    state: SwUpdateState,
}

impl SoftwareUpdate2 {
    #[allow(clippy::new_without_default)]
    #[must_use]
    pub fn new() -> Self {
        Self {
            autoinstall: json!({ "on": true, "updatetime": "T14:00:00" }),
            bridge: SwUpdate {
                lastinstall: Utc::now(),
                state: SwUpdateState::NoUpdates,
            },
            checkforupdate: false,
            lastchange: Utc::now(),
            state: SwUpdateState::NoUpdates,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Whitelist {
    #[serde(with = "date_format::legacy_utc", rename = "create date")]
    pub create_date: DateTime<Utc>,
    #[serde(with = "date_format::legacy_utc", rename = "last use date")]
    pub last_use_date: DateTime<Utc>,
    pub name: String,
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiConfig {
    pub analyticsconsent: bool,
    pub backup: ApiBackup,
    #[serde(flatten)]
    pub short_config: ApiShortConfig,
    pub dhcp: bool,
    pub internetservices: ApiInternetServices,
    pub linkbutton: bool,
    pub portalconnection: ConnectionState,
    pub portalservices: bool,
    pub portalstate: PortalState,
    pub proxyaddress: String,
    pub proxyport: u16,
    pub swupdate2: SoftwareUpdate2,
    pub zigbeechannel: u8,
    pub ipaddress: Ipv4Addr,
    pub netmask: Ipv4Addr,
    pub gateway: Ipv4Addr,
    pub timezone: String,
    #[serde(with = "date_format::legacy_utc", rename = "UTC")]
    pub utc: DateTime<Utc>,
    #[serde(with = "date_format::legacy_local")]
    pub localtime: DateTime<Local>,
    pub whitelist: HashMap<String, Whitelist>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ApiEffect {
    None,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ApiAlert {
    None,
    Select,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiGroupAction {
    on: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    bri: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hue: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sat: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    effect: Option<ApiEffect>,
    #[serde(skip_serializing_if = "Option::is_none")]
    xy: Option<[f64; 2]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ct: Option<u16>,
    alert: ApiAlert,
    #[serde(skip_serializing_if = "Option::is_none")]
    colormode: Option<LightColorMode>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ApiGroupType {
    Entertainment,
    LightGroup,
    Room,
    Zone,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiGroup {
    name: String,
    lights: Vec<String>,
    action: ApiGroupAction,

    #[serde(rename = "type")]
    group_type: ApiGroupType,
    class: String,
    recycle: bool,
    sensors: Vec<Value>,
    state: Value,
    #[serde(skip_serializing_if = "Value::is_null", default)]
    stream: Value,
    #[serde(skip_serializing_if = "Value::is_null", default)]
    locations: Value,
}

impl ApiGroup {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    #[must_use]
    pub fn from_lights_and_room(
        glight: &api::GroupedLight,
        lights: Vec<String>,
        room: api::Room,
    ) -> Self {
        Self {
            name: room.metadata.name,
            lights,
            action: ApiGroupAction {
                on: glight.on.is_some_and(|on| on.on),
                bri: glight.dimming.map(|dim| (dim.brightness * 2.54) as u32),
                hue: None,
                sat: None,
                effect: None,
                xy: None,
                ct: None,
                alert: ApiAlert::None,
                colormode: None,
            },
            class: "Bedroom".to_string(),
            group_type: ApiGroupType::Room,
            recycle: false,
            sensors: vec![],
            state: json!({}),
            stream: Value::Null,
            locations: Value::Null,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiGroupState {
    pub all_on: bool,
    pub any_on: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LightColorMode {
    Ct,
    Xy,
    Hs,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiLightState {
    on: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    bri: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hue: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sat: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    effect: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    xy: Option<[f64; 2]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ct: Option<u16>,
    alert: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    colormode: Option<LightColorMode>,
    mode: String,
    reachable: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiLightStateUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bri: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xy: Option<[f64; 2]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ct: Option<u16>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiGroupUpdate {
    pub scene: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiGroupActionUpdate {
    GroupUpdate(ApiGroupUpdate),
    LightUpdate(ApiLightStateUpdate),
}

impl From<api::SceneAction> for ApiLightStateUpdate {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn from(action: api::SceneAction) -> Self {
        Self {
            on: action.on.map(|on| on.on),
            bri: action.dimming.map(|dim| (dim.brightness * 2.54) as u32),
            xy: action.color.map(|col| col.xy.into()),
            ct: action.color_temperature.map(|ct| ct.mirek),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiLight {
    state: ApiLightState,
    swupdate: SwUpdate,
    #[serde(rename = "type")]
    light_type: String,
    name: String,
    modelid: String,
    manufacturername: String,
    productname: String,
    capabilities: Value,
    config: Value,
    uniqueid: String,
    swversion: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    swconfigid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    productid: Option<String>,
}

impl ApiLight {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    #[must_use]
    pub fn from_dev_and_light(uuid: &Uuid, dev: &api::Device, light: &api::Light) -> Self {
        let colormode = if light.color.is_some() {
            LightColorMode::Xy
        } else {
            LightColorMode::Ct
        };

        let product_data = dev.product_data.clone();

        Self {
            state: ApiLightState {
                on: light.on.on,
                bri: light.dimming.map(|dim| (dim.brightness * 2.54) as u32),
                hue: None,
                sat: None,
                effect: None,
                xy: light.color.clone().map(|col| col.xy.into()),
                ct: light.color_temperature.clone().and_then(|ct| ct.mirek),
                alert: String::new(),
                colormode: Some(colormode),
                mode: "homeautomation".to_string(),
                reachable: true,
            },
            swupdate: SwUpdate::default(),
            name: light.metadata.name.clone(),
            modelid: product_data.product_name,
            manufacturername: product_data.manufacturer_name,
            productname: "Hue color spot".to_string(),
            productid: Some(product_data.model_id),
            capabilities: json!({
                "certified": true,
                "control": {
                    "colorgamut": [
                        [0.6915, 0.3083 ],
                        [0.17,   0.7    ],
                        [0.1532, 0.0475 ],
                    ],
                    "colorgamuttype": "C",
                    "ct": {
                        "max": 500,
                        "min": 153
                    },
                    "maxlumen": 300,
                    "mindimlevel": 200
                },
                "streaming": {
                    "proxy": true,
                    "renderer": true
                }
            }),
            config: json!({
                "archetype": "spotbulb",
                "function": "mixed",
                "direction": "downwards",
                "startup": {
                    "mode": "safety",
                    "configured": true
                }
            }),
            light_type: "Extended color light".to_string(),
            uniqueid: uuid.as_simple().to_string(),
            swversion: product_data.software_version,
            swconfigid: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResourceLink {
    #[serde(rename = "type")]
    pub link_type: String,
    pub name: String,
    pub description: String,
    pub classid: u32,
    pub owner: Uuid,
    pub recycle: bool,
    pub links: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiRule {
    pub name: String,
    pub recycle: bool,
    pub status: String,
    pub conditions: Vec<Value>,
    pub actions: Vec<Value>,
    pub owner: Uuid,
    pub timestriggered: u32,
    #[serde(with = "date_format::legacy_utc")]
    pub created: DateTime<Utc>,
    pub lasttriggered: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ApiSceneType {
    LightScene,
    GroupScene,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ApiSceneVersion {
    V2 = 2,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiSceneAppData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiScene {
    name: String,
    #[serde(rename = "type")]
    scene_type: ApiSceneType,
    lights: Vec<String>,
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    lightstates: HashMap<String, ApiLightStateUpdate>,
    owner: String,
    recycle: bool,
    locked: bool,
    appdata: ApiSceneAppData,
    picture: String,
    #[serde(with = "date_format::legacy_utc")]
    lastupdated: DateTime<Utc>,
    version: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group: Option<String>,
}

impl ApiScene {
    pub fn from_scene(res: &Resources, owner: Uuid, scene: &api::Scene) -> ApiResult<Self> {
        let lights = scene
            .actions
            .iter()
            .map(|sae| res.get_id_v1(sae.target.rid))
            .collect::<ApiResult<_>>()?;

        let lightstates = scene
            .actions
            .iter()
            .map(|sae| {
                Ok((
                    res.get_id_v1(sae.target.rid)?,
                    ApiLightStateUpdate::from(sae.action.clone()),
                ))
            })
            .collect::<ApiResult<_>>()?;

        let room_id = res.get_id_v1_index(scene.group.rid)?;

        Ok(Self {
            name: scene.metadata.name.clone(),
            scene_type: ApiSceneType::GroupScene,
            lights,
            lightstates,
            owner: owner.to_string(),
            recycle: false,
            locked: false,
            /* Some clients (e.g. Hue Essentials) require .appdata */
            appdata: ApiSceneAppData {
                data: Some(format!("xxxxx_r{room_id}")),
                version: Some(1),
            },
            picture: String::new(),
            lastupdated: Utc::now(),
            version: ApiSceneVersion::V2 as u32,
            image: scene.metadata.image.map(|rl| rl.rid),
            group: Some(room_id.to_string()),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiSchedule {
    pub recycle: bool,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autodelete: Option<bool>,
    pub description: String,
    pub command: Value,
    #[serde(with = "date_format::legacy_utc")]
    pub created: DateTime<Utc>,
    #[serde(
        with = "date_format::legacy_utc_opt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub starttime: Option<DateTime<Utc>>,
    pub time: String,
    pub localtime: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiSensor {
    #[serde(rename = "type")]
    pub sensor_type: String,
    pub config: Value,
    pub name: String,
    pub state: Value,
    pub manufacturername: String,
    pub modelid: String,
    pub swversion: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swupdate: Option<SwUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uniqueid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diversityid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub productname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recycle: Option<bool>,
    #[serde(skip_serializing_if = "Value::is_null", default)]
    pub capabilities: Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiUserConfig {
    pub config: ApiConfig,
    pub groups: HashMap<String, ApiGroup>,
    pub lights: HashMap<String, ApiLight>,
    pub resourcelinks: HashMap<u32, ApiResourceLink>,
    pub rules: HashMap<u32, ApiRule>,
    pub scenes: HashMap<String, ApiScene>,
    pub schedules: HashMap<u32, ApiSchedule>,
    pub sensors: HashMap<u32, ApiSensor>,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            analyticsconsent: false,
            backup: ApiBackup::default(),
            short_config: ApiShortConfig::default(),
            dhcp: true,
            internetservices: ApiInternetServices::default(),
            linkbutton: Default::default(),
            portalconnection: ConnectionState::Disconnected,
            portalservices: Default::default(),
            portalstate: PortalState::default(),
            proxyaddress: "none".to_string(),
            proxyport: Default::default(),
            swupdate2: SoftwareUpdate2::new(),
            zigbeechannel: 25,
            ipaddress: Ipv4Addr::UNSPECIFIED,
            netmask: Ipv4Addr::UNSPECIFIED,
            gateway: Ipv4Addr::UNSPECIFIED,
            timezone: best_guess_timezone(),
            utc: Utc::now(),
            localtime: Local::now(),
            whitelist: HashMap::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Capacity {
    pub available: u32,
    pub total: u32,
}

impl Capacity {
    #[must_use]
    pub const fn new(total: u32, available: u32) -> Self {
        Self { available, total }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SensorsCapacity {
    pub available: u32,
    pub total: u32,
    pub clip: Capacity,
    pub zll: Capacity,
    pub zgp: Capacity,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ScenesCapacity {
    pub available: u32,
    pub total: u32,
    pub lightstates: Capacity,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RulesCapacity {
    pub available: u32,
    pub total: u32,
    pub conditions: Capacity,
    pub actions: Capacity,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct StreamingCapacity {
    pub available: u32,
    pub total: u32,
    pub channels: u32,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Capabilities {
    pub lights: Capacity,
    pub sensors: SensorsCapacity,
    pub groups: Capacity,
    pub schedules: Capacity,
    pub rules: RulesCapacity,
    pub resourcelinks: Capacity,
    pub streaming: StreamingCapacity,
    pub timezones: Value,
}

impl Capabilities {
    #[must_use]
    pub fn new() -> Self {
        Self {
            lights: Capacity::new(63, 60),
            sensors: SensorsCapacity {
                available: 240,
                total: 250,
                clip: Capacity::new(250, 240),
                zll: Capacity::new(64, 63),
                zgp: Capacity::new(64, 63),
            },
            groups: Capacity::new(64, 60),
            schedules: Capacity::new(100, 95),
            rules: RulesCapacity {
                available: 233,
                total: 255,
                conditions: Capacity::new(1500, 1451),
                actions: Capacity::new(1000, 954),
            },
            resourcelinks: Capacity::new(64, 59),
            streaming: StreamingCapacity {
                available: 1,
                total: 1,
                channels: 20,
            },
            timezones: json!({
                "values": [
                    "CET",
                    "UTC",
                    "GMT",
                    "Europe/Copenhagen",
                ],
            }),
        }
    }
}
