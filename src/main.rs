// -------------------------------------------------------------
//
// Rustify https://learnopengl.com/
//
// -------------------------------------------------------------
use glfw::{Action, Context, Key, Window};
use glfw::ffi::glfwGetTime;
extern crate gl;
use gl::{types::*, Disable};
use glam;

mod shaderprogram;
mod texture;

// global values
const WIN_WIDTH: u32 = 1024;
const WIN_HEIGHT: u32 = 840;


// Main loop
fn main() {
    // init glfw
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
    
    // Create a windowed mode window and its OpenGL context
    let (mut window, events) = glfw.create_window(WIN_WIDTH, WIN_HEIGHT, "Welcome to openGL with Rust!", glfw::WindowMode::Windowed)
    .expect("Failed to create GLFW window.");
    

    // Make the window's context current
    window.make_current();
    window.set_key_polling(true);
    // Normal, Hidden and Disabled
    // Normal is a mouse that can exit window, hidden does the same but doesnt show cursor
    // Disabled lockes the mouse to the glfw window
    window.set_cursor_mode(glfw::CursorMode::Disabled);



    // init GL
    let gl = gl::load_with(|s| window.get_proc_address(s) as *const _);
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }
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

    // Timing 
    let mut deltaTime = 0.0;
    let mut lastFrame = 0.0;
    
    // cube vertex data
    let vertices: [f32; 180] = [
    //   // position     // colors      // texture cords
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
    ];

    // Cubes position in the world space
    let cubePositions: [glam::Vec3; 10] = [    
        glam::Vec3::new( 0.0,  0.0,  0.0), 
        glam::Vec3::new( 2.0,  5.0, -15.0), 
        glam::Vec3::new(-1.5, -2.2, -2.5),  
        glam::Vec3::new(-3.8, -2.0, -12.0),  
        glam::Vec3::new( 2.4, -0.4, -3.5),  
        glam::Vec3::new(-1.7,  3.0, -7.5),  
        glam::Vec3::new( 1.3, -2.0, -2.5),  
        glam::Vec3::new( 1.5,  2.0, -2.5), 
        glam::Vec3::new( 1.5,  0.2, -1.5), 
        glam::Vec3::new(-1.3,  1.0, -1.5)  
    ];

    // indices data
    // let indices: [i32; 6] = [
    //     0, 1, 2, // first triangle
    //     0, 2, 3 // second triangle
    //     ];
        
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
        //gl::GenBuffers(1, &mut ebo);
        // 1. bind vertex array object
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        // 2. set the vertex attributes pointers
        gl::BufferData(gl::ARRAY_BUFFER, (vertices.len() * std::mem::size_of::<f32>()) as isize,
        vertices.as_ptr() as *const std::ffi::c_void, gl::STATIC_DRAW);
       // gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        // gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (indices.len() * std::mem::size_of::<i32>()) as isize,
        //                                                 indices.as_ptr() as *const std::ffi::c_void, gl::STATIC_DRAW);
        // 3. set our vertex attributes pointers
        // position attribute
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 5 * std::mem::size_of::<f32>() as i32, std::ptr::null());
        gl::EnableVertexAttribArray(0);
        // color attribute
        // gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, 5 * std::mem::size_of::<f32>() as i32, (3 * std::mem::size_of::<f32>() as i32) as *const std::ffi::c_void);
        // gl::EnableVertexAttribArray(1);
        // texture attribute
        gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, 5 * std::mem::size_of::<f32>() as i32, (3 * std::mem::size_of::<f32>() as i32) as *const std::ffi::c_void);
        gl::EnableVertexAttribArray(1);

    }

    // Init shader
    let shader = shaderprogram::Shader::shader_program("./src/shaders/default.vert", "./src/shaders/default.frag");
    
    
    // init texture
     let texture1 = texture::Texture::new("./resources/texture/256_256.png").unwrap();
    //  let texture2 = texture::Texture::new("./resources/texture/rust_icon.png").unwrap();
    
     shader.use_shader();    
     shader.set_int("texture1", 0);
     //shader.set_int("texture2", 1);
     
     let mut cameraPos = glam::Vec3::new(0.0, 0.0, 3.0);
     let cameraFront = glam::Vec3::new(0.0, 0.0, -1.0);
     let cameraUp = glam::Vec3::new(0.0, 1.0, 0.0);

     // Loop until the user closes the window
     while !window.should_close() {
        let pos = window.get_cursor_pos();
        println!("{:?}", pos);
        let mut currentFrame = unsafe { glfw::ffi::glfwGetTime() as f32 };
        deltaTime = currentFrame - lastFrame;
        lastFrame = currentFrame;
         
        let mut model = glam::Mat4::IDENTITY;
        //model = model * glam::Mat4::from_rotation_translation(glam::Quat::from_axis_angle(glam::Vec3::X, -55.0), glam::Vec3::new(1.0, 0.0,0.0));
        //model = model * glam::Mat4::from_rotation_x(-55.0_f32.to_radians());
        model = model * glam::Mat4::from_rotation_x((deltaTime * 50.0_f32.to_radians()));
        model = model * glam::Mat4::from_rotation_y((deltaTime * 50.0_f32.to_radians()));
        let mut view = glam::Mat4::IDENTITY;
        //view = view * glam::Mat4::from_translation(glam::Vec3::new(0.0, 0.0, -3.0));
        view = glam::Mat4::look_at_rh(cameraPos,cameraPos + cameraFront, cameraUp);
        
        let mut projection = glam::Mat4::IDENTITY;
        // rh stands for right hand
        projection = projection * glam::Mat4::perspective_rh(60.0_f32.to_radians(), (WIN_WIDTH / WIN_HEIGHT) as f32, 0.1, 100.0);
        //  let mut vec = glam::Vec4::new(1.0, 0.0, 0.0, 1.0);
        //  let mut trans = glam::Mat4::IDENTITY;
        //  trans = trans * glam::Mat4::from_transla<tion(glam::Vec3::new(0.5, -0.5, 0.0));
        //  trans = trans * glam::Mat4::from_rotation_translation(glam::Quat::from_axis_angle(glam::Vec3::Z, time), glam::Vec3::new(0.0, 0.0, 1.0));
        //  trans = trans * glam::Mat4::from_scale(glam::Vec3::new(0.5, 0.5, 0.5));
        //  vec = trans * vec;   
        //println!("x: {}, y: {} z: {}, w: {}", vec.x, vec.y, vec.z, vec.w);
        
        // input
        for (_, event)  in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }
        
        process_input(&mut window, &mut cameraPos, &cameraFront, &cameraUp, &deltaTime);
        
        // resize viewport if window is resized
        window.set_framebuffer_size_callback(|_, width, height| {
            unsafe {
                gl::Viewport(0,0, width as i32, height as i32);
            }
        });
        
        let modelLoc = unsafe { gl::GetUniformLocation(shader.get_id(), std::ffi::CString::new("model").unwrap().as_ptr()) };
        let viewLoc = unsafe { gl::GetUniformLocation(shader.get_id(), "view\0".as_ptr() as *const i8) };
        // Rendering commands here
        unsafe {
            // default clear color
            gl::ClearColor(1.0, 0.5, 0.0, 1.0);
            // Clear the window
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            // link shaders
            
            shader.use_shader();        
            // bind texture 
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture1.get_id());
            // gl::ActiveTexture(gl::TEXTURE1);
            // gl::BindTexture(gl::TEXTURE_2D, texture2.get_id());
            let model_data: [f32; 16] = model.to_cols_array();
            gl::UniformMatrix4fv(modelLoc, 1, gl::FALSE, model_data.as_ptr() as *const f32);  
            let view_data: [f32; 16] = view.to_cols_array();
            gl::UniformMatrix4fv(viewLoc, 1, gl::FALSE, view_data.as_ptr() as *const f32);
            shader.set_mat4("projection",projection);
            //shader.use_shader();
            // bind vertex array object
            gl::BindVertexArray(vao);
            for x in 0..cubePositions.len() {
                model = glam::Mat4::IDENTITY;
                model = model * glam::Mat4::from_translation(cubePositions[x]);
                let mut angle = 25.0 * x as f32;
                model = model * glam::Mat4::from_rotation_x(angle.to_radians());
                model = model * glam::Mat4::from_rotation_y(angle.to_radians());
                model = model * glam::Mat4::from_rotation_z(angle.to_radians());
                
                if x % 3 == 0 {
                    angle = 25.0 *  glfw::ffi::glfwGetTime() as f32 ;
                    model = model * glam::Mat4::from_axis_angle((glam::Vec3::new(1.0, 0.3, 0.5)).normalize(), angle.to_radians());
                }


                shader.set_mat4("model",model);
                gl::DrawArrays(gl::TRIANGLES, 0,36);
            }
            // Draw triangle
            //  gl::DrawArrays(gl::TRIANGLES, 0, 3);
            // Draw a square
             //gl::DrawElements(gl::TRIANGLES, indices.len() as i32, gl::UNSIGNED_INT,std::ptr::null());
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

fn process_input(window: &mut glfw::Window, cameraPos: &mut glam::Vec3, cameraFront: &glam::Vec3, cameraUp: &glam::Vec3, deltaTime: &f32) {
    let cameraSpeed: f32 = 2.5 * *deltaTime;
    if window.get_key(glfw::Key::W) == glfw::Action::Press {
        *cameraPos += cameraSpeed * *cameraFront;
    }
    if window.get_key(glfw::Key::S) == glfw::Action::Press {
        *cameraPos -= cameraSpeed * *cameraFront;
    }
    if window.get_key(glfw::Key::A) == glfw::Action::Press {
        *cameraPos -= cameraSpeed * (*cameraFront).cross(*cameraUp).normalize();
    }
    if window.get_key(glfw::Key::D) == glfw::Action::Press {
        *cameraPos += cameraSpeed * (*cameraFront).cross(*cameraUp).normalize();
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

fn mouse_callback(xpos: f64,ypos: f64) {
    let firstMouse = true;

    if firstMouse {
        let lastX = xpos;
        let lastY = ypos;
    }

}

