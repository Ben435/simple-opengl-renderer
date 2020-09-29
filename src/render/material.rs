use cgmath::{vec3};
use super::common::Color;

pub struct Material {
    pub ambient: Color,
    pub diffuse: Color,
    pub specular: Color,
    pub shininess: f32,
}

impl Default for Material {
    fn default() -> Material {
        let base_color = vec3(1.0, 0.5, 0.2); // Orange
        Material {
            ambient: base_color,
            diffuse: base_color,
            specular: vec3(1.0, 1.0, 1.0),
            shininess: 32.0,
        }
    }
}
