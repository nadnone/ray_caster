mod rasterizer;

use crate::rasterizer::misc::*;

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


fn load(size: &LogicalSize<f32>) -> PixelsCoordinate
{
    let mut objet = ObjectDraw::new();


    let factor: [i32; 2] = [size.width as i32 / 10, size.height as i32 / 10];

    for j in 0..5
    {
        
        for i in 0..5
        {
                let x = i as i32 * factor[0];
                let y = j as i32 * factor[1];

                let mut color = [0, 0, 255];

                if MAP[j][i] == 1
                {
                    color = [0, 255, 0];

                }
                else if MAP[j][i] == 0
                {
                    color = [255, 0, 0];

                }
                let triangle_1 = Triangle::new(
                    Item { x: x + factor[0], y: y, color: color },
                    Item { x: x, y: y + factor[1], color: color },
                    Item { x: x + factor[0], y: y + factor[1], color: color },
                    color
                );
            
                let triangle_2 = Triangle::new(
                    Item { x: x, y: y, color: color },
                    Item { x: x + factor[0], y: y, color: color },
                    Item { x: x, y: y + factor[1], color: color },
                    color,
                );

                objet.obj.push(triangle_1);
                objet.obj.push(triangle_2);
                

            }

    }
    



    let mut m = PixelsCoordinate::new(WIDTH, HEIGHT);

    rasterizer::Rasterizer::init(&mut m, objet);

    return m;
}

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

    println!("loading map");
    let mut m = load(&size);
    println!("map loaded");


    println!("started!");
    game_loop(&mut m, &mut canvas);

}