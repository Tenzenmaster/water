#![allow(dead_code)]

use std::f32::consts::FRAC_PI_2;

use bytemuck::{Pod, Zeroable};
use nalgebra as na;

#[rustfmt::skip]
const OPENGL_TO_WGPU_MATRIX: na::Matrix4<f32> = na::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);

const SAFE_FRAC_PI_2: f32 = FRAC_PI_2 - 0.0001;

#[derive(Debug)]
pub struct Camera(pub na::Isometry3<f32>);

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Projection {
    pub aspect: f32,
    pub fovy: f32,
    pub z_near: f32,
    pub z_far: f32,
}

impl Projection {
    pub fn to_matrix(&self) -> na::Matrix4<f32> {
        OPENGL_TO_WGPU_MATRIX * na::Matrix4::new_perspective(self.aspect, self.fovy, self.z_near, self.z_far)
    }
}
