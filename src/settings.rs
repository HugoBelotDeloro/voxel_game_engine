use bevy::prelude::*;

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Settings>();
    }
}

#[derive(Resource)]
pub struct Settings {
    pub keys: Keys,
    pub sensitivity: f32,
    pub horizontal_speed: f32,
    pub vertical_speed: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            keys: Keys::default(),
            sensitivity: 0.2,
            horizontal_speed: 2.,
            vertical_speed: 5.,
        }
    }
}

pub struct Keys {
    pub forward: u32,
    pub back: u32,
    pub left: u32,
    pub right: u32,
    pub up: u32,
    pub down: u32,
}

impl Default for Keys {
    fn default() -> Self {
        Keys {
            forward: 17,
            back: 31,
            left: 30,
            right: 32,
            up: 57,
            down: 42,
        }
    }
}
