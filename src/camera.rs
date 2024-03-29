use crate::tuple::{cross, normalize, Tuple};
use crate::{matrices::Matrix, tuple::vector};

pub struct Camera {
    pub fov: f32,
    pub camera_position: Tuple,
    pub camera_front: Tuple,
    camera_up: Tuple,
    sensivity: f32,
    pub camera_speed: f32,
    delta_time: f32,
    last_frame: f32,
    last_x: f32,
    last_y: f32,
    yaw: f32,
    pitch: f32,
    first_mouse: bool,
}

impl Camera {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            camera_position: vector(0., 0., -3.),
            camera_front: vector(0., 0., -1.),
            camera_up: vector(0., 1., 0.),
            fov: 45.0,
            sensivity: 0.1,
            camera_speed: 2.5,
            delta_time: 0.,
            last_frame: 0.,
            last_x: (width / 2) as f32,
            last_y: (height / 2) as f32,
            yaw: -90.,
            pitch: 0.,
            first_mouse: true,
        }
    }

    pub fn look_at(&self) -> Matrix {
        Matrix::look_at(
            self.camera_position,
            self.camera_position - self.camera_front,
            self.camera_up,
        )
    }

    pub fn handle_cursor(&mut self, xpos: f32, ypos: f32) {
        if self.first_mouse {
            self.last_x = xpos;
            self.last_y = ypos;
            self.first_mouse = false;
        }
        let xoffset = xpos - self.last_x;
        let yoffset = self.last_y - ypos;
        self.last_x = xpos;
        self.last_y = ypos;
        self.yaw += xoffset * self.sensivity;
        self.pitch -= yoffset * self.sensivity;
        if self.pitch > 89.0 {
            self.pitch = 89.0;
        }
        if self.pitch < -89.0 {
            self.pitch = -89.0;
        }
        self.camera_front = vector(
            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
        );
        self.camera_front = normalize(self.camera_front);
        let camera_right = normalize(cross(self.camera_front, vector(0., 1., 0.)));
        self.camera_up = normalize(cross(camera_right, self.camera_front));
    }

    pub fn handle_scroll(&mut self, yoffset: f32) {
        self.fov -= yoffset as f32;
        if self.fov < 1.0 {
            self.fov = 1.0;
        }
        if self.fov > 45.0 {
            self.fov = 45.0;
        }
    }

    pub fn update_delta_time(&mut self, current_frame: f32) {
        self.delta_time = current_frame - self.last_frame;
        self.last_frame = current_frame;
    }

    pub fn update_camera_speed(&mut self) {
        self.camera_speed = 2.5 * self.delta_time;
    }

    pub fn handle_w(&mut self) {
        self.camera_position = self.camera_position - (self.camera_front * self.camera_speed);
    }

    pub fn handle_s(&mut self) {
        self.camera_position = self.camera_position + (self.camera_front * self.camera_speed);
    }

    pub fn handle_a(&mut self) {
        self.camera_position = self.camera_position
            + (normalize(cross(self.camera_front, self.camera_up)) * self.camera_speed);
    }

    pub fn handle_d(&mut self) {
        self.camera_position = self.camera_position
            - (normalize(cross(self.camera_front, self.camera_up)) * self.camera_speed);
    }
}
