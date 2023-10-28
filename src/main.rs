
mod constants;
mod inputs;
mod camera;
mod raycaster;
mod mini_map;
mod gen_maze;

use sdl2::pixels::Color;

use crate::gen_maze::Maze;
use crate::inputs::Inputs;
use crate::camera::Camera;
use crate::constants::{FPS, CAMERA_ANGLE_START, WIDTH, HEIGHT, STATES};
use crate::raycaster::*;


pub fn main()
{
    

    println!("[!] Loading window");

    let mut sdl_context = sdl2::init().unwrap();

    let video_subsystem = sdl_context.video().unwrap();
   
    let size = video_subsystem.current_display_mode(0).unwrap();

    let wind = video_subsystem.window("RayCaster", size.w as u32, size.h as u32)
        .position_centered()
        //.fullscreen_desktop()
        .build()
        .unwrap();
    


    println!("[!] Wipe screen");
    let mut canvas = wind.into_canvas().build().unwrap();
    canvas.set_logical_size(WIDTH as u32, HEIGHT as u32).unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

        
    println!("[!] Initialization.");


   
    // events
    let mut event_pump = sdl_context.event_pump().unwrap();

    // loading textures
    let wall_texture = image::open("./res/brick.png").unwrap();


    // load camera
    let mut camera = Camera::new(5.5, 5.0, CAMERA_ANGLE_START);

    let mut inputs = Inputs::new();
 
   
    // maze gen
    let mut maze_gen = Maze::init();
    maze_gen.gen_maze(&mut canvas, &mut event_pump);

    
    // camera random position
    camera.get_rand_position(&maze_gen.map);

    // load minimap
    let minimap = mini_map::Minimap::minimap_load(&maze_gen.map);
    
    
    //let mut t = 0.0;
    
    sdl_context.mouse().show_cursor(false);
    sdl_context.mouse().set_relative_mouse_mode(true);

    loop 
    {
        //let t0 = std::time::Instant::now();
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
    

        // ray caster
        Raycaster::engine(&mut canvas, &wall_texture, &mut camera, &maze_gen.map);


        // minimap
        mini_map::Minimap::minimap_draw(&mut canvas, &minimap, camera);
    
        // Event
        event_pump.pump_events();
        let callback = inputs.update(&mut camera, &mut event_pump, &mut sdl_context, &maze_gen.map);
        if callback == STATES::QUIT
        {
            break;
        }
    
    
        
        canvas.present();

        std::thread::sleep(std::time::Duration::from_secs_f32(FPS));

        //t = t0.elapsed().as_secs_f32();
        //println!("{t} seconds");
    
    }


 

}