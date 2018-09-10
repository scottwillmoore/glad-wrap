use gl::types::*;

// Note this requires OpenGL 4.3+ or extensions ARB_debug_output, AMD_debug_output.
// pub fn register_gl_error_handler() {
//     gl::Enable(gl::DEBUG_OUTPUT);
//     gl::DebugMessageCallback(gl_error_callback, std::ptr::null());
// }

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! assert_no_gl_error {
    () => {
        $crate::assert_no_gl_error(file!(), line!());
    };
}

#[cfg(not(debug_assertions))]
macro_rules! assert_no_gl_error { {} => {}; }

pub fn check_gl_error() -> Option<(GLenum, &'static str)> {
    let error = unsafe { gl::GetError() };
    if error == gl::NO_ERROR {
        return None;
    }

    let message = match error {
        gl::INVALID_ENUM => "INVALID_ENUM",
        gl::INVALID_VALUE => "INVALID_VALUE",
        gl::INVALID_OPERATION => "INVALID_OPERATION",
        gl::STACK_OVERFLOW => "STACK_OVERFLOW",
        gl::STACK_UNDERFLOW => "STACK_UNDERFLOW",
        gl::OUT_OF_MEMORY => "OUT_OF_MEMORY",
        gl::INVALID_FRAMEBUFFER_OPERATION => "INVALID_FRAMEBUFFER_OPERATION",
        gl::CONTEXT_LOST => "CONTEXT_LOST",
        _ => "UNKNOWN",
    };

    Some((error, message))
}

pub fn assert_no_gl_error(file: &str, line: u32) {
    if let Some((error, message)) = check_gl_error() {
        panic!(
            "an OpenGL error has occurred: {} ({}) at {}:{}",
            message, error, file, line
        );
    }
}
