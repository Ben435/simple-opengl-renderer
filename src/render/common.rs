use cgmath::{vec3,Vector3};

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

impl From<[f32; 3]> for Color {
    fn from(col: [f32; 3]) -> Self {
        Color {
            r: col[0],
            g: col[1],
            b: col[2],
        }
    }
}
