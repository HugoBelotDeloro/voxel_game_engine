use bevy::prelude::*;

pub(super) struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Settings>();
    }
}

#[derive(Resource)]
pub(crate) struct Settings {
    pub(crate) keys: Keys,
    pub(crate) sensitivity: f32,
    pub(crate) horizontal_speed: f32,
    pub(crate) vertical_speed: f32,
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

pub(crate) struct Keys {
    pub(crate) forward: u32,
    pub(crate) back: u32,
    pub(crate) left: u32,
    pub(crate) right: u32,
    pub(crate) up: u32,
    pub(crate) down: u32,
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
