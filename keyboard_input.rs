
use fltk::enums::*;

use crate::camera::Camera;

pub fn Inputs(camera: &mut Camera)
{


    if fltk::app::event_key_down(Key::from_char('w'))
    {
        camera.translate(0, -2);
    }
    else if fltk::app::event_key_down(Key::from_char('s'))
    {
        camera.translate(0, 2);
    }

    if fltk::app::event_key_down(Key::from_char('a'))
    {
        camera.translate(-2, 0);
    }   
    else if fltk::app::event_key_down(Key::from_char('d'))
    {
        camera.translate(2, 0);
    }



    let m_x = fltk::app::event_x();
    let m_y = fltk::app::event_y();

    camera.rotate(m_x, m_y);




}


