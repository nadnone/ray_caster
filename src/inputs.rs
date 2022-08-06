
use sdl2::{
    keyboard::Scancode,
    EventPump, 
    Sdl
};

use crate::{camera::Camera, misc::{SPEED, self}};

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
 


    let mut angle = camera.get_angle();

    let mut x = angle.cos() * SPEED;
    let mut y = angle.sin() * SPEED;


    if event_keyboard.is_scancode_pressed(Scancode::W)
    {   
        camera.translate(-x, -y, angle);
    }
    else if event_keyboard.is_scancode_pressed(Scancode::S)
    {
        camera.translate(x, y, angle);
    }



    let angle_cross = angle + 90.0;
    let px = angle_cross.cos() * SPEED;
    let py = angle_cross.sin() * SPEED;

    if event_keyboard.is_scancode_pressed(Scancode::A)
    {   
        camera.translate(px, py, angle);
    }
    else if event_keyboard.is_scancode_pressed(Scancode::D)
    {
        camera.translate(-px, -py, angle);
    }




    if event_keyboard.is_scancode_pressed(Scancode::E)
    {
        angle -= misc::degtorad(2.0);

        x = angle.sin() * 0.001;
        y = angle.cos() * 0.001;

        camera.translate(x, y, angle);

    }
    else if event_keyboard.is_scancode_pressed(Scancode::Q)
    { 
        angle += misc::degtorad(2.0);
        
        x = angle.sin() * 0.001;
        y = angle.cos() * 0.001;

        camera.translate(-x, -y, angle);

    }



    // MOUSE event
    // A REVOIR
/*
    let mouse_relative = even_pump.mouse_state();

    let mouse_x = Camera::degtorad(mouse_relative.x() as f32) * Camera::degtorad(FOV as f32) / 10.0;
    
    angle += mouse_x;

    x = angle.cos() * 0.001;
    y = angle.sin() * 0.001;

    camera.translate(x, y, angle);


    // recapture the cursor
    if mouse_relative.is_mouse_button_pressed(sdl2::mouse::MouseButton::Left)
    {
        sdl_context.mouse().show_cursor(false);
        sdl_context.mouse().set_relative_mouse_mode(true);
    }
    
*/

   
    return 0;

}


