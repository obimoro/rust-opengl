use crate::shaderprogram;


pub struct Cube {
    // Vertex data
    position: glam::Vec3,
    normal: glam::Vec3,
    TexCoords: glam::Vec2,

    vertices: [f32; 180],

            // VBO
        vbo: u32,
        // VAO
        vao: u32,
        // EBO
        ebo: u32,

    // Texture data
    //id: u32,
    //texture_type: str,

}

impl Cube {
    // defined a constant default position
    const DEFAULT_POSITION: glam::Vec3 = glam::Vec3::new(0.0, 0.0, 0.0);

    pub fn new() -> Self {
        let mut cube = Cube {
            position: Self::DEFAULT_POSITION,
            normal: glam::Vec3::new(0.0, 1.0, 0.0),
            TexCoords: glam::Vec2::new(1.0, 0.0),

   
            vbo: 0,
            vao: 0,
            ebo: 0,

              // cube vertex data
            vertices:[
            //   // position       // texture cords
                -0.5, -0.5, -0.5,  0.0, 0.0,
                0.5, -0.5, -0.5,  1.0, 0.0,
                0.5,  0.5, -0.5,  1.0, 1.0,
                0.5,  0.5, -0.5,  1.0, 1.0,
                -0.5,  0.5, -0.5,  0.0, 1.0,
                -0.5, -0.5, -0.5,  0.0, 0.0,
                
                -0.5, -0.5,  0.5,  0.0, 0.0,
                0.5, -0.5,  0.5,  1.0, 0.0,
                0.5,  0.5,  0.5,  1.0, 1.0,
                0.5,  0.5,  0.5,  1.0, 1.0,
                -0.5,  0.5,  0.5,  0.0, 1.0,
                -0.5, -0.5,  0.5,  0.0, 0.0,
                
                -0.5,  0.5,  0.5,  1.0, 0.0,
                -0.5,  0.5, -0.5,  1.0, 1.0,
                -0.5, -0.5, -0.5,  0.0, 1.0,
                -0.5, -0.5, -0.5,  0.0, 1.0,
                -0.5, -0.5,  0.5,  0.0, 0.0,
                -0.5,  0.5,  0.5,  1.0, 0.0,
                
                0.5,  0.5,  0.5,  1.0, 0.0,
                0.5,  0.5, -0.5,  1.0, 1.0,
                0.5, -0.5, -0.5,  0.0, 1.0,
                0.5, -0.5, -0.5,  0.0, 1.0,
                0.5, -0.5,  0.5,  0.0, 0.0,
                0.5,  0.5,  0.5,  1.0, 0.0,
                
                -0.5, -0.5, -0.5,  0.0, 1.0,
                0.5, -0.5, -0.5,  1.0, 1.0,
                0.5, -0.5,  0.5,  1.0, 0.0,
                0.5, -0.5,  0.5,  1.0, 0.0,
                -0.5, -0.5,  0.5,  0.0, 0.0,
                -0.5, -0.5, -0.5,  0.0, 1.0,

                -0.5,  0.5, -0.5,  0.0, 1.0,
                0.5,  0.5, -0.5,  1.0, 1.0,
                0.5,  0.5,  0.5,  1.0, 0.0,
                0.5,  0.5,  0.5,  1.0, 0.0,
                -0.5,  0.5,  0.5,  0.0, 0.0,
                -0.5,  0.5, -0.5,  0.0, 1.0
            ],
            

        };
        cube.setup_cube();
        cube
    
    }

    pub fn new_position(position: glam::Vec3) ->  Self{
        let mut cube = Cube {
            position,
            ..Self::new()
        };
        cube.setup_cube();
        cube
    }

    fn setup_cube(&mut self) {
        unsafe {
            gl::GenVertexArrays(1, &mut self.vao);
            gl::GenBuffers(1, &mut self.vbo);
            //gl::GenBuffers(1, &mut ebo);

            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);

            gl::BufferData(gl::ARRAY_BUFFER, (self.vertices.len() * std::mem::size_of::<f32>()) as isize,
            self.vertices.as_ptr() as *const std::ffi::c_void, gl::STATIC_DRAW);
            
            // gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            // gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (indices.len() * std::mem::size_of::<i32>()) as isize,
            //                                           indices.as_ptr() as *const std::ffi::c_void, gl::STATIC_DRAW);
            // position attribute
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 5 * std::mem::size_of::<f32>() as i32, std::ptr::null());
            // color attribute
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, 5 * std::mem::size_of::<f32>() as i32, (3 * std::mem::size_of::<f32>() as i32) as *const std::ffi::c_void);
            // texture attribute
            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, 5 * std::mem::size_of::<f32>() as i32, (3 * std::mem::size_of::<f32>() as i32) as *const std::ffi::c_void);

        }
    }

    pub fn draw(&mut self, shader: &shaderprogram::Shader) {
        // create model matrix
        let model = glam::Mat4::from_translation(self.position);
        let model_location = unsafe { gl::GetUniformLocation(shader.get_id(), std::ffi::CString::new("model").unwrap().as_ptr())};
        let model_data: [f32; 16] = model.to_cols_array();
        unsafe {
            gl::UniformMatrix4fv(model_location, 1, gl::FALSE, model_data.as_ptr() as *const f32);  
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0,36);
            gl::BindVertexArray(0);
        }
    }

    pub fn destroy(&mut self) {
        // Cleanup and housekeeping
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteBuffers(1, &self.ebo);
        }
    }

    pub fn get_pos(&mut self) -> glam::Vec3 {
        self.position
    }
}