pub const WIDTH:f32 = 480.0;
pub const HEIGHT:f32 = 270.0;
pub const HALF_HEIGHT: i32 = HEIGHT as i32 / 2;

pub const FPS: f32 = 1.0/120.0;

pub const CAMERA_ANGLE_START: f32 = 90.0;

pub const FOV: f32 = 68.0;
pub const HALF_FOV: f32 = FOV / 2.0;

pub const SPEED: f32 = 0.1;
pub const MAP_LEN: f32 = 6.0;

pub const RAY_PRECISION: i32 = 256;
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

#[derive(Copy, Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32
}

impl Position
{
    pub fn new(x: f32, y: f32) -> Position
    {
        return Position {
            x: x as f32,
            y: y as f32
        };
    }
}


 
pub fn degtorad(deg: f32) -> f32
{
    return deg * 3.1415 / 180.0;
}