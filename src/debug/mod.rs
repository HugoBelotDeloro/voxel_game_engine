mod diagnostics_hud;
mod scan_codes_hud;

use bevy::prelude::*;

pub(super) struct DebugToolsPlugin;

impl Plugin for DebugToolsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            diagnostics_hud::DiagnosticsHudPlugin,
            scan_codes_hud::ScanCodesHudPlugin,
        ));
    }
}
