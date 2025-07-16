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
    const DEFAULT_POSITION: glam::Vec3 = glam::Vec3::new(3.0, 0.0, 3.0);

    pub fn new() -> Self {
        Cube {
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

        }
    
    }

    pub fn new_position(position: glam::Vec3) ->  Self{
        Cube {
            position,
            ..Self::new()
        }
    }

    pub fn setup_cube(&mut self) {
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

    pub fn render(&mut self) {

        unsafe {

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
}