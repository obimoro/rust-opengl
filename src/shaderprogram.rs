use std::fs;

pub struct  Shader {
    ID: u32

}

impl Shader {

    // build and compile shader program
    pub fn shaderProgram(vertex_shader_source: &str, fragment_shader_source: &str) -> u32{
        // vertex shader
        let shader_source = fs::read_to_string(vertex_shader_source).unwrap();
        let c_str: *const i8 = shader_source.as_bytes().as_ptr() as *const i8;
        let vertex_shader = unsafe {
            let shader: u32 = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(shader, 1, &c_str, std::ptr::null());
            gl::CompileShader(shader);
            shader
        };
        
        // fragment shader
        let shader_source = fs::read_to_string(fragment_shader_source).unwrap();
        let c_str: *const i8 = shader_source.as_bytes().as_ptr() as *const i8;
        let fragment_shader = unsafe {
            let shader: u32 = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(shader, 1, &c_str, std::ptr::null());
            gl::CompileShader(shader);
            shader
        };
        
        let shader_Program = unsafe {
            let program = gl::CreateProgram();
            gl::AttachShader(program, vertex_shader);
            gl::AttachShader(program, fragment_shader);
            gl::LinkProgram(program);
            // check for linking erros
            let mut success = 0;
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
            if success == 0 {
                println!("ERROR::SHADER::PROGRAM::LINKING_FAILED")
            }
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
            return program;
        };
    }

    pub fn user_shader(&self) {
        unsafe {
            gl::UseProgram(self.ID)
        }
    }
}