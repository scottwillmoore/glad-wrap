use gl;
use gl::types::*;
use std;
use std::path::Path;

use shader::Shader;

pub struct Program {
    id: GLuint,
}

impl Program {
    pub fn new(shaders: &[Shader]) -> Result<Program, String> {
        let id = unsafe { gl::CreateProgram() };
        let program = Program { id };

        let success = program.link(shaders);

        if success {
            Ok(program)
        } else {
            Err(program.get_info_log())
        }
    }

    pub fn from_file<P>(shaders: &[P]) -> Result<Program, String>
    where
        P: AsRef<Path>,
    {
        let shaders = shaders
            .iter()
            .map(|s| Shader::from_file(s.as_ref()))
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        Program::new(shaders.as_ref())
    }

    pub fn get_id(&self) -> GLuint {
        self.id
    }

    pub fn bind(&self) {
        unsafe { gl::UseProgram(self.id) };
    }

    fn link(&self, shaders: &[Shader]) -> bool {
        for shader in shaders {
            unsafe { gl::AttachShader(self.id, shader.get_id()) };
        }

        unsafe { gl::LinkProgram(self.id) };

        let mut success = gl::FALSE as GLint;
        unsafe { gl::GetProgramiv(self.id, gl::LINK_STATUS, &mut success) };

        success == (gl::TRUE as GLint)
    }

    fn get_info_log(&self) -> String {
        let mut len = 0;
        unsafe { gl::GetProgramiv(self.id, gl::INFO_LOG_LENGTH, &mut len) };

        let mut info_log: Vec<u8> = std::iter::repeat(0).take(len as usize).collect();
        unsafe {
            gl::GetProgramInfoLog(
                self.id,
                len,
                std::ptr::null_mut(),
                info_log.as_mut_ptr() as *mut GLchar,
            )
        };

        String::from_utf8(info_log).expect("program info_log is not valid utf8")
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.id) };
    }
}
