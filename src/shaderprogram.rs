use std::fs;

pub struct  Shader {
    id: u32
}

impl Shader {

    // build and compile shader program
    pub fn shader_program(vertexpath: &str, fragmentpath: &str) -> Self{
        // vertex shader
        // read file
        let v_shader_file = fs::read_to_string(vertexpath).expect("Error reading vertex shader file");
        let f_shader_file = fs::read_to_string(fragmentpath).expect("Error reading fragment shader file");


        let v_shader_code: *const i8 = v_shader_file.as_bytes().as_ptr() as *const i8;
        let vertex = unsafe {
            let vertex: u32 = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vertex, 1, &v_shader_code, std::ptr::null());
            gl::CompileShader(vertex);
            vertex
        };
        
        // fragment shader
        let f_shader_code: *const i8 = f_shader_file.as_bytes().as_ptr() as *const i8;
        let fragment = unsafe {
            let fragment: u32 = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(fragment, 1, &f_shader_code, std::ptr::null());
            gl::CompileShader(fragment);
            fragment
        };
        
        let program = unsafe { gl::CreateProgram() };
        unsafe {
            gl::AttachShader(program, vertex);
            gl::AttachShader(program, fragment);
            gl::LinkProgram(program);
        };
            // check for linking erros
            let mut success = 0;
            unsafe { gl::GetProgramiv(program, gl::LINK_STATUS, &mut success) };
            if success == 0 {
                println!("ERROR::SHADER::PROGRAM::LINKING_FAILED")
            }
            unsafe {
                gl::DeleteShader(vertex);
                gl::DeleteShader(fragment);
            };
        //return program;
        Shader { id: program }
    }

    pub fn use_shader(&self) {
        unsafe {
            gl::UseProgram(self.id)
        }
    }

    pub fn set_float(&self, name: &str, value: f32) {
        let location = self.get_uniform_location(name);
        unsafe { gl::Uniform1f(location, value) };
    }

    pub fn set_int(&self, name: &str, value: i32) {
        let location = self.get_uniform_location(name);
        unsafe { gl::Uniform1i(location, value) };
    }

    fn get_uniform_location(&self, name: &str) -> i32 {
        let c_str = std::ffi::CString::new(name).unwrap();
        let location = unsafe { gl::GetUniformLocation(self.id, c_str.as_ptr()) };
        location
    }

}