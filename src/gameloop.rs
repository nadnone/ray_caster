use std::time;

use sdl2::EventPump;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::inputs::inputs;
use crate::camera::{Camera, self};
use crate::misc::{FPS, CAMERA_ANGLE_START};
use crate::raycaster::*;


pub fn gameloop(canvas: &mut Canvas<Window>, event_pump: &mut EventPump, sdl_context: &mut sdl2::Sdl)
{


    // loading textures
    let wall_texture = image::open("./res/bricksx64.png").unwrap();


    // load camera
    let mut camera = Camera::new(1.0, 1.0, 5.5, 5.0, CAMERA_ANGLE_START);

    //let mut t = 0.0;

    loop 
    {
        //let t0 = std::time::Instant::now();
        
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
    


        // ray caster
        Raycaster::raycasterize(canvas, &wall_texture, &camera);



        // minimap
        camera.minimap_draw(canvas);
    
    
        //println!("{} {}", camera.get_position().0, camera.get_position().1);
    
    
    
    
        // Event
        event_pump.wait_event_timeout(5);
        let callback = inputs(&mut camera, event_pump, sdl_context);
        if callback == 1
        {
            break;
        }
    
    
        
        canvas.present();

        std::thread::sleep(std::time::Duration::from_secs_f32(FPS));

        //t = t0.elapsed().as_secs_f32();

        //println!("{t}");
    
    }
}



