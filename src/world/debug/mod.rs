use bevy::prelude::*;
use overlays::OverlaysPlugin;
pub(crate) use overlays::ToggleChunkBoundaryOverlayEvent;

mod overlays;

pub(super) struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(OverlaysPlugin);
    }
}
