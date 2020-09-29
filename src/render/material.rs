use super::common::Color;
use wavefront_obj::mtl::Material as WavefrontMaterial;

#[derive(Debug,Clone,Copy)]
pub struct Material {
    pub ambient: Color,
    pub diffuse: Color,
    pub specular: Color,
    pub shininess: f32,
}

impl Default for Material {
    fn default() -> Material {
        let base_color = Color::new(1.0, 0.5, 0.2); // Orange
        Material {
            ambient: base_color,
            diffuse: base_color,
            specular: Color::new(1.0, 1.0, 1.0),
            shininess: 32.0,
        }
    }
}

impl From<&WavefrontMaterial> for Material {
    fn from(mat: &WavefrontMaterial) -> Self {
        Material {
            ambient: Color::from(mat.color_ambient),
            diffuse: Color::from(mat.color_diffuse),
            specular: Color::from(mat.color_specular),
            shininess: mat.specular_coefficient as f32,
        }
    }
}
