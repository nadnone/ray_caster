pub const WIDTH:f32 = 640.0;
pub const HEIGHT:f32 = 480.0;
pub const HALF_HEIGHT: i32 = HEIGHT as i32 / 2;

pub const FPS: u64 = 10;

pub const CAMERA_ANGLE_START: f32 = 90.0;

pub const FOV: f32 = 60.0;
pub const HALF_FOV: f32 = FOV / 2.0;

pub const SPEED: f32 = 0.01;
pub const MAP_LEN: f32 = 6.0;

pub const RAY_PRECISION: i32 = 128;
pub const RAY_INCREAMENT_ANGLE: f32 = FOV / WIDTH as f32;

pub const MAP: [[i32; 7]; 7] = [
    [1, 1, 1, 1, 1, 1, 1],
    [1, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 1, 1],
    [1, 0, 0, 0, 1, 1, 1],
    [1, 1, 0, 0, 0, 0, 1],
    [1, 1, 1, 1, 1, 1, 1],

]; 