use super::common::Color;
use tobj::Material as TobjMaterial;

#[derive(Debug,Clone,Copy)]
pub struct Material {
    pub ambient: Color,
    pub diffuse: Color,
    pub specular: Color,
    pub shininess: f32,
}

impl Default for Material {
    fn default() -> Material {
        let base_color = Color::rgb(1.0, 0.5, 0.2); // Orange
        Material {
            ambient: base_color,
            diffuse: base_color,
            specular: Color::rgb(1.0, 1.0, 1.0),
            shininess: 32.0,
        }
    }
}

impl From<&TobjMaterial> for Material {
    fn from(mat: &TobjMaterial) -> Self {
        Material {
            ambient: Color::from(mat.ambient),
            diffuse: Color::from(mat.diffuse),
            specular: Color::from(mat.specular),
            shininess: mat.shininess as f32,
        }
    }
}
