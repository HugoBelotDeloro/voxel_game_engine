mod diagnostics_hud;

use bevy::prelude::*;

pub struct DebugToolsPlugin;

impl Plugin for DebugToolsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(diagnostics_hud::DiagnosticsHudPlugin);
    }
}
