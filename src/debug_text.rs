use bevy::prelude::*;

use crate::player_controller::PlayerController;

pub struct DebugTextPlugin;

impl Plugin for DebugTextPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(Update, update_text);
    }
}

#[derive(Component)]
pub struct DebugText;

fn setup(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section(
            "Hello",
            TextStyle {
                font_size: 20.,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(12.),
            left: Val::Px(12.),
            ..default()
        }),
        DebugText,
    ));
}

fn update_text(
    mut text: Query<&mut Text, With<DebugText>>,
    player_controller: Query<&GlobalTransform, With<PlayerController>>,
) {
    for mut text in text.iter_mut() {
        for transform in player_controller.iter() {
            let t = transform.compute_transform();
            let Vec3 { x, y, z } = t.translation;
            let (yaw, pitch, roll) = t.rotation.to_euler(EulerRot::XYZ);
            text.sections[0].value = format!(
                "Coords: X: {x:.2}/Y: {y:.2}/Z: {z:.2}\nRotation: Yaw: {yaw:.2}/Pitch: {pitch:.2}/Roll: {roll:.2}"
            );
        }
    }
}
