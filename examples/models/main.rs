use simple_opengl_renderer::window::*;
use simple_opengl_renderer::camera::*;
use simple_opengl_renderer::render::{*, ogl::*};
use simple_opengl_renderer::resources::*;
use log::{debug,info,LevelFilter};
use env_logger::{Builder};
use cgmath::{Matrix4,vec3,Deg};
use std::path::Path;

pub fn main() {
    Builder::new()
        .filter(None, LevelFilter::Debug)
        .init();
    info!("Logger initialized");

    let mut window = Window::new("Model Renderer", 800, 600).expect("Failed to init window");
    let mut cam = Camera::default();
    let renderer = SimpleRenderer::<GlMesh>::new();
    let resource_loader = ResourceLoader::from_relative_exe_path(Path::new("../../../examples/models")).unwrap();

    let shader = GlShader::default_shader();

    let model_path = resource_loader.resolve_path("assets/teacup.obj").unwrap();
    let base = Model::builder().with_object(
        GlMesh::square(),
        Material::default(),
    ).build();

    info!("Loading: {:?}", model_path);   
 
    let room_model = Model::builder()
        .with_obj_file(
            model_path,
        ).build();

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

        {
            let mut ctx = renderer.begin();
            let time = window.get_time() as f32;
            let rotation = (time % 360.0) * 10.0;
            let transform = Matrix4::from_translation(vec3(0.0, -1.0, -10.0)) * Matrix4::from_angle_x(Deg(45.0)) * Matrix4::from_angle_y(Deg(rotation)) * Matrix4::from_scale(1.0);

            room_model.objects.iter().for_each(|obj| {
                ctx.submit(&obj.mesh, transform, &obj.material, &shader);
            });

            let transform = Matrix4::from_translation(vec3(-5.0, -5.0, -6.0)) * Matrix4::from_angle_x(Deg(-45.0)) * Matrix4::from_scale(10.0);
            base.objects.iter().for_each(|obj| {
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
