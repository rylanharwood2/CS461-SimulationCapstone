//Taken from https://github.com/bevyengine/bevy/blob/main/examples/shader/shader_material.rs
//WIP DOES NOT WORK YET

use bevy::{
    prelude::*,
    pbr::{MaterialPipeline, MaterialPipelineKey},
    reflect::TypePath,
    render::{
        mesh::MeshVertexBufferLayout,
        render_resource::{
            AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError,
        },
    },
};

//basic terrain shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct BasicTerrainMaterial {
    #[uniform(0)]
    pub color: Color,
    #[texture(1)]
    #[sampler(2)]
    pub color_texture: Option<Handle<Image>>,
    pub alpha_mode: AlphaMode,
}

/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
/// When using the GLSL shading language for your shader, the specialize method must be overridden.
impl Material for BasicTerrainMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/terrain_shader.wgsl".into()
    }
    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}