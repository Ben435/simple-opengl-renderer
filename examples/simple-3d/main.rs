use simple_opengl_renderer::render::*;
use simple_opengl_renderer::window::*;
use simple_opengl_renderer::camera::*;
use log::{debug,info,LevelFilter};
use env_logger::{Builder};
use std::ffi::CString;
use cgmath::{Matrix4,vec3};

pub fn main() {
    Builder::new()
        .filter(None, LevelFilter::Debug)
        .init();
    info!("Logger initialized");

    let mut window = Window::new("Simple Renderer", 800, 600).expect("Failed to init window");
    let cam = Camera::default();
    let renderer = SimpleRenderer::<GlMesh>::new();

    let frag_shader = include_str!("./assets/shader.frag");
    let vert_shader = include_str!("./assets/shader.vert");

    let shader = GlShader::builder()
        .with_frag_shader(CString::new(frag_shader).expect("Failed to convert frag shader to CString"))
        .with_vert_shader(CString::new(vert_shader).expect("Failed to convert vert shader to CString"))
        .build();

    let demo_mesh = GlMesh::cube();

    while !window.should_close() {
        for (_, event) in window.flush_events() {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    // make sure the viewport matches the new window dimensions; note that width and
                    // height will be significantly larger than specified on retina displays.
                    debug!("Resize to {}, {}", width, height);
                    unsafe { gl::Viewport(0, 0, width, height) }
                },
                _ => {},
            }
        };

        {
            let mut ctx = renderer.begin();

            ctx.submit(&demo_mesh, Matrix4::from_translation(vec3(0.0, 0.0, -10.0)), &shader);

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
