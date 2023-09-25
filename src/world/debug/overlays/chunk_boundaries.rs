use bevy::prelude::*;

use crate::{materials::LineMaterial, world::chunk::CHUNK_SIZE};

pub(super) struct ChunkBoundaryOverlayPlugin;

impl Plugin for ChunkBoundaryOverlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_event::<ToggleChunkBoundaryOverlayEvent>()
            .add_systems(Update, toggle_overlay);
    }
}

#[derive(Component)]
struct ChunkBoundaryOverlay;

#[derive(Event)]
pub(crate) struct ToggleChunkBoundaryOverlayEvent;

fn setup(
    mut commands: Commands,
    mut line_materials: ResMut<Assets<LineMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let line_material = line_materials.add(LineMaterial {
        color: Color::BLACK,
    });
    const OFFSET: f32 = CHUNK_SIZE as f32 / 2.;
    commands.spawn((
        MaterialMeshBundle {
            material: line_material,
            transform: Transform::from_xyz(OFFSET, OFFSET, OFFSET),
            mesh: meshes.add(Mesh::from(shape::Cube::new(CHUNK_SIZE as f32))),
            visibility: Visibility::Hidden,
            ..default()
        },
        ChunkBoundaryOverlay,
    ));
}

fn toggle_overlay(
    mut overlay: Query<&mut Visibility, With<ChunkBoundaryOverlay>>,
    mut events: EventReader<ToggleChunkBoundaryOverlayEvent>,
) {
    for _ in events.iter() {
        let mut visibility = overlay.single_mut();
        let new_visibility = match *visibility {
            Visibility::Hidden => Visibility::Visible,
            _ => Visibility::Hidden,
        };
        *visibility = new_visibility;
    }
}
