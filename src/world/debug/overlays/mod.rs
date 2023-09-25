use bevy::prelude::*;
mod chunk_boundaries;

pub(crate) use chunk_boundaries::ToggleChunkBoundaryOverlayEvent;

pub(super) struct OverlaysPlugin;

impl Plugin for OverlaysPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(chunk_boundaries::ChunkBoundaryOverlayPlugin);
    }
}
