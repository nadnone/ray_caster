
use sdl2::pixels::Color;
use sdl2::rect::{Rect};
use sdl2::render::Canvas;
use sdl2::video::Window;


#[derive(Copy, Clone)]
struct Position {
    x: i32,
    y: i32
}

impl Position
{
    fn new(x: i32, y: i32) -> Position
    {
        return Position {
            x: x,
            y: y
        };
    }
}

#[derive(Copy, Clone)]
pub struct Camera {
    width: i32,
    height: i32,
    position: Position,
    m_x: f32,
}

impl Camera {
    
    pub fn new(w:i32, h:i32, x:i32, y:i32) -> Camera
    {
        
        return Camera {
            width: w,
            height: h,
            position: Position::new(x , y),
            m_x: 0.0,
        };
    }


    pub fn translate(&mut self, x: i32 , y: i32)
    {
        self.position.x += x;
        self.position.y += y;
        

    }

    pub fn rotate(&mut self, mx: f32)
    {
        self.m_x += mx;
    }

    pub fn draw(&mut self, canvas: &mut Canvas<Window>)
    {

        let x = self.position.x as i32;
        let y = self.position.y as i32;

        let color = [255, 0, 0];

        
        canvas.set_draw_color(Color::RGB(color[0], color[1], color[2]));
        let rect = Rect::new(x, y, self.width as u32, self.height as u32);
        canvas.fill_rect(rect).unwrap();
        
        // calculer le raycaster
        
    


    

    }


}


