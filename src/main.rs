use std::char::CharTryFromError;

// -------------------------------------------------------------
//
// Rustify https://learnopengl.com/
//
// -------------------------------------------------------------
use glfw::{ffi::{glfwSetCursorPosCallback, glfwSetScrollCallback}, Action, Context, Key};
extern crate gl;
use glam;
use log::{debug, error, log_enabled, info, Level};


mod constants;
mod shaderprogram;
mod texture;
mod camera;
mod cube;


// global values
use constants::{WIN_WIDTH, WIN_HEIGHT};


// Main loop
fn main() {
    // init logger
    env_logger::builder()
    .default_format()
    .format_line_number(true)
    .init();

    // init glfw
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
    info!("Init glfw");
    
    // Create a windowed mode window and its OpenGL context
    let (mut window, events) = glfw.create_window(WIN_WIDTH, WIN_HEIGHT, "Welcome to openGL with Rust!", glfw::WindowMode::Windowed)
                                                                .expect("msg");
    

    // Make the window's context current
    window.make_current();
    window.set_key_polling(true);
    // Normal, Hidden and Disabled
    // Normal is a mouse that can exit window, hidden does the same but doesnt show cursor
    // Disabled lockes the mouse to the glfw window
    window.set_cursor_mode(glfw::CursorMode::Disabled);
    // Test to see if device supports raw mouse motion
    // Raw mouse motion is better for controlling for example a 3D camera. 
    // cursor needs to be disabled
    let raw_mouse_support = unsafe {glfw::ffi::glfwRawMouseMotionSupported()};

    let enable_raw_mouse_motion = match raw_mouse_support {
        0 => {
            info!("Raw mouse motion is not supported");
            false
        },
        1 => {
            info!("Raw mouse motion is supported");
            true
        },
        _ => {
            info!("Unkown raw mouse motion support status");
            false
        },
    };
    window.set_raw_mouse_motion(enable_raw_mouse_motion);

    // init GL
    let _gl = gl::load_with(|s| window.get_proc_address(s) as *const _);
    unsafe {
        gl::Enable(gl::DEPTH_TEST); 

    }
    // Max nr of vertex supported
    let mut nr_attributes = 0;
    unsafe {
        gl::GetIntegerv(gl::MAX_VERTEX_ATTRIBS, &mut nr_attributes);
    }
    info!("Maximum nr of vertex attributes supported: {}", nr_attributes);

    // set the viewport for openGL
    unsafe {
        gl::Viewport(0, 0, WIN_WIDTH as i32, WIN_HEIGHT as i32);
    }


    // Timing 
    let mut last_frame = 0.0;
    
    // cube vertex data
    // let vertices: [f32; 180] = [
    // //   // position       // texture cords
    //     -0.5, -0.5, -0.5,  0.0, 0.0,
    //      0.5, -0.5, -0.5,  1.0, 0.0,
    //      0.5,  0.5, -0.5,  1.0, 1.0,
    //      0.5,  0.5, -0.5,  1.0, 1.0,
    //     -0.5,  0.5, -0.5,  0.0, 1.0,
    //     -0.5, -0.5, -0.5,  0.0, 0.0,
        
    //     -0.5, -0.5,  0.5,  0.0, 0.0,
    //      0.5, -0.5,  0.5,  1.0, 0.0,
    //      0.5,  0.5,  0.5,  1.0, 1.0,
    //      0.5,  0.5,  0.5,  1.0, 1.0,
    //     -0.5,  0.5,  0.5,  0.0, 1.0,
    //     -0.5, -0.5,  0.5,  0.0, 0.0,
        
    //     -0.5,  0.5,  0.5,  1.0, 0.0,
    //     -0.5,  0.5, -0.5,  1.0, 1.0,
    //     -0.5, -0.5, -0.5,  0.0, 1.0,
    //     -0.5, -0.5, -0.5,  0.0, 1.0,
    //     -0.5, -0.5,  0.5,  0.0, 0.0,
    //     -0.5,  0.5,  0.5,  1.0, 0.0,
        
    //      0.5,  0.5,  0.5,  1.0, 0.0,
    //      0.5,  0.5, -0.5,  1.0, 1.0,
    //      0.5, -0.5, -0.5,  0.0, 1.0,
    //      0.5, -0.5, -0.5,  0.0, 1.0,
    //      0.5, -0.5,  0.5,  0.0, 0.0,
    //      0.5,  0.5,  0.5,  1.0, 0.0,
        
    //     -0.5, -0.5, -0.5,  0.0, 1.0,
    //      0.5, -0.5, -0.5,  1.0, 1.0,
    //      0.5, -0.5,  0.5,  1.0, 0.0,
    //      0.5, -0.5,  0.5,  1.0, 0.0,
    //     -0.5, -0.5,  0.5,  0.0, 0.0,
    //     -0.5, -0.5, -0.5,  0.0, 1.0,

    //     -0.5,  0.5, -0.5,  0.0, 1.0,
    //      0.5,  0.5, -0.5,  1.0, 1.0,
    //      0.5,  0.5,  0.5,  1.0, 0.0,
    //      0.5,  0.5,  0.5,  1.0, 0.0,
    //     -0.5,  0.5,  0.5,  0.0, 0.0,
    //     -0.5,  0.5, -0.5,  0.0, 1.0
    // ];

    // Cubes position in the world space
    // let cube_positions: [glam::Vec3; 10] = [    
    //     glam::Vec3::new( 0.0,  0.0,  0.0), 
    //     glam::Vec3::new( 2.0,  5.0, -15.0), 
    //     glam::Vec3::new(-1.5, -2.2, -2.5),  
    //     glam::Vec3::new(-3.8, -2.0, -12.0),  
    //     glam::Vec3::new( 2.4, -0.4, -3.5),  
    //     glam::Vec3::new(-1.7,  3.0, -7.5),  
    //     glam::Vec3::new( 1.3, -2.0, -2.5),  
    //     glam::Vec3::new( 1.5,  2.0, -2.5), 
    //     glam::Vec3::new( 1.5,  0.2, -1.5), 
    //     glam::Vec3::new(-1.3,  1.0, -1.5)  
    // ];

    // indices data
    // let indices: [i32; 6] = [
    //     0, 1, 2, // first triangle
    //     0, 2, 3 // second triangle
    //     ];
        
    // VBO
    // let mut vbo: u32 = 0;
    
    // // VAO
    // let mut vao: u32 = 0;

    // // EBO
    // let ebo: u32 = 0;
    // unsafe {
    //     gl::GenVertexArrays(1, &mut vao);
    //     // 0. copy vertices array in a buffer for openGL to use
    //     gl::GenBuffers(1, &mut vbo);
    //     //gl::GenBuffers(1, &mut ebo);
    //     // 1. bind vertex array object
    //     gl::BindVertexArray(vao);
    //     gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
    //     // 2. set the vertex attributes pointers
    //     gl::BufferData(gl::ARRAY_BUFFER, (vertices.len() * std::mem::size_of::<f32>()) as isize,
    //     vertices.as_ptr() as *const std::ffi::c_void, gl::STATIC_DRAW);
    //    // gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
    //     // gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (indices.len() * std::mem::size_of::<i32>()) as isize,
    //     //                                                 indices.as_ptr() as *const std::ffi::c_void, gl::STATIC_DRAW);
    //     // 3. set our vertex attributes pointers
    //     // position attribute
    //     gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 5 * std::mem::size_of::<f32>() as i32, std::ptr::null());
    //     gl::EnableVertexAttribArray(0);
    //     // color attribute
    //     // gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, 5 * std::mem::size_of::<f32>() as i32, (3 * std::mem::size_of::<f32>() as i32) as *const std::ffi::c_void);
    //     // gl::EnableVertexAttribArray(1);
    //     // texture attribute
    //     gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, 5 * std::mem::size_of::<f32>() as i32, (3 * std::mem::size_of::<f32>() as i32) as *const std::ffi::c_void);
    //     gl::EnableVertexAttribArray(1);

    // }

    // Init shader
    let shader = shaderprogram::Shader::shader_program("./src/shaders/default.vert", "./src/shaders/default.frag");
    
    // init texture
    let texture1 = texture::Texture::new("./resources/texture/512_512.png").unwrap();
    //  let texture2 = texture::Texture::new("./resources/texture/rust_icon.png").unwrap();
     shader.use_shader();    
     shader.set_int("texture1", 0);
     //shader.set_int("texture2", 1);
     
    // init camera
    let mut camera = camera::Camera::new();
    //window.set_cursor_pos_callback(camera.mouse_callback(&mut window));
    //camera.scroll_callback(&mut window);
    
    // Set up a scroll callback

    // window.set_scroll_callback(move |_, _, yoffset| {
    //     println!("{}, {}", yoffset, camera.get_fov());

    //     // Handle the scroll event here
    // });

    // let mut cube1 = cube::Cube::new();
    // cube1.setup_cube();
    let mut cube2 = cube::Cube::new();
    cube2.setup_cube();
    

    // Loop until the user closes the window
    while !window.should_close() {

        let current_frame = unsafe { glfw::ffi::glfwGetTime() as f32 };
        let delta_time = current_frame - last_frame;
        last_frame = current_frame;
        
        // view
        //let mut view = glam::Mat4::IDENTITY;
        //view = view * glam::Mat4::from_translation(glam::Vec3::new(0.0, 0.0, -3.0));
        let view = glam::Mat4::look_at_rh(camera.get_position(),camera.get_position() + camera.get_front(), camera.get_up());
        
        let mut projection = glam::Mat4::IDENTITY;
        // rh stands for right hand
        projection = projection * glam::Mat4::perspective_rh(camera.get_fov().to_radians(), (WIN_WIDTH / WIN_HEIGHT) as f32, 0.1, 100.0);

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
        
        // Gets mouse position to feed to mouse callback function
        // makes so you can look around
        let mouse_pos = glfw::Window::get_cursor_pos(&mut window);
        camera.mouse_callback(mouse_pos.0, mouse_pos.1);
        camera.process_input(&mut window, &delta_time);

        // resize viewport if window is resized
        window.set_framebuffer_size_callback(|_, width, height| {
            unsafe {
                gl::Viewport(0,0, width as i32, height as i32);
            }
        });

        
        let model_location = unsafe { gl::GetUniformLocation(shader.get_id(), std::ffi::CString::new("model").unwrap().as_ptr()) };
        let view_location = unsafe { gl::GetUniformLocation(shader.get_id(), "view\0".as_ptr() as *const i8) };
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
            let mut model = glam::Mat4::IDENTITY;
            let model_data: [f32; 16] = model.to_cols_array();
            gl::UniformMatrix4fv(model_location, 1, gl::FALSE, model_data.as_ptr() as *const f32);  
            let view_data: [f32; 16] = view.to_cols_array();
            gl::UniformMatrix4fv(view_location, 1, gl::FALSE, view_data.as_ptr() as *const f32);
            shader.set_mat4("projection",projection);
            //shader.use_shader();
            // bind vertex array object
            //gl::BindVertexArray(vao);

            // Render goes here
            cube2.render();
            // for x in 0..cube_positions.len() {
            //     model = glam::Mat4::IDENTITY;
            //     model = model * glam::Mat4::from_translation(cube_positions[x]);
            //     let mut angle = 25.0 * x as f32;
            //     model = model * glam::Mat4::from_rotation_x(angle.to_radians());
            //     model = model * glam::Mat4::from_rotation_y(angle.to_radians());
            //     model = model * glam::Mat4::from_rotation_z(angle.to_radians());
                
            //     if x % 3 == 0 {
            //         angle = 25.0 *  glfw::ffi::glfwGetTime() as f32 ;
            //         model = model * glam::Mat4::from_axis_angle((glam::Vec3::new(1.0, 0.3, 0.5)).normalize(), angle.to_radians());
                
            //     }

            //     shader.set_mat4("model",model);
            //     gl::DrawArrays(gl::TRIANGLES, 0,36);

            // }
            

            // for x in 0..20 {
            //     for y in 0..20 {
            //         for z in 0..20{

            //             if x % 2 == 0 {
            //                 if y % 2 == 0 { 
            //                     if z % 2 == 0 {


            //                         model = glam::Mat4::IDENTITY;
            //                         model = model * glam::Mat4::from_translation(glam::Vec3::new(x as f32  * 1.2, y as f32  * 1.2, z as f32 * 1.2));

            //                         shader.set_mat4("model",model);
            //                         gl::DrawArrays(gl::TRIANGLES, 0,36);
            //                     }
            //                 }
            //             }
            //         } 
            //     }

            // }
        }

        // Poll for and process events
        glfw.poll_events();
        // swap front and back buffers
        window.swap_buffers();
    }

    // Cleanup and housekeeping
    cube2.destroy();
    // unsafe {
    //     gl::DeleteVertexArrays(1, &vao);
    //     gl::DeleteBuffers(1, &vbo);
    //     gl::DeleteBuffers(1, &ebo);
    // }
    
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
        glfw::WindowEvent::Scroll(_, _) =>{
            
            println!("I am scrolling");

        } 
        _ => {}
    }
}

