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

        Self::check_compile_erros(vertex,"VERTEX");
        
        // fragment shader
        let f_shader_code: *const i8 = f_shader_file.as_bytes().as_ptr() as *const i8;
        let fragment = unsafe {
            let fragment: u32 = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(fragment, 1, &f_shader_code, std::ptr::null());
            gl::CompileShader(fragment);
            fragment
        };
        Self::check_compile_erros(fragment,"FRAGMENT");
        
        let program = unsafe { gl::CreateProgram() };
        unsafe {
            gl::AttachShader(program, vertex);
            gl::AttachShader(program, fragment);
            gl::LinkProgram(program);
        };

        // check for linking erros
        Self::check_compile_erros(program,"PROGRAM");

            unsafe {
                gl::DeleteShader(vertex);
                gl::DeleteShader(fragment);
            };
        //return program;
        Shader { id: program }

        
    }

    fn check_compile_erros(shader:u32, type_: &str) {
        let mut success = 0;
        let mut info_log  = [0; 1024];

        if type_ != "PROGRAM" {
            unsafe {
                gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
                if success == 0 {
                    gl::GetShaderInfoLog(shader, 1024, std::ptr::null_mut(), info_log.as_mut_ptr());
                    let info_log_str = std::ffi::CStr::from_ptr(info_log.as_ptr()).to_str().unwrap_or("Failed to decode info log");
                    println!("ERROR::SHADER_COMPILATION_ERROR of type: {}\n{}\n -- --------------------------------------------------- -- ", type_, info_log_str);
                }
            }
        } else {
            unsafe {
                gl::GetProgramiv(shader, gl::LINK_STATUS, &mut success);
                if success == 0 {
                    gl::GetProgramInfoLog(shader, 1024, std::ptr::null_mut(), info_log.as_mut_ptr());
                     let info_log_str = std::ffi::CStr::from_ptr(info_log.as_ptr()).to_str().unwrap_or("Failed to decode info log");
                    println!("ERROR::PROGRAM_LINKING_ERROR of type: {}\n{}\n -- --------------------------------------------------- -- ", type_, info_log_str);
                }
            }
        }
    }

    pub fn use_shader(&self) {
        unsafe {
            gl::UseProgram(self.id)
        }
    }

    pub fn get_id(&self)  -> u32 {
        self.id
    }

    pub fn set_float(&self, name: &str, value: f32) {
        let location = self.get_uniform_location(name);
        unsafe { gl::Uniform1f(location, value) };
    }

    pub fn set_int(&self, name: &str, value: i32) {
        let location = self.get_uniform_location(name);
        unsafe { gl::Uniform1i(location, value) };
    }

    pub fn get_uniform_location(&self, name: &str) -> i32 {
        let c_str = std::ffi::CString::new(name).unwrap();
        let location = unsafe { gl::GetUniformLocation(self.id, c_str.as_ptr()) };
        location
    }

}