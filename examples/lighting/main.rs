use simple_opengl_renderer::window::*;
use simple_opengl_renderer::camera::*;
use simple_opengl_renderer::render::{*, ogl::*};
use simple_opengl_renderer::resources::*;
use log::{debug,info,LevelFilter};
use env_logger::{Builder};
use cgmath::{Matrix4,vec3,Deg,Rotation,Rotation3,Vector3,Quaternion};
use std::path::Path;

pub fn main() {
    Builder::new()
        .filter(None, LevelFilter::Debug)
        .init();
    info!("Logger initialized");

    let mut window = Window::new("Model Renderer", 800, 600).expect("Failed to init window");
    let mut cam = Camera::default();
    let mut renderer = SimpleRenderer::<GlMesh>::new();
    let resource_loader = ResourceLoader::from_relative_exe_path(Path::new("../../../examples/lighting")).unwrap();

    let shader = GlShader::default_shader();

    let sphere_path = resource_loader.resolve_path("assets/icosphere.obj").unwrap();
 
    let sphere_model = Model::builder()
        .with_obj_file(sphere_path)
        .build();

    info!("Model loaded!");

    while !window.should_close() {
        for (_, event) in window.flush_events() {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    // make sure the viewport matches the new window dimensions
                    debug!("Resize to {}, {}", width, height);
                    unsafe { gl::Viewport(0, 0, width, height) }
                    cam.update_viewport(width, height);
                },
                _ => {},
            }
        };
        let time = window.get_time() as f32;
        let rotation = (time % 360.0) * 50.0;

        let quat: Quaternion<f32> = Quaternion::from_angle_z(Deg(rotation));
        let new_dir = quat.rotate_vector(Vector3::<f32>::unit_y());
        renderer.directional_light.direction = new_dir;

        {
            let mut ctx = renderer.begin();

            let transform = Matrix4::from_translation(vec3(0.0, 0.0, -5.0)) * Matrix4::from_angle_y(Deg(rotation)) * Matrix4::from_scale(0.5);
            sphere_model.objects.iter().for_each(|obj| {
                ctx.submit(&obj.mesh, transform, &obj.material, &shader);
            });

            unsafe {
                gl::ClearColor(0.2, 0.3, 0.3, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            }

            ctx.present(&cam);
        }

        gl_errors::check_gl_error();

        window.update_screen();
    }
}
