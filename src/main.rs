use bevy::prelude::*;

mod debug_text;
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
            debug_text::DebugTextPlugin,
        ))
        .run();
}
