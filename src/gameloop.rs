use sdl2::EventPump;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

use std::thread;
use std::time::{Instant, Duration};

use crate::map::Map;
use crate::inputs::inputs;
use crate::camera::Camera;




pub fn gameloop(canvas: &mut Canvas<Window>, event_pump: &mut EventPump, sdl_context: &mut sdl2::Sdl)
{

    let mut camera = Camera::new(25, 25, 400, 300);

    //let mut t = 0.0;


    loop 
    {
        
        //let t0 = Instant::now();
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
    





        // minimap
        //Map::load_2d_map(canvas);
    
    
    
    
        // calculer le raycaster
        camera.raycaster(canvas);


        // objects
        camera.draw(canvas);
    

    
    
    
    
    
    
    
        // Event
        event_pump.wait_event_timeout(5);
        let callback = inputs(&mut camera, event_pump, sdl_context);
        if callback == 1
        {
            break;
        }
        else if callback == 2
        {
            thread::sleep(Duration::from_secs_f32(1.0));
        }
    
    
    
    
        

    
        //t = t0.elapsed().as_secs_f32();


        
        //println!("dt: {t}");
    
        canvas.present();
    
    }
}



