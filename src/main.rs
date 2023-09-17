//! Create a custom material to draw basic lines in 3D

use bevy::{
    input::mouse::MouseMotion,
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::{
        mesh::{MeshVertexBufferLayout, PrimitiveTopology},
        render_resource::{
            AsBindGroup, PolygonMode, RenderPipelineDescriptor, ShaderRef,
            SpecializedMeshPipelineError,
        },
    },
};

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

#[derive(Default, AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "050ce6ac-080a-4d8c-b6b5-b5bab7560d8f"]
struct LineMaterial {
    #[uniform(0)]
    color: Color,
}

impl Material for LineMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/line_material.wgsl".into()
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        // This is the important part to tell bevy to render this material as a line between vertices
        descriptor.primitive.polygon_mode = PolygonMode::Line;
        Ok(())
    }
}

/// A list of lines with a start and end position
#[derive(Debug, Clone)]
pub struct LineList {
    pub lines: Vec<(Vec3, Vec3)>,
}

impl From<LineList> for Mesh {
    fn from(line: LineList) -> Self {
        // This tells wgpu that the positions are list of lines
        // where every pair is a start and end point
        let mut mesh = Mesh::new(PrimitiveTopology::LineList);

        let vertices: Vec<_> = line.lines.into_iter().flat_map(|(a, b)| [a, b]).collect();
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        mesh
    }
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
