use glfw::{Action, Context, Key, WindowEvent};
use gl::types::*;


const WIN_WIDTH: u32 = 640;
const WIN_HEIGHT: u32 = 480;

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
        gl::ClearColor(1.0, 0.5, 0.0, 1.0);
    }

    // Loop until the user closes the window
    while !window.should_close() {
        
        // input
        let (mouse_x, mouse_y) =  window.get_cursor_pos();
        println!("Mouse pos X {}, Mouse pos Y {}", mouse_x, mouse_y);
        // Rendering commands here
        
        // Clear the window
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        
        }

        // Poll for and process events
        glfw.poll_events();
        for (_, event)  in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }
        // swap front and back buffers
        window.swap_buffers();
    }
}


fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true);
        }
        _ => {}
    }
}

