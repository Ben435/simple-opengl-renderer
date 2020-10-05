use simple_opengl_renderer::window::*;
use simple_opengl_renderer::camera::*;
use simple_opengl_renderer::render::{*,common::*,ogl::*};
use log::{debug,info,LevelFilter};
use env_logger::{Builder};
use cgmath::{Matrix4,vec3};

pub fn main() {
    Builder::new()
        .filter(None, LevelFilter::Debug)
        .init();
    info!("Logger initialized");

    let mut window = Window::new("Simple Renderer", 800, 600).expect("Failed to init window");
    let mut cam = Camera::default();
    let renderer = SimpleRenderer::<GlMesh>::new();

    let shader = GlShader::default_shader();

    let demo_mesh = GlMesh::cube();
    // Gold from (http://devernay.free.fr/cours/opengl/materials.html)
    let demo_material = Material {
        ambient: Color::rgb(0.24725, 0.1995, 0.0745),
        diffuse: Color::rgb(0.75164, 0.60648, 0.22648),
        specular: Color::rgb(0.628281, 0.555802, 0.366065),
        shininess: 0.4,
    };

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
            let time = window.get_time();

            let x: f32 = time.sin() as f32 * 2.0;
            let y: f32 = time.cos() as f32 * 2.0;

            ctx.submit(&demo_mesh, Matrix4::from_translation(vec3(x, y, -6.0)), &demo_material, &shader);

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
