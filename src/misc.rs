pub const WIDTH:u32 = 800;
pub const HEIGHT:u32 = 800;
pub const HALF_HEIGHT: i32 = 400;

pub const FPS:f32 = 1.0 / 60.0;

//pub const CAMERA_ANGLE:f32 = 60.0;
//pub const CAMERA_MAX_D:f32 = 10.0;

pub const WORLD_HEIGHT: i32 = 100;
pub const WORLD_WIDTH: i32 = 100;

pub const MAP_WALL_SIZE: i32 = 200;

pub const RAY_PRECISION: i8 = 8;


pub const MAP: [[i32; 7]; 7] = [
    [1, 1, 1, 1, 1, 1, 1],
    [1, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 1, 0, 1],
    [1, 0, 1, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 1, 0, 0, 1],
    [1, 1, 1, 1, 1, 1, 1],
];