use simple_opengl_renderer::render::*;
use simple_opengl_renderer::window::*;
use simple_opengl_renderer::camera::*;
use log::{debug,info,LevelFilter};
use env_logger::{Builder};
use cgmath::{Matrix4,vec3};

pub fn main() {
    Builder::new()
        .filter(None, LevelFilter::Debug)
        .init();
    info!("Logger initialized");

    let mut window = Window::new("Simple Renderer", 800, 600).expect("Failed to init window");
    let cam = Camera::default();
    let renderer = SimpleRenderer::<GlMesh>::new();

    let shader = GlShader::default_shader();

    let room_model = GlModel::builder()
        .with_obj_file("./assets/room.obj".to_string())
        .build();

    while !window.should_close() {
        for (_, event) in window.flush_events() {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    // make sure the viewport matches the new window dimensions
                    debug!("Resize to {}, {}", width, height);
                    unsafe { gl::Viewport(0, 0, width, height) }
                },
                _ => {},
            }
        };

        {
            let mut ctx = renderer.begin();
            let time = window.get_time();

            ctx.submit(&room_model.meshes.get(0).unwrap(), Matrix4::from_translation(vec3(0.0, 0.0, 0.0)), &demo_material, &shader);

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
