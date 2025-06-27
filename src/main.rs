// -------------------------------------------------------------
//
// Rustify https://learnopengl.com/
//
// -------------------------------------------------------------
use glfw::{Action, Context, Key};
use glfw::ffi::glfwGetTime;
extern crate gl;
use gl::types::*;


mod shaderprogram;
mod texture;


// global values
const WIN_WIDTH: u32 = 1024;
const WIN_HEIGHT: u32 = 840;

// Main loop
fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

    // Create a windowed mode window and its OpenGL context
    let (mut window, events) = glfw.create_window(WIN_WIDTH, WIN_HEIGHT, "Welcome to openGL with Rust!", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    // Make the window's context current
    window.make_current();
    window.set_key_polling(true);

    // init GL
    let gl = gl::load_with(|s| window.get_proc_address(s) as *const _);
    
    // Max nr of vertex supported
    let mut nr_attributes = 0;
    unsafe {
        gl::GetIntegerv(gl::MAX_VERTEX_ATTRIBS, &mut nr_attributes);
    }
    println!("Maximum nr of vertex attributes supported: {}", nr_attributes);

    // set the viewport for openGL
    unsafe {
        gl::Viewport(0, 0,WIN_WIDTH as i32, WIN_HEIGHT as i32);
    }
    
    // vertex data
    let vertices: [f32; 32] = [ 
        // position     // colors      // texture cords
         0.5, 0.5, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, // top right
         0.5,-0.5, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, // bottom right
        -0.5,-0.5, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, // bottom left
        -0.5, 0.5, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0];// top left

    // indices data
    let indices: [i32; 6] = [
        0, 1, 2, // first triangle
        0, 2, 3 // second triangle
    ];
    
    // VBO
    let mut vbo: u32 = 0;
    
    // VAO
    let mut vao: u32 = 0;

    // EBO
    let mut ebo: u32 = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        // 0. copy vertices array in a buffer for openGL to use
        gl::GenBuffers(1, &mut vbo);
        gl::GenBuffers(1, &mut ebo);
        // 1. bind vertex array object
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        // 2. set the vertex attributes pointers
        gl::BufferData(gl::ARRAY_BUFFER, (vertices.len() * std::mem::size_of::<f32>()) as isize,
        vertices.as_ptr() as *const std::ffi::c_void, gl::STATIC_DRAW);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (indices.len() * std::mem::size_of::<i32>()) as isize,
                                                        indices.as_ptr() as *const std::ffi::c_void, gl::STATIC_DRAW);
        // 3. set our vertex attributes pointers
        // position attribute
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 8 * std::mem::size_of::<f32>() as i32, std::ptr::null());
        gl::EnableVertexAttribArray(0);
        // color attribute
        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, 8 * std::mem::size_of::<f32>() as i32, (3 * std::mem::size_of::<f32>() as i32) as *const std::ffi::c_void);
        gl::EnableVertexAttribArray(1);
        // texture attribute
        gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, 8 * std::mem::size_of::<f32>() as i32, (6 * std::mem::size_of::<f32>() as i32) as *const std::ffi::c_void);
        gl::EnableVertexAttribArray(2);

    }

    // Init shader
    let shader = shaderprogram::Shader::shader_program("./src/shaders/default.vert", "./src/shaders/default.frag");
    
    
    // init texture
     let texture1 = texture::Texture::new("./resources/texture/container.jpg").unwrap();
     let texture2 = texture::Texture::new("./resources/texture/awesomeface.png").unwrap();
    

    shader.set_int("texture1", 0);
    shader.set_int("texture2", 1);

    
    // Loop until the user closes the window
    while !window.should_close() {
        // input
        for (_, event)  in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }
        
        // resize viewport if window is resized
        window.set_framebuffer_size_callback(|_, width, height| {
            unsafe {
                gl::Viewport(0,0, width as i32, height as i32);
            }
        });
        
        // Rendering commands here
        unsafe {
            // default clear color
            gl::ClearColor(1.0, 0.5, 0.0, 1.0);
            // Clear the window
            gl::Clear(gl::COLOR_BUFFER_BIT);
            // link shaders
            
            // bind texture 
            // gl::ActiveTexture(gl::TEXTURE0);
            // gl::BindTexture(gl::TEXTURE_2D, texture1.get_id());
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, texture2.get_id());
            shader.use_shader();
            // bind vertex array object
            gl::BindVertexArray(vao);
            // Draw triangle
            //  gl::DrawArrays(gl::TRIANGLES, 0, 3);
            // Draw a square
             gl::DrawElements(gl::TRIANGLES, indices.len() as i32, gl::UNSIGNED_INT,std::ptr::null());

        }

        // Poll for and process events
        glfw.poll_events();
        // swap front and back buffers
        window.swap_buffers();
    }

    // Cleanup and housekeeping
    unsafe {
        gl::DeleteVertexArrays(1, &vao);
        gl::DeleteBuffers(1, &vbo);
        gl::DeleteBuffers(1, &ebo);
    }
    
}


fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true);
        }
        glfw::WindowEvent::Key(Key::Num1, _, Action::Press, _) => {
            unsafe {
                gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
            }
        }
        glfw::WindowEvent::Key(Key::Num2, _, Action::Press, _) => {
            unsafe {
                gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
            }
        }
        _ => {}
    }
}


