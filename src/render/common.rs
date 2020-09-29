use cgmath::{vec3,Vector3};
use wavefront_obj::mtl::Color as WavefrontColor;

#[derive(Debug,Clone,Copy)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Color {
            r,
            g,
            b,
        }
    }
}

impl Into<Vector3<f32>> for Color {
    fn into(self) -> Vector3<f32> {
        vec3(self.r, self.g, self.b)
    }
}

impl From<Vector3<f32>> for Color {
    fn from(v: Vector3<f32>) -> Self {
        Color {
            r: v.x,
            g: v.y,
            b: v.z,
        }
    }
}

impl From<WavefrontColor> for Color {
    fn from(col: WavefrontColor) -> Self {
        Color {
            r: col.r as f32,
            g: col.g as f32,
            b: col.b as f32,
        }
    }
}
