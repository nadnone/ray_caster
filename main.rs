mod rasterizer;


mod game_loop;
use crate::game_loop::*;

mod misc;
use crate::misc::*;

use winit::{ 
    dpi::LogicalSize, 
    window::WindowBuilder,
    event_loop::{EventLoop}
};
use pixels::*;



fn window_create(size: &LogicalSize<f32>, scaled_size: &LogicalSize<f32>) -> Pixels
{
    let event_loop = EventLoop::new();

    let window = {



        WindowBuilder::new()
            .with_title("Rasterizer")
            .with_inner_size(*scaled_size)
            .with_min_inner_size(*size)
            .build(&event_loop)
            .unwrap()
    };

    let canvas = Pixels::new(WIDTH, HEIGHT, SurfaceTexture::new(window.inner_size().width, window.inner_size().height, &window)).unwrap();

    return canvas;
}

pub fn main()
{
    let size = LogicalSize::new(WIDTH as f32, HEIGHT as f32);
    let scaled_size = LogicalSize::new(WIDTH as f32 * FACTOR_SIZE, HEIGHT as f32 * FACTOR_SIZE);

    println!("loading canvas");
    let mut canvas = window_create(&size, &scaled_size);
    println!("canvas loaded");


    println!("started!");
    game_loop(&mut canvas);

}