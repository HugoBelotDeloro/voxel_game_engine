use bevy::{input::mouse::MouseMotion, prelude::*};

#[derive(Component, Default)]
pub struct PlayerController {
    pub mouse_delta: Vec2,
    pub horizontal_movement: Vec2,
    pub vertical_movement: f32,
}

pub fn player_controller(
    keys: Res<Input<ScanCode>>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut query: Query<&mut PlayerController>,
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
    for mut player_controller in query.iter_mut() {
        player_controller.mouse_delta = mouse_delta;
        player_controller.horizontal_movement = horizontal_movement;
        player_controller.vertical_movement = vertical_movement;
    }
}
