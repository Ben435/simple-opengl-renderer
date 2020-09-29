use cgmath::{Point3,Vector3,vec3};
use super::common::{Color};

pub struct PointLight {
    pub position: Point3<f32>,

    pub diffuse: Color,
    pub ambient: Color,
    pub specular: Color,
}

impl PointLight {
    pub fn white(position: Point3<f32>) -> PointLight {
        let color: Color = vec3(1.0, 1.0, 1.0);

        let diffuse = color * 0.5;
        let ambient = diffuse * 0.2;
        let specular = vec3(1.0, 1.0, 1.0);

        PointLight {
            position,
            diffuse,
            ambient,
            specular,
        }
    }
}