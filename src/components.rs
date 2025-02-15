use bevy::prelude::*;
use std::collections::BTreeMap;

#[derive(Default, Component)]
pub struct Map {
    pub asset: Handle<crate::MapAsset>,
}

#[derive(Default, Bundle)]
pub struct MapBundle {
    pub map: Map,
    pub transform: TransformBundle,
    pub visibility: VisibilityBundle,
}

#[derive(Default, Component)]
pub struct MapEntityProperties {
    pub classname: String,
    pub transform: Transform,
    pub properties: BTreeMap<String, String>,
}

impl MapEntityProperties {
    // FGD format: https://developer.valvesoftware.com/wiki/FGD

    pub fn get_property_as_f32(&self, key: &str, default: f32) -> f32 {
        if let Some(value) = self.properties.get(key) {
            if let Ok(value) = value.parse::<f32>() {
                return value;
            }
        }
        default
    }

    pub fn get_property_as_i32(&self, key: &str, default: i32) -> i32 {
        if let Some(value) = self.properties.get(key) {
            if let Ok(value) = value.parse::<i32>() {
                return value;
            }
        }
        default
    }

    pub fn get_property_as_bool(&self, key: &str, default: bool) -> bool {
        if let Some(value) = self.properties.get(key) {
            if let Ok(value) = value.parse::<i32>() {
                return value != 0;
            }
        }
        default
    }

    pub fn get_property_as_color(&self, key: &str, default: Color) -> Color {
        if let Some(value) = self.properties.get(key) {
            let value = value.trim().split(" ").collect::<Vec<&str>>();
            if value.len() == 3 {
                if let (Ok(r), Ok(g), Ok(b)) = (
                    value[0].parse::<f32>(),
                    value[1].parse::<f32>(),
                    value[2].parse::<f32>(),
                ) {
                    return Color::rgb(r, g, b);
                }
            }
        }
        default
    }
}
