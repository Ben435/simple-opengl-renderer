use cgmath::{vec3,Vector3};

#[derive(Debug,Clone,Copy)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
}

impl Color {
    pub fn rgb(r: f32, g: f32, b: f32) -> Self {
        Color {
            r,
            g,
            b,
        }
    }
}

impl From<Color> for Vector3<f32> {
    fn from(color: Color) -> Vector3<f32> {
        vec3(color.r, color.g, color.b)
    }
}

impl From<Vector3<f32>> for Color {
    fn from(vec: Vector3<f32>) -> Color {
        Color {
            r: vec.x,
            g: vec.y,
            b: vec.z,
        }
    }
}

impl From<[f32; 3]> for Color {
    fn from(col: [f32; 3]) -> Self {
        Color {
            r: col[0],
            g: col[1],
            b: col[2],
        }
    }
}
