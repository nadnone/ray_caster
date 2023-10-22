use sdl2::EventPump;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;

use crate::inputs::Inputs;
use crate::camera::Camera;
use crate::misc::{FPS, CAMERA_ANGLE_START};
use crate::{raycaster::*, mini_map};


pub fn gameloop(canvas: &mut Canvas<Window>, event_pump: &mut EventPump, sdl_context: &mut sdl2::Sdl)
{


    // loading textures
    let wall_texture = image::open("./res/brick.png").unwrap();


    // load camera
    let mut camera = Camera::new(5.5, 5.0, CAMERA_ANGLE_START);
    
    // load minimap
    let minimap = mini_map::Minimap::minimap_load();

    
    let mut inputs = Inputs::new();

    let mut t = 0.0;

    loop 
    {
        let t0 = std::time::Instant::now();
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
    

        // ray caster
        Raycaster::engine(canvas, &wall_texture, &camera);


        // minimap
        mini_map::Minimap::minimap_draw(canvas, &minimap, camera);
    
    
    
    
        // Event
        event_pump.pump_events();
        let callback = inputs.update(&mut camera, event_pump, sdl_context);
        if callback == 1
        {
            break;
        }
    
    
        
        canvas.present();

        std::thread::sleep(std::time::Duration::from_secs_f32(FPS));

        t = t0.elapsed().as_secs_f32();

        println!("{t} seconds");
    
    }
}



