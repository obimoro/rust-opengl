// -------------------------------------------------------------
//
// Rustify https://learnopengl.com/
//
// -------------------------------------------------------------
use glfw::{Action, Context, Key, WindowEvent};
use gl::types::*;



const WIN_WIDTH: u32 = 640;
const WIN_HEIGHT: u32 = 480;

const VERTEX_SHADER_SOURCE: &str = "#version 330 core
layout (location = 0) in vec3 aPos;
void main()
{
    gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
}";

const FRAGMENT_SHADER_SOURCE: &str = "#version 330 core
out vec4 FragColor;
void main()
{
    FragColor = vec4(0.0f, 0.0f, 1.0f, 1.0f);
}";

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
    
    // set the viewport for openGL
    unsafe {
        gl::Viewport(0, 0,WIN_WIDTH as i32, WIN_HEIGHT as i32);
    }
    
    // build and compile shader program
    // vertex shader
    let vertex_shader = unsafe {
        let shader: u32 = gl::CreateShader(gl::VERTEX_SHADER);
        let c_str_vert = std::ffi::CString::new(VERTEX_SHADER_SOURCE).unwrap();
        gl::ShaderSource(shader, 1, &c_str_vert.as_ptr(), std::ptr::null());
        gl::CompileShader(shader);
        shader
    };
    
    // fragment shader
    let fragment_shader = unsafe {
        let shader: u32 = gl::CreateShader(gl::FRAGMENT_SHADER);
        let c_str_vert = std::ffi::CString::new(FRAGMENT_SHADER_SOURCE).unwrap();
        gl::ShaderSource(shader, 1, &c_str_vert.as_ptr(), std::ptr::null());
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
        program
    };
    
    // vertex data
    let vertices: [f32; 12] = [ 
         0.5, 0.5, 0.0,
         0.5,-0.5, 0.0,
        -0.5,-0.5, 0.0,
        -0.5, 0.5, 0.0];
    let indices: [i32; 6] = [
        0, 1, 3,
        1, 2, 3
    ];
    
    // VBO
    let mut vbo: u32 = 0;
    unsafe {
        // 0. copy vertices array in a buffer for openGL to use
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        // 1. set the vertex attributes pointers
        gl::BufferData(gl::ARRAY_BUFFER, (vertices.len() * std::mem::size_of::<f32>()) as isize,
        vertices.as_ptr() as *const std::ffi::c_void, gl::STATIC_DRAW);
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 
                            3 * std::mem::size_of::<f32>() as i32, std::ptr::null());
        gl::EnableVertexAttribArray(0);
        // 2. use shader program when we want to render an object
        gl::UseProgram(shader_Program);
    }
    
    // EBO
    let mut ebo: u32 = 0;
    unsafe {
        gl::GenBuffers(1, &mut ebo);
    }

    // VAO
    let mut vao: u32 = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        // 1. bind vertex array object
        gl::BindVertexArray(vao);
        // 2. copy vertices array in a buffer for openGL to use
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER, (vertices.len() * std::mem::size_of::<f32>()) as isize, vertices.as_ptr() as *const std::ffi::c_void, gl::STATIC_DRAW);
        // 3. set our vertex attributes pointers

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (indices.len() * std::mem::size_of::<i32>()) as isize, indices.as_ptr() as *const std::ffi::c_void, gl::STATIC_DRAW);

        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * std::mem::size_of::<f32>() as i32, std::ptr::null());
        gl::EnableVertexAttribArray(0);
    }

    
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
            gl::UseProgram(shader_Program);
            // bind vertex array object
            gl::BindVertexArray(vao);
            // Draw triangle
            // gl::DrawArrays(gl::TRIANGLES, 0, 3);
            // Draw rect
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
            
        }
        
        
        
        // Poll for and process events
        glfw.poll_events();
        // swap front and back buffers
        window.swap_buffers();
    }

    // Cleanup and housekeeping
    
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


