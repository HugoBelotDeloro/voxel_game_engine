use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::{AsBindGroup, ShaderRef},
};

impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/custom_material.wgsl".into()
    }
}

#[derive(TypePath, AsBindGroup, Debug, Clone, TypeUuid)]
#[uuid = "7d48eaba-6e47-41eb-a738-06288771bb68"]
pub struct CustomMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub color_texture: Option<Handle<Image>>,
}
