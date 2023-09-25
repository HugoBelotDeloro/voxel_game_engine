use bevy::{
    diagnostic::{
        DiagnosticsStore, EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin,
        SystemInformationDiagnosticsPlugin,
    },
    prelude::*,
};

use crate::player_controller::PlayerController;

pub struct DiagnosticsHudPlugin;

impl Plugin for DiagnosticsHudPlugin {
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
pub struct DiagnosticsHud;

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
        DiagnosticsHud,
    ));
}

fn update_text(
    mut text: Query<&mut Text, With<DiagnosticsHud>>,
    player_controller: Query<&GlobalTransform, With<PlayerController>>,
    diagnostics: Res<DiagnosticsStore>,
) {
    for mut text in text.iter_mut() {
        for transform in player_controller.iter() {
            let fps = diagnostics
                .get(FrameTimeDiagnosticsPlugin::FPS)
                .and_then(|fps| fps.smoothed())
                .map(|fps| format!("{fps:.0}"))
                .unwrap_or(String::from("??"));

            let cpu = diagnostics
                .get(SystemInformationDiagnosticsPlugin::CPU_USAGE)
                .and_then(|cpu| cpu.smoothed())
                .map(|cpu| format!("{cpu:.2}"))
                .unwrap_or(String::from("??"));

            let mem = diagnostics
                .get(SystemInformationDiagnosticsPlugin::MEM_USAGE)
                .and_then(|mem| mem.smoothed())
                .map(|mem| format!("{mem:.2}"))
                .unwrap_or(String::from("??"));

            let ec = diagnostics
                .get(EntityCountDiagnosticsPlugin::ENTITY_COUNT)
                .and_then(|ec| ec.smoothed())
                .map(|ec| format!("{ec}"))
                .unwrap_or(String::from("??"));

            let t = transform.compute_transform();
            let Vec3 { x, y, z } = t.translation;
            let (yaw, pitch, roll) = t.rotation.to_euler(EulerRot::XYZ);

            text.sections[1].value = fps.to_string();
            text.sections[3].value = cpu.to_string();
            text.sections[5].value = mem.to_string();
            text.sections[7].value = ec.to_string();
            text.sections[9].value = format!("{x:.2}");
            text.sections[11].value = format!("{y:.2}");
            text.sections[13].value = format!("{z:.2}");
            text.sections[15].value = format!("{yaw:.2}");
            text.sections[17].value = format!("{pitch:.2}");
            text.sections[19].value = format!("{roll:.2}");
        }
    }
}
