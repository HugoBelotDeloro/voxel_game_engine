use bevy::prelude::*;

mod debug;
mod materials;
mod player;
mod player_controller;
mod settings;
mod world;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            world::WorldPlugin,
            player::PlayerPlugin,
            settings::SettingsPlugin,
            debug::DebugToolsPlugin,
        ))
        .run();
}
