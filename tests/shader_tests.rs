mod common;
use simple_opengl_renderer::render::ogl::GlShader;

#[test]
fn test_default_shader() {
    let window = common::load_offscreen_window().unwrap();

    // Test it loads properly
    GlShader::default_shader();

    // cleanup
    window.close();
}
