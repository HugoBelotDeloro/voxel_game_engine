use bevy::prelude::*;

mod debug_text;
mod material;
mod player;
mod player_controller;
mod settings;
mod test_scene;

use material::LineMaterial;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            MaterialPlugin::<LineMaterial>::default(),
            test_scene::TestScenePlugin,
            player::PlayerPlugin,
            settings::SettingsPlugin,
            debug_text::DebugTextPlugin,
        ))
        .run();
}
