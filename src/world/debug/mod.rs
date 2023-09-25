use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

use overlays::ChunkBoundaryOverlayPlugin;
pub use overlays::ToggleChunkBoundaryOverlayEvent;

mod overlays;

pub struct DebugPluginGroup;

impl PluginGroup for DebugPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<DebugPluginGroup>().add(ChunkBoundaryOverlayPlugin)
    }
}
