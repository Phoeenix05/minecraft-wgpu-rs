// use cgmath::{Point3, Vector3, Matrix4};

use winit::event::VirtualKeyCode::{Down, Left, Right, Up, A, D, S, W};
use winit::event::{ElementState, KeyboardInput, WindowEvent};

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
  1.0, 0.0, 0.0, 0.0,
  0.0, 1.0, 0.0, 0.0,
  0.0, 0.0, 0.5, 0.0,
  0.0, 0.0, 0.5, 1.0,
);

pub struct Camera {
    pub eye: cgmath::Point3<f32>,
    pub target: cgmath::Point3<f32>,
    pub up: cgmath::Vector3<f32>,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

impl Camera {
    pub fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);
        OPENGL_TO_WGPU_MATRIX * view * proj
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix().into();
    }
}

pub struct CameraController {
    speed: f32,
    // is_forward_pressed: bool,
    // is_backward_pressed: bool,
    // is_left_pressed: bool,
    // is_right_pressed: bool,
    /// 0 forward, 1 backward, 2 left, 3 right
    key_pressed: [bool; 4],
}

impl CameraController {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            key_pressed: [false; 4],
        }
    }

    pub fn process_events(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state,
                        virtual_keycode: Some(keycode),
                        ..
                    },
                ..
            } => {
                let is_pressed = *state == ElementState::Pressed;
                match keycode {
                    W | Up => {
                        self.key_pressed[0] = is_pressed;
                        true
                    }
                    A | Left => {
                        self.key_pressed[2] = is_pressed;
                        true
                    }
                    S | Down => {
                        self.key_pressed[1] = is_pressed;
                        true
                    }
                    D | Right => {
                        self.key_pressed[3] = is_pressed;
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }

    pub fn update_camera(&self, camera: &mut Camera) {
        use cgmath::InnerSpace;
        let forward = camera.target - camera.eye;
        let forward_norm = forward.normalize();
        let forward_mag = forward.magnitude();

        if self.key_pressed[0] && forward_mag > self.speed {
            camera.eye += forward_norm * self.speed;
        }
        if self.key_pressed[1] {
            camera.eye -= forward_norm * self.speed;
        }

        let right = forward_norm.cross(camera.up);

        let forward = camera.target - camera.eye;
        let forward_mag = forward.magnitude();

        if self.key_pressed[3] {
            camera.eye = camera.target - (forward + right * self.speed).normalize() * forward_mag;
        }
        if self.key_pressed[2] {
            camera.eye = camera.target - (forward - right * self.speed).normalize() * forward_mag;
        }
    }
}
