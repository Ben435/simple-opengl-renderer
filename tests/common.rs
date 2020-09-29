
pub fn load_offscreen_window() -> Result<glfw::Window, String> {
    let glfw = glfw::init(glfw::FAIL_ON_ERRORS);
    if glfw.is_err() {
        return Err(format!("Init glfw error: {}", glfw.err().unwrap()))
    }
    let mut glfw = glfw.unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    glfw.window_hint(glfw::WindowHint::Visible(false));
   
    #[cfg(target_os = "macos")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    let (mut window, _recv) = glfw.create_window(800, 600, "test window", glfw::WindowMode::Windowed).unwrap();

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
    glfw.set_swap_interval(glfw::SwapInterval::None);

    Ok(window)
}
