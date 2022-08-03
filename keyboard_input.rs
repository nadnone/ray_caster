
use fltk::enums::*;

use crate::camera::Camera;

pub fn keyboard_input(camera: &mut Camera)
{


    if fltk::app::event_key_down(Key::from_char('w'))
    {
        camera.translate(0, 2);
    }


}


