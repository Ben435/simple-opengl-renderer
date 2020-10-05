use gl;
use std::collections::{HashMap};
use std::marker::PhantomData;
use std::ptr;
use cgmath::{Matrix4,EuclideanSpace,Point3,vec3};

use crate::camera::Camera;
use super::renderable::Renderable;
use super::ogl::GlShader;
use super::light::{PointLight,DirectionalLight};
use super::material::Material;

// const MAX_VERTICES: usize = 10_000;
// const MAX_VBO_SIZE: usize = MAX_VERTICES * size_of::<Vertex>();
// const MAX_IBO_SIZE: usize = 20_000;

/// Simple Renderer. 
/// Use `SimpleRenderer::begin()` to open a context, submit renderables to the context, then present.
/// This "manager" + "context" pattern helps guide the borrow checker, while still persisting the parent manager.
pub struct SimpleRenderer<T : Renderable> {
    phantom: PhantomData<T>,
    pub point_light: PointLight,
    pub directional_light: DirectionalLight,
}

pub struct SimpleRenderContext<'a, T : Renderable> {
    shader_buckets: HashMap<&'a GlShader, Vec<RenderJob<'a, T>>>,
    renderer: &'a SimpleRenderer<T>,
}

struct RenderJob<'a, T : Renderable> {
    pub renderable: &'a T,
    pub transform: Matrix4<f32>,
    pub material: &'a Material,
}

impl <'a, T: Renderable> SimpleRenderer<T> {
    pub fn new() -> SimpleRenderer<T> {
        let mut res = SimpleRenderer::<T>{
            phantom: PhantomData,
            point_light: PointLight::white(Point3::new(10.0, 0.0, -5.0)),
            directional_light: DirectionalLight::white(vec3(0.0, 0.0, 1.0))
        };

        res.init();

        res
    }

    fn init(&mut self) {}

    /// Begin new render context
    /// Encapsulates lifetime around render queue
    /// TODO: Optimization available by carrying buffer from VecDeque between frames.
    pub fn begin(&'a self) -> SimpleRenderContext<'a, T> {
        SimpleRenderContext{
            shader_buckets: HashMap::new(),
            renderer: self,
        }
    }
}

impl <'a, T : Renderable> SimpleRenderContext<'a, T> {
    pub fn submit(&mut self, renderable: &'a T, transform: Matrix4<f32>, material: &'a Material, shader: &'a GlShader) {
        let existing = self.shader_buckets.get_mut(shader);
        let render_job = RenderJob{
            renderable,
            transform,
            material,
        };

        if existing.is_some() {
            let existing = existing.unwrap();
            existing.push(render_job);
        } else {
            let existing = vec!(render_job);
            self.shader_buckets.insert(shader, existing);
        }
    }

    pub fn present(&mut self, camera: &Camera) {
        let vw_matrix = camera.get_view_matrix();
        let pr_matrix = camera.get_projection_matrix();

        for shader in self.shader_buckets.keys() {
            let to_render = self.shader_buckets.get(shader);
            if to_render.is_none() {
                // Nothing to render for this shader.
                continue;
            }
            let to_render = to_render.unwrap();

            shader.enable();
            shader.set_uniform_3f("view_pos".to_string(), &camera.position.to_vec());
            shader.set_uniform_mat4("vw_matrix".to_string(), &vw_matrix);
            shader.set_uniform_mat4("pr_matrix".to_string(), &pr_matrix);

            shader.set_uniform_3f("point_light.position".to_string(), &self.renderer.point_light.position.to_vec());
            shader.set_uniform_3f("point_light.ambient".to_string(), &self.renderer.point_light.ambient.into());
            shader.set_uniform_3f("point_light.diffuse".to_string(), &self.renderer.point_light.diffuse.into());
            shader.set_uniform_3f("point_light.specular".to_string(), &self.renderer.point_light.specular.into());
            shader.set_uniform_1f("point_light.constant".to_string(), 1.0);
            shader.set_uniform_1f("point_light.linear".to_string(), 0.09);
            shader.set_uniform_1f("point_light.quadratic".to_string(), 0.032);

            shader.set_uniform_3f("directional_light.direction".to_string(), &self.renderer.directional_light.direction);
            shader.set_uniform_3f("directional_light.ambient".to_string(), &self.renderer.directional_light.ambient.into());
            shader.set_uniform_3f("directional_light.diffuse".to_string(), &self.renderer.directional_light.diffuse.into());
            shader.set_uniform_3f("directional_light.specular".to_string(), &self.renderer.directional_light.specular.into());

            for job in to_render {
                shader.set_uniform_mat4("ml_matrix".to_string(), &job.transform);

                shader.set_uniform_3f("material.ambient".to_string(), &job.material.ambient.into());
                shader.set_uniform_3f("material.diffuse".to_string(), &job.material.diffuse.into());
                shader.set_uniform_3f("material.specular".to_string(), &job.material.specular.into());
                shader.set_uniform_1f("material.shininess".to_string(), job.material.shininess);

                job.renderable.get_vao().bind();
                let ebo = job.renderable.get_ebo();
                ebo.bind();

                unsafe {
                    gl::DrawElements(gl::TRIANGLES, ebo.components as i32, gl::UNSIGNED_SHORT, ptr::null());
                }
            }
        }
    }
}
