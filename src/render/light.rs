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
        let color = Color::rgb(1.0, 1.0, 1.0);

        let (ambient, diffuse, specular) = standard_split(color);

        PointLight {
            position,
            ambient,
            diffuse,
            specular,
        }
    }
}

pub struct DirectionalLight {
    pub direction: Vector3<f32>,

    pub diffuse: Color,
    pub ambient: Color,
    pub specular: Color,
}

impl DirectionalLight {
    pub fn white(direction: Vector3<f32>) -> DirectionalLight {
        let color = Color::rgb(1.0, 1.0, 1.0);

        let (ambient, diffuse, specular) = standard_split(color);

        DirectionalLight {
            direction,
            ambient,
            diffuse,
            specular,
        }
    }
}

/// Split color into ambient + diffuse + specular
fn standard_split(color: Color) -> (Color, Color, Color) {
    let vec_color: Vector3<f32> = color.into();
    let diffuse = vec_color * 0.5;
    let ambient = diffuse * 0.2;
    let specular = vec3(1.0, 1.0, 1.0);

    return (ambient.into(), diffuse.into(), specular.into())
}
