use bevy::{
    diagnostic::{
        DiagnosticsStore, EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin,
        SystemInformationDiagnosticsPlugin,
    },
    prelude::*,
};

use crate::player_controller::PlayerController;

pub struct DebugTextPlugin;

impl Plugin for DebugTextPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            FrameTimeDiagnosticsPlugin,
            EntityCountDiagnosticsPlugin,
            SystemInformationDiagnosticsPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, update_text);
    }
}

#[derive(Component)]
pub struct DebugText;

fn setup(mut commands: Commands) {
    let sections_text = [
        "FPS: ",             // 0
        "",                  // 1
        "\nCPU: ",           // 2
        "",                  // 3
        "%\nMEM: ",          // 4
        "",                  // 5
        "%\nEntities: ",     // 6
        "",                  // 7
        "\nCoords:\nX: ",    // 8
        "",                  // 9
        "\nY: ",             // 10
        "",                  // 11
        "\nZ: ",             // 12
        "",                  // 13
        "\nRotation: Yaw: ", // 14
        "",                  // 15
        "\nPitch: ",         // 16
        "",                  // 17
        "\nRoll: ",          // 18
        "",                  // 19
    ];

    commands.spawn((
        TextBundle::from_sections(sections_text.map(|section_text| {
            TextSection::new(
                section_text,
                TextStyle {
                    font_size: 20.,
                    ..default()
                },
            )
        }))
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
    diagnostics: Res<DiagnosticsStore>,
) {
    for mut text in text.iter_mut() {
        for transform in player_controller.iter() {
            let fps = diagnostics
                .get(FrameTimeDiagnosticsPlugin::FPS)
                .map(|fps| fps.smoothed())
                .flatten()
                .map(|fps| format!("{fps:.0}"))
                .unwrap_or(String::from("??"));

            let cpu = diagnostics
                .get(SystemInformationDiagnosticsPlugin::CPU_USAGE)
                .map(|cpu| cpu.smoothed())
                .flatten()
                .map(|cpu| format!("{cpu:.2}"))
                .unwrap_or(String::from("??"));

            let mem = diagnostics
                .get(SystemInformationDiagnosticsPlugin::MEM_USAGE)
                .map(|mem| mem.smoothed())
                .flatten()
                .map(|mem| format!("{mem:.2}"))
                .unwrap_or(String::from("??"));

            let ec = diagnostics
                .get(EntityCountDiagnosticsPlugin::ENTITY_COUNT)
                .map(|ec| ec.smoothed())
                .flatten()
                .map(|ec| format!("{ec}"))
                .unwrap_or(String::from("??"));

            let t = transform.compute_transform();
            let Vec3 { x, y, z } = t.translation;
            let (yaw, pitch, roll) = t.rotation.to_euler(EulerRot::XYZ);

            text.sections[1].value = format!("{fps}");
            text.sections[3].value = format!("{cpu}");
            text.sections[5].value = format!("{mem}");
            text.sections[7].value = format!("{ec}");
            text.sections[9].value = format!("{x:.2}");
            text.sections[11].value = format!("{y:.2}");
            text.sections[13].value = format!("{z:.2}");
            text.sections[15].value = format!("{yaw:.2}");
            text.sections[17].value = format!("{pitch:.2}");
            text.sections[19].value = format!("{roll:.2}");
        }
    }
}
