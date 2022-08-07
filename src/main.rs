
mod misc;
mod inputs;
mod camera;
mod gameloop;
mod raycaster;
mod mini_map;


use crate::{gameloop::gameloop, misc::*};


pub fn main()
{
    

    println!("Loading window");
    let mut sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
   
    let size = video_subsystem.current_display_mode(0).unwrap();

    let wind = video_subsystem.window("RayCaster", size.w as u32, size.h as u32)
        .position_centered()
        .fullscreen_desktop()
        .build()
        .unwrap();
    
    println!("Initialization.");

    let mut canvas = wind.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    sdl_context.mouse().show_cursor(false);
    sdl_context.mouse().set_relative_mouse_mode(true);

    canvas.set_logical_size(WIDTH as u32, HEIGHT as u32).unwrap();

    println!("Start!");
    gameloop(&mut canvas, &mut event_pump, &mut sdl_context);
    
   


 

}