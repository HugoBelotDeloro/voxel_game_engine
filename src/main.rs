use bevy::prelude::*;

mod chunk;
mod debug_text;
mod materials;
mod player;
mod player_controller;
mod settings;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            chunk::TestScenePlugin,
            player::PlayerPlugin,
            settings::SettingsPlugin,
            debug_text::DebugTextPlugin,
        ))
        .run();
}
