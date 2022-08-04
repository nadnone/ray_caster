
use sdl2::{
    keyboard::Scancode,
    EventPump, 
    Sdl
};

use crate::{camera::Camera};

pub fn inputs(camera: &mut Camera, even_pump: &mut EventPump, sdl_context: &mut Sdl) -> u8
{

   
    // Keyboard Events


    let event_keyboard = even_pump.keyboard_state();

    if event_keyboard.is_scancode_pressed(Scancode::Escape)
    {
        return 1;

    }
    if event_keyboard.is_scancode_pressed(Scancode::Tab)
    {
        sdl_context.mouse().show_cursor(true);
        sdl_context.mouse().set_relative_mouse_mode(false);
    }
    


    if event_keyboard.is_scancode_pressed(Scancode::W)
    {
        camera.translate(0, -1);

    }
    else if event_keyboard.is_scancode_pressed(Scancode::S)
    {
        camera.translate(0, 1);

    }

    if event_keyboard.is_scancode_pressed(Scancode::A)
    {
        camera.translate(-1, 0);

    }
    else if event_keyboard.is_scancode_pressed(Scancode::D)
    {
        camera.translate(1, 0);

    }




    // Mouse Events
    
    // Camera LootA

    let mouve_event = even_pump.relative_mouse_state();


    let mx = mouve_event.x() as f32;
    camera.rotate(mx);

   
    return 0;

}


