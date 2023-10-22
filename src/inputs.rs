
use std::f32::consts::PI;

use sdl2::{
    keyboard::Scancode,
    EventPump, 
    Sdl
};

use crate::{camera::Camera, misc::{SPEED, self, WIDTH}};

pub struct Inputs {
    mouse_capture: bool
}

impl Inputs {
    

    pub fn new() -> Inputs
    {
        return Inputs {
            mouse_capture: true
        }
    }

    pub fn update(&mut self, camera: &mut Camera, even_pump: &mut EventPump, sdl_context: &mut Sdl) -> u8
    {
        

        
        let mut angle = camera.get_angle();
        let angle_cross = angle + (90. * PI / 180.);

        let mut x = 0.;
        let mut y = 0.;


        // MOUSE event
        angle += even_pump.relative_mouse_state().x() as f32 / WIDTH;


        // Keyboard Events
        let event_keyboard = even_pump.keyboard_state();

        if event_keyboard.is_scancode_pressed(Scancode::Escape)
        {
            return 1;

        }
        if event_keyboard.is_scancode_pressed(Scancode::Tab)
        {
            self.mouse_capture = !self.mouse_capture;

            sdl_context.mouse().show_cursor(!self.mouse_capture);
            sdl_context.mouse().set_relative_mouse_mode(self.mouse_capture);
        }
    

        if event_keyboard.is_scancode_pressed(Scancode::W)
        {   
            x -= angle.cos() * SPEED;
            y -= angle.sin() * SPEED;
        }
        else if event_keyboard.is_scancode_pressed(Scancode::S)
        {
            x += angle.cos() * SPEED;
            y += angle.sin() * SPEED;
        }

        if event_keyboard.is_scancode_pressed(Scancode::A)
        {   
            x += angle_cross.cos() * SPEED;
            y += angle_cross.sin() * SPEED;
        }
        else if event_keyboard.is_scancode_pressed(Scancode::D)
        {
            x -= angle_cross.cos() * SPEED;
            y -= angle_cross.sin() * SPEED;
        }

        if event_keyboard.is_scancode_pressed(Scancode::E)
        {
            angle += misc::degtorad(2.0);

            x += angle.sin() * SPEED;
            y += angle.cos() * SPEED;
        }
        else if event_keyboard.is_scancode_pressed(Scancode::Q)
        { 
            angle -= misc::degtorad(2.0);
            
            x -= angle.sin() * SPEED;
            y -= angle.cos() * SPEED;
        }



        camera.translate(x, y, angle);

        return 0;

    }


}
