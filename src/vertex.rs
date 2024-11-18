#![allow(dead_code)]

use bytemuck::{Pod, Zeroable};

pub trait Vertex {
    const LAYOUT: wgpu::VertexBufferLayout<'static>;
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct ColorVertex {
    pub pos: [f32; 3],
    pub color: [f32; 3],
}

impl Vertex for ColorVertex {
    const LAYOUT: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
        array_stride: size_of::<Self>() as u64,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &[
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x3,
                offset: 0,
                shader_location: 0,
            },
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x3,
                offset: size_of::<[f32; 3]>() as u64,
                shader_location: 1,
            },
        ],
    };
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct TextureVertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}

impl Vertex for TextureVertex {
    const LAYOUT: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
        array_stride: size_of::<Self>() as u64,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &[
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x3,
                offset: 0,
                shader_location: 0,
            },
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x2,
                offset: size_of::<[f32; 3]>() as u64,
                shader_location: 1,
            },
        ],
    };
}

impl TextureVertex {
    pub const SQUARE_VERTICES: [Self; 4] = [
        Self { position: [0.5, 0.5, 0.0], tex_coords: [1.0, 0.0] },
        Self { position: [-0.5, 0.5, 0.0], tex_coords: [0.0, 0.0] },
        Self { position: [-0.5, -0.5, 0.0], tex_coords: [0.0, 1.0] },
        Self { position: [0.5, -0.5, 0.0], tex_coords: [1.0, 1.0] },
    ];
    pub const SQUARE_INDICES: [u16; 6] = [
        0, 1, 2,
        0, 2, 3,
    ];
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct ModelVertex {
    pub position: [f32; 3],
    pub uv: [f32; 2],
    pub normal: [f32; 3],
}

impl Vertex for ModelVertex {
    const LAYOUT: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
        array_stride: size_of::<Self>() as u64,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &[
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x3,
                offset: 0,
                shader_location: 0,
            },
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x2,
                offset: size_of::<[f32; 3]>() as u64,
                shader_location: 1,
            },
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x3,
                offset: size_of::<[f32; 5]>() as u64,
                shader_location: 2,
            },
        ],
    };
}
