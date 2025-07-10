// TODO: 
// 1. Calculate the mouse's offset since the last frame.
// 2. Add the offset values to the camera's yaw and pitch values.
// 3. Add some constraints to the minimum/maximum pitch values.
// 4. Calculate the direction vector.

use glfw::{self, ffi::glfwSetScrollCallback};
use crate::constants;

use constants::{WIN_WIDTH, WIN_HEIGHT};

pub struct Camera {
    position: glam::Vec3,
    front: glam::Vec3,
    up: glam::Vec3,

    yaw: f32,
    pitch: f32,
    last_x: f32,
    last_y: f32,
    fov: f32,
    // other camera proerties goes here
}

impl Camera {
    pub fn new() -> Self {
        // Creates a new camera with the given initial position, front direction and up direction
        Camera {
            position: glam::Vec3::new(0.0, 0.0,3.0),
            front: glam::Vec3::new(0.0, 0.0,-1.0),
            up: glam::Vec3::new(0.0, 1.0,0.0),
            // other initialize properrties goes here
            yaw: -90.0,
            pitch: 0.0,
            last_x: WIN_WIDTH as f32 / 2.0,
            last_y: WIN_HEIGHT as f32  / 2.0,
            fov: 66.0,
        }
    }

    // Gets the current position of the camera
    pub fn get_position(&self) -> glam::Vec3 {
        self.position
    }

    // Gets the current front direction of the camera
    pub fn get_front(&self) -> glam::Vec3 {
        self.front
    }

    // Get the current up direction of the camera
    pub fn get_up(&self) -> glam::Vec3 {
        self.up
    }

    pub fn get_fov(&self) -> f32 {
        self.fov
        
    }

    pub fn set_fov(&mut self, fov: f32) {
        self.fov = fov;
        
    }


    // Processes input from the window and updates the camera's position accordingly.
    pub fn process_input(&mut self, window: &mut glfw::Window, delta_time: &f32) {
        let camera_speed: f32 = 10.0 * delta_time;
        if window.get_key(glfw::Key::W) == glfw::Action::Press {
            self.position += camera_speed * self.front;
        }
        if window.get_key(glfw::Key::S) == glfw::Action::Press {
            self.position -= camera_speed * self.front;
        }
        if window.get_key(glfw::Key::A) == glfw::Action::Press {
            self.position -= camera_speed * (self.front).cross(self.up).normalize();
        }
        if window.get_key(glfw::Key::D) == glfw::Action::Press {
            self.position += camera_speed * (self.front).cross(self.up).normalize();
        }
        if window.get_key(glfw::Key::Space) == glfw::Action::Press {
            self.position += camera_speed * self.up;
        }
        if window.get_key(glfw::Key::LeftControl) == glfw::Action::Press {
            self.position -= camera_speed * self.up;
        }
        if window.get_key(glfw::Key::Q) == glfw::Action::Press {
            self.fov += 1.0;
        }
        if window.get_key(glfw::Key::E) == glfw::Action::Press {
            self.fov -= 1.0;
        }
    }

    pub fn mouse_callback(&mut self, xpos: f64, ypos: f64) {
        let xoffset = (xpos as f32 - self.last_x) * 0.1;
        let yoffset = (self.last_y - (ypos as f32)) * 0.1;

        self.yaw += xoffset;
        self.pitch += yoffset;

        // Add some constraints to the minimum/maximum pitch values
        self.pitch = self.pitch.clamp(-89.0, 89.0);

        self.last_x = xpos as f32;
        self.last_y = ypos as f32;
    }

}