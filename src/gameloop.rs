use sdl2::EventPump;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::inputs::inputs;
use crate::camera::Camera;
use crate::misc::{FPS, CAMERA_ANGLE_START};



pub fn gameloop(canvas: &mut Canvas<Window>, event_pump: &mut EventPump, sdl_context: &mut sdl2::Sdl)
{

    let mut camera = Camera::new(1.0, 1.0, 4.5, 4.5, CAMERA_ANGLE_START);

    loop 
    {
        
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
    



    
        // calcul du raycaster
        camera.raycaster(canvas);


        // minimap
        camera.minimap_draw(canvas);
    
    
    
    
    
    
        // Event
        event_pump.wait_event_timeout(5);
        let callback = inputs(&mut camera, event_pump, sdl_context);
        if callback == 1
        {
            break;
        }
    
    
        
        canvas.present();

        std::thread::sleep(std::time::Duration::from_millis(FPS));
    
    }
}



