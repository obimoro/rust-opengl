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

    first_mouse: bool,
    yaw: f64,
    pitch: f64,
    last_x: f64,
    last_y: f64,
    fov: f32,
    // other camera proerties goes here
    
}

impl Camera {
    const MIN_FOV: f32 = 1.0;
    const MAX_FOV: f32 = 65.0;

    const MIN_PITCH: f64 = -89.0;
    const MAX_PITCH: f64 = 89.0;

    pub fn new() -> Self {
        // Creates a new camera with the given initial position, front direction and up direction
        Camera {
            position: glam::Vec3::new(0.0, 0.0,3.0),
            front: glam::Vec3::new(0.0, 0.0,-1.0),
            up: glam::Vec3::new(0.0, 1.0,0.0),
            // other initialize properrties goes here
            first_mouse: false,
            yaw: 90.0,
            pitch: 0.0,
            last_x: WIN_WIDTH as f64 / 2.0,
            last_y: WIN_HEIGHT as f64  / 2.0,
            fov: 45.0,
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
    pub fn set_fov(&mut self, yoffset: f64) {

        self.fov += yoffset as f32;
        
        
        
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
                if self.fov > Self::MAX_FOV {
                self.fov = Self::MAX_FOV;
            }
        }
        if window.get_key(glfw::Key::E) == glfw::Action::Press {
            self.fov -= 1.0;
            if self.fov < Self::MIN_FOV {
                self.fov = Self::MIN_FOV;
            }
        }
    }


    pub fn mouse_callback(&mut self, xpos: f64, ypos: f64) {
        if self.first_mouse {
            self.last_x = xpos;
            self.last_y = ypos;
            self.first_mouse = false;
        }

        let mut xoffset = xpos - self.last_x;
        let mut yoffset = self.last_y - ypos;
        self.last_x = xpos;
        self.last_y = ypos;

        let sensitivity = 0.1;
        xoffset *= sensitivity;
        yoffset *= sensitivity;

        self.yaw += xoffset;
        self.pitch += yoffset;

        // make sure that pitch doesnt flipp screen when out of bounds
        self.pitch = self.pitch.clamp(Self::MIN_PITCH, Self::MAX_PITCH);

        let direction = glam::Vec3::new((self.yaw.to_radians().cos() * self.pitch.to_radians().cos()) as f32,
                                                self.pitch.to_radians().sin() as f32,
                                                (self.yaw.to_radians().sin() * self.pitch.to_radians().cos()) as f32,

        );
        self.front = direction.normalize();
    }


}