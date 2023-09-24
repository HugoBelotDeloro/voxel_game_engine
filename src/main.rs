use bevy::prelude::*;

mod world;
mod debug_text;
mod materials;
mod player;
mod player_controller;
mod settings;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            world::chunk::TestScenePlugin,
            player::PlayerPlugin,
            settings::SettingsPlugin,
            debug_text::DebugTextPlugin,
        ))
        .run();
}
