use crate::{player_inputs::PlayerInputs, settings::Settings};
use bevy::prelude::*;

pub(super) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (move_body, move_head))
            .add_systems(Startup, setup);
    }
}

#[derive(Component)]
struct Player;

fn setup(mut commands: Commands) {
    let camera = commands
        .spawn((
            Camera3dBundle {
                transform: Transform::from_xyz(0., 0., 0.)
                    .looking_at(Vec3::new(0., 0., -1.), Vec3::Y),
                ..default()
            },
            PlayerInputs::default(),
        ))
        .id();

    let player = commands
        .spawn((
            SpatialBundle {
                transform: Transform::from_xyz(0.5, 0.5, 0.5),
                ..default()
            },
            Player,
        ))
        .id();

    commands.entity(player).push_children(&[camera]);
}

fn move_body(
    mut body_query: Query<&mut Transform, With<Player>>,
    controller_query: Query<&PlayerInputs>,
    timer: Res<Time>,
    settings: Res<Settings>,
) {
    for player_controller in controller_query.iter() {
        for mut transform in body_query.iter_mut() {
            let direction = (transform.back() * player_controller.horizontal_movement.y
                + transform.right() * player_controller.horizontal_movement.x)
                * settings.horizontal_speed
                + transform.up() * player_controller.vertical_movement * settings.vertical_speed;
            transform.translation += direction * timer.delta_seconds();

            let mouse_delta =
                player_controller.mouse_delta * timer.delta_seconds() * settings.sensitivity;

            transform.rotate_y(mouse_delta.x);
        }
    }
}

fn move_head(
    mut head_query: Query<(&mut Transform, &PlayerInputs)>,
    timer: Res<Time>,
    settings: Res<Settings>,
) {
    for (mut transform, player_controller) in head_query.iter_mut() {
        transform.rotate_x(
            player_controller.mouse_delta.y * timer.delta_seconds() * settings.sensitivity,
        );
    }
}
