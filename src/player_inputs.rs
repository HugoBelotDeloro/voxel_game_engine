use bevy::{input::mouse::MouseMotion, prelude::*};

use crate::world::ToggleChunkBoundaryOverlayEvent;

pub(super) struct PlayerInputsPlugin;

impl Plugin for PlayerInputsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, process_player_inputs)
            .init_resource::<PlayerInputs>();
    }
}

#[derive(Resource, Default)]
pub(crate) struct PlayerInputs {
    pub(crate) mouse_delta: Vec2,
    pub(crate) horizontal_movement: Vec2,
    pub(crate) vertical_movement: f32,
}

fn process_player_inputs(
    keys: Res<Input<ScanCode>>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut player_inputs: ResMut<PlayerInputs>,
    mut ev_toggle_chunk_boundary_overlay: EventWriter<ToggleChunkBoundaryOverlayEvent>,
) {
    // Mouse
    let mut mouse_delta = Vec2::ZERO;
    for ev in mouse_motion.iter() {
        mouse_delta += ev.delta;
    }

    mouse_motion.clear();

    // Keyboard horizontal
    let mut horizontal_movement = Vec2::ZERO;
    if keys.pressed(ScanCode(17)) {
        horizontal_movement += Vec2::new(0., -1.);
    }
    if keys.pressed(ScanCode(30)) {
        horizontal_movement += Vec2::new(-1., 0.);
    }
    if keys.pressed(ScanCode(31)) {
        horizontal_movement += Vec2::new(0., 1.);
    }
    if keys.pressed(ScanCode(32)) {
        horizontal_movement += Vec2::new(1., 0.);
    }
    horizontal_movement = horizontal_movement.normalize_or_zero();

    // Keyboard vertical
    let mut vertical_movement = 0.;
    if keys.pressed(ScanCode(57)) {
        vertical_movement += 1.;
    }
    if keys.pressed(ScanCode(42)) {
        vertical_movement -= 1.;
    }

    // Apply
    player_inputs.mouse_delta = mouse_delta;
    player_inputs.horizontal_movement = horizontal_movement;
    player_inputs.vertical_movement = vertical_movement;

    if keys.just_pressed(ScanCode(18)) {
        ev_toggle_chunk_boundary_overlay.send(ToggleChunkBoundaryOverlayEvent);
    }
}
