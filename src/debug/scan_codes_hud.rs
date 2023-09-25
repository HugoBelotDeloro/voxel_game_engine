use bevy::prelude::*;

pub struct ScanCodesHudPlugin;

impl Plugin for ScanCodesHudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, update_text);
    }
}

#[derive(Component)]
struct ScanCodeHud;

fn setup(mut commands: Commands) {
    let sections_text = ["Last pressed ScanCode: ", ""];

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
            right: Val::Px(12.),
            ..default()
        }),
        ScanCodeHud,
    ));
}

fn update_text(mut text: Query<&mut Text, With<ScanCodeHud>>, keys: Res<Input<ScanCode>>) {
    for mut text in text.iter_mut() {
        if let Some(key) = keys.get_just_pressed().last() {
            text.sections[1].value = key.0.to_string();
        }
    }
}
