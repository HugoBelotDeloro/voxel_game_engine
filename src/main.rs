//! Create a custom material to draw basic lines in 3D

use bevy::{input::mouse::MouseMotion, prelude::*};

mod material;
use material::{LineList, LineMaterial};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MaterialPlugin::<LineMaterial>::default()))
        .add_systems(Startup, setup)
        .add_systems(Update, (mouse_move_camera, kb_move_camera))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<LineMaterial>>,
) {
    // Spawn a list of lines with start and end points for each lines
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(LineList {
            lines: vec![
                (Vec3::ZERO, Vec3::new(1.0, 0.0, 0.0)),
                (Vec3::ZERO, Vec3::new(0.0, 1.0, 0.0)),
                (Vec3::ZERO, Vec3::new(0.0, 0.0, 1.0)),
                (Vec3::new(1.0, 1.0, 0.0), Vec3::new(1.0, 1.0, 1.0)),
                (Vec3::new(1.0, 1.0, 0.0), Vec3::new(1.0, 0.0, 0.0)),
                (Vec3::new(1.0, 1.0, 0.0), Vec3::new(0.0, 1.0, 0.0)),
                (Vec3::new(0.0, 1.0, 1.0), Vec3::new(1.0, 1.0, 1.0)),
                (Vec3::new(0.0, 1.0, 1.0), Vec3::new(0.0, 0.0, 1.0)),
                (Vec3::new(0.0, 1.0, 1.0), Vec3::new(0.0, 1.0, 0.0)),
                (Vec3::new(1.0, 0.0, 1.0), Vec3::new(0.0, 0.0, 1.0)),
                (Vec3::new(1.0, 0.0, 1.0), Vec3::new(1.0, 1.0, 1.0)),
                (Vec3::new(1.0, 0.0, 1.0), Vec3::new(1.0, 0.0, 0.0)),
            ],
        })),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        material: materials.add(LineMaterial {
            color: Color::GREEN,
        }),
        ..default()
    });

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.5, 0.5, 5.0).looking_at(Vec3::new(0.5, 0.5, 0.0), Vec3::Y),
        ..default()
    });
}

fn mouse_move_camera(
    mut ev_motion: EventReader<MouseMotion>,
    mut query: Query<&mut Transform, With<Camera>>,
    timer: Res<Time>,
) {
    let mut rotation_move = Vec2::ZERO;
    for ev in ev_motion.iter() {
        rotation_move += ev.delta;
    }
    rotation_move *= timer.delta_seconds() * 0.1;

    for mut transform in query.iter_mut() {
        transform.rotation = transform.rotation * Quat::from_rotation_x(rotation_move.y);
        transform.rotation = Quat::from_rotation_y(rotation_move.x) * transform.rotation;
    }

    ev_motion.clear();
}

fn kb_move_camera(
    keys: Res<Input<ScanCode>>,
    mut query: Query<&mut Transform, With<Camera>>,
    timer: Res<Time>,
) {
    // 17 30 31 32 57 42
    let mut translation = Vec3::ZERO;
    if keys.pressed(ScanCode(17)) {
        translation += Vec3::new(0., 0., -1.);
    }
    if keys.pressed(ScanCode(30)) {
        translation += Vec3::new(-1., 0., 0.);
    }
    if keys.pressed(ScanCode(31)) {
        translation += Vec3::new(0., 0., 1.);
    }
    if keys.pressed(ScanCode(32)) {
        translation += Vec3::new(1., 0., 0.);
    }
    translation = translation.normalize_or_zero() * timer.delta_seconds();

    for mut transform in query.iter_mut() {
        transform.translation += translation;
    }
}
