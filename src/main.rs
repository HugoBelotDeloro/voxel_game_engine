use bevy::prelude::*;

mod debug;
mod materials;
mod player;
mod player_inputs;
mod settings;
mod world;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            debug::DebugToolsPlugin,
            materials::MaterialsPlugin,
            player_inputs::PlayerInputsPlugin,
            player::PlayerPlugin,
            settings::SettingsPlugin,
            world::WorldPlugin,
        ))
        .run();
}
