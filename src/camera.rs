
use sdl2::pixels::Color;
use sdl2::rect::{Rect, Point};
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::misc::{MAP, WIDTH, HEIGHT, RAY_PRECISION, WORLD_HEIGHT, WORLD_WIDTH, HALF_HEIGHT};


#[derive(Copy, Clone)]
struct Position {
    x: f32,
    y: f32
}

impl Position
{
    fn new(x: i32, y: i32) -> Position
    {
        return Position {
            x: x as f32,
            y: y as f32
        };
    }
}

#[derive(Copy, Clone)]
pub struct Camera {
    width: f32,
    height: f32,
    position: Position,
    angle: f32
}

impl Camera {
    
    pub fn new(w:i32, h:i32, x:i32, y:i32) -> Camera
    {
        
        let x_ = x + w/2;
        let y_ = y + h/2;

        return Camera {
            width: w as f32,
            height: h as f32,
            position: Position::new(x_ , y_),
            angle: Self::degtorad(50.0)
        };
    }


    pub fn translate(&mut self, x: f32 , y: f32)
    {

        self.position.x += x;
        self.position.y += y;

    }
 
    pub fn degtorad(deg: f32) -> f32
    {
        return deg * 3.1415 / 180.0;
    }

    pub fn get_angle(self) -> f32
    {
        return self.angle;
    }

    pub fn rotate(&mut self, mx: f32)
    {
        let angle = Self::degtorad(mx);
        self.angle += angle;
        
        self.angle %= Self::degtorad(360.0);

        self.position.x += self.angle.cos();
        self.position.y += self.angle.sin();


    }

    pub fn draw(&mut self, canvas: &mut Canvas<Window>)
    {

        let x = self.position.x + self.width / 2.0 / WORLD_WIDTH as f32;
        let y = self.position.y + self.height / 2.0 / WORLD_HEIGHT as f32;

        let color = [255, 0, 0];

        canvas.set_draw_color(Color::RGB(color[0], color[1], color[2]));
        let rect = Rect::new(x as i32, y as i32,  10 as u32, 10 as u32);
        canvas.fill_rect(rect).unwrap();
        

        // vecteur caméra
        let p2_x = x + (self.angle.cos() * 100.0);
        let p2_y = y + (self.angle.sin() * 100.0);
        canvas.draw_line(Point::new(x as i32, y as i32), Point::new(p2_x as i32, p2_y as i32)).unwrap();

        

    }


    pub fn raycaster(&mut self, canvas: &mut Canvas<Window>) 
    {
        

        let mut ray_angle = self.angle - 30.0;

        for ray_counter in 0..(WIDTH as i32)
        {
            let mut ray_x = self.position.x;
            let mut ray_y = self.position.y;

            // wall detection
            let mut wall = 0;
            while wall == 0 
            {
                
                ray_x += ray_angle.cos() / RAY_PRECISION as f32;
                ray_y += ray_angle.sin() / RAY_PRECISION as f32;


                let x_map = (ray_x / WIDTH as f32) * MAP[0].len() as f32 - 1.0;
                let y_map = (ray_y / HEIGHT as f32) * MAP.len() as f32 - 1.0;

                if !(x_map as usize >= MAP[0].len() || y_map as usize >= MAP.len())
                {
                    wall = MAP[y_map as usize][x_map as usize];

                }

            }


            // A revoir
            
            let mut distance = ((self.position.x - ray_x).powi(2) + (self.position.y - ray_y).powi(2)).sqrt();
            
            distance *= (ray_angle - self.angle).cos();
            


            let wallheight = (HALF_HEIGHT as f32 / distance) as i32;




            let top_screen = HALF_HEIGHT as i32 - wallheight;
            let bottom_screen = HALF_HEIGHT as i32 + wallheight;


            //sky
            canvas.set_draw_color(Color::RGB(0, 0, 255));
            canvas.draw_line(Point::new(ray_counter as i32, 0), Point::new(ray_counter as i32, HALF_HEIGHT as i32 - wallheight as i32)).unwrap();
            // Draw the wall
            canvas.set_draw_color(Color::RGB(0, 255, 0));
            canvas.draw_line(Point::new(ray_counter as i32, top_screen as i32), Point::new(ray_counter as i32, bottom_screen as i32)).unwrap();
            // floor
            canvas.set_draw_color(Color::RGB(155, 155, 155));
            canvas.draw_line(Point::new(ray_counter as i32, bottom_screen as i32), Point::new(ray_counter as i32, HEIGHT as i32)).unwrap();





            ray_angle += Self::degtorad(60.0 / WIDTH as f32);

        }
    }

}


