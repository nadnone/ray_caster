
use sdl2::pixels::Color;
use sdl2::rect::{Rect};
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::misc::{*, self};


#[derive(Copy, Clone)]
pub struct Camera {
    position: Position,
    angle: f32
}

impl Camera {
    
    pub fn new(w:f32, h:f32, x:f32, y:f32, angle: f32) -> Camera
    {
        
        let x_ = x + w/2.0;
        let y_ = y + h/2.0;

        return Camera {
            position: Position::new(x_ , y_),
            angle: misc::degtorad(angle)
        };
    }


    pub fn translate(&mut self, x: f32 , y: f32, angle: f32)
    {
        if MAP[(self.position.x + x) as usize][(self.position.y + y) as usize] != 1
        {
            self.position.x += x;
            self.position.y += y;
            self.angle = angle;
        }

    }


    pub fn get_angle(self) -> f32
    {
        return self.angle;
    }

    pub fn get_position(self) -> (f32, f32)
    {
        return (self.position.x, self.position.y);
    }

    pub fn minimap_draw(self, canvas: &mut Canvas<Window>)
    {

       
        // MAP
        for j in 0..MAP[0].len() {
            
            for i in 0..MAP.len() {

                canvas.set_draw_color(Color::RGB(255, 155, 0));

                if MAP[i][j] == 1
                {
                    canvas.set_draw_color(Color::RGB(155, 155, 155));
                }
                let rect = Rect::new(i as i32 * 10, j as i32 * 10 , 10, 10);
                canvas.fill_rect(rect).unwrap();

            }
        }


        // CAMERA
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        let rect = Rect::new(((self.position.x / MAP_LEN) * 60.0) as i32, (self.position.y / MAP_LEN * 60.0) as i32, 5, 5);
        canvas.fill_rect(rect).unwrap();
 
 
    }

}


