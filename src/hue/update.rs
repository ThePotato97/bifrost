use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    hue::v2::{On, RType},
    types::XY,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Update {
    /* BehaviorScript(BehaviorScriptUpdate), */
    /* BehaviorInstance(BehaviorInstanceUpdate), */
    /* Bridge(BridgeUpdate), */
    /* BridgeHome(BridgeHomeUpdate), */
    /* Device(DeviceUpdate), */
    /* Entertainment(EntertainmentUpdate), */
    /* GeofenceClient(GeofenceClientUpdate), */
    /* Geolocation(GeolocationUpdate), */
    GroupedLight(GroupedLightUpdate),
    /* Homekit(HomekitUpdate), */
    Light(LightUpdate),
    /* Matter(MatterUpdate), */
    /* PublicImage(PublicImageUpdate), */
    /* Room(RoomUpdate), */
    Scene(SceneUpdate),
    /* SmartScene(SmartSceneUpdate), */
    /* ZigbeeConnectivity(ZigbeeConnectivityUpdate), */
    /* ZigbeeDeviceDiscovery(ZigbeeDeviceDiscoveryUpdate), */
    /* Zone(ZoneUpdate), */
}

impl Update {
    #[must_use]
    pub const fn rtype(&self) -> RType {
        match self {
            Self::GroupedLight(_) => RType::GroupedLight,
            Self::Light(_) => RType::Light,
            Self::Scene(_) => RType::Scene,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateRecord {
    id: Uuid,
    id_v1: String,
    #[serde(flatten)]
    pub obj: Update,
}

impl UpdateRecord {
    #[must_use]
    pub fn from_ref((id, obj): (&Uuid, &Update)) -> Self {
        Self {
            id: *id,
            id_v1: format!("/legacy/{}", id.as_simple()),
            obj: obj.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LightUpdate {
    pub on: Option<On>,
    pub dimming: Option<DimmingUpdate>,
    pub color: Option<ColorUpdate>,
    pub color_temp: Option<f64>,
    pub color_temperature: Option<ColorTemperatureUpdate>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupedLightUpdate {
    pub on: Option<On>,
    pub dimming: Option<DimmingUpdate>,
    pub color: Option<ColorUpdate>,
    pub color_temp: Option<f64>,
    pub color_temperature: Option<ColorTemperatureUpdate>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DimmingUpdate {
    pub brightness: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ColorUpdate {
    pub xy: XY,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ColorTemperatureUpdate {
    pub mirek: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SceneUpdate {
    pub recall: Option<SceneRecall>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SceneRecall {
    pub action: Option<SceneRecallAction>,
    pub duration: Option<u32>,
    pub dimming: Option<DimmingUpdate>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum SceneRecallAction {
    Active,
    DynamicPalette,
    Static,
}