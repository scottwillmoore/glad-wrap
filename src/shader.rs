use gl::types::*;
use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::Read;
use std::path::Path;

// https://codereview.stackexchange.com/questions/175001/modern-opengl-wrapper-abstractions-shaders

#[derive(Debug, Clone, Copy)]
pub enum ShaderType {
    Fragment,
    Geometry,
    Vertex,
}

impl Into<GLenum> for ShaderType {
    fn into(self) -> GLenum {
        use crate::ShaderType::*;
        match self {
            Fragment => gl::FRAGMENT_SHADER,
            Geometry => gl::GEOMETRY_SHADER,
            Vertex => gl::VERTEX_SHADER,
        }
    }
}

pub struct Shader {
    id: GLuint,
}

impl Shader {
    pub fn new(source: &CStr, kind: ShaderType) -> Result<Shader, String> {
        let id = unsafe { gl::CreateShader(kind.into()) };
        let shader = Shader { id };

        let success = shader.compile(&source);
        match success {
            true => Ok(shader),
            false => Err(shader.get_info_log()),
        }
    }

    pub fn from_str(source: &str, kind: ShaderType) -> Result<Shader, String> {
        let c_string = CString::new(source).unwrap();
        Shader::new(&c_string, kind)
    }

    pub fn from_file<P>(path: P) -> Result<Shader, String>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();

        let mut file = File::open(path).unwrap();
        let mut source = Vec::new();
        file.read_to_end(&mut source).unwrap();

        let c_string = CString::new(source).unwrap();

        let extension = path
            .extension()
            .and_then(|s| s.to_str())
            .map_or("".to_string(), |s| s.to_ascii_lowercase());

        let kind = match extension.as_str() {
            "fs" => ShaderType::Fragment,
            "vs" => ShaderType::Vertex,
            _ => {
                return Err(format!(
                    "cannot determine ShaderType from the file extension: {}",
                    extension
                ));
            }
        };

        Shader::new(&c_string, kind)
    }

    pub fn get_id(&self) -> GLuint {
        self.id
    }

    fn compile(&self, source: &CStr) -> bool {
        unsafe {
            gl::ShaderSource(self.id, 1, &source.as_ptr(), std::ptr::null());
            gl::CompileShader(self.id);
        }

        let mut success = gl::FALSE as GLint;
        unsafe { gl::GetShaderiv(self.id, gl::COMPILE_STATUS, &mut success) };

        success == (gl::TRUE as GLint)
    }

    fn get_info_log(&self) -> String {
        let mut len = 0;
        unsafe { gl::GetShaderiv(self.id, gl::INFO_LOG_LENGTH, &mut len) };

        let mut info_log: Vec<u8> = std::iter::repeat(0).take(len as usize).collect();
        unsafe {
            gl::GetShaderInfoLog(
                self.id,
                len,
                std::ptr::null_mut(),
                info_log.as_mut_ptr() as *mut GLchar,
            )
        };

        String::from_utf8(info_log).expect("shader info_log is not valid utf8")
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { gl::DeleteShader(self.id) };
    }
}
