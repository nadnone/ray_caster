
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::misc::{MAP, WIDTH, HEIGHT, RAY_PRECISION, HALF_HEIGHT, RAY_INCREAMENT_ANGLE, HALF_FOV, MAP_LEN};


#[derive(Copy, Clone)]
struct Position {
    x: f32,
    y: f32
}

impl Position
{
    fn new(x: f32, y: f32) -> Position
    {
        return Position {
            x: x as f32,
            y: y as f32
        };
    }
}

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
            angle: Self::degtorad(angle)
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
 
    pub fn degtorad(deg: f32) -> f32
    {
        return deg * 3.1415 / 180.0;
    }

    pub fn get_angle(self) -> f32
    {
        return self.angle;
    }

    pub fn raycaster(&mut self, canvas: &mut Canvas<Window>) 
    {
        
        //println!("{} {}", self.position.x, self.position.y);


        let mut ray_angle = self.angle - Camera::degtorad(HALF_FOV);

        for ray_counter in 0..(WIDTH as i32)
        {
            let mut ray_x = self.position.x;
            let mut ray_y = self.position.y;



            // wall detection
            let mut wall = 0;
            while wall == 0
            {
                
                ray_x -= ray_angle.cos() / RAY_PRECISION as f32;
                ray_y -= ray_angle.sin() / RAY_PRECISION as f32;
                
                wall = MAP[ray_x as usize][ray_y as usize];

            }


            let mut distance = ((self.position.x - ray_x).powf(2.0) + (self.position.y - ray_y).powf(2.0)).sqrt();
            distance *= (ray_angle - self.angle).cos();



            let wallheight = (HALF_HEIGHT as f32 / distance) as i32; 


            let top_screen = HALF_HEIGHT - wallheight;
            let bottom_screen = HALF_HEIGHT + wallheight;


            //sky
            canvas.set_draw_color(Color::RGB(0, 0, 255));
            canvas.draw_line(Point::new(ray_counter as i32, 0), Point::new(ray_counter as i32, (HALF_HEIGHT - wallheight) as i32)).unwrap();
            // Draw the wall
            canvas.set_draw_color(Color::RGB(0, 255, 0));
            canvas.draw_line(Point::new(ray_counter as i32, top_screen as i32), Point::new(ray_counter as i32, bottom_screen as i32)).unwrap();
            // floor
            canvas.set_draw_color(Color::RGB(155, 155, 155));
            canvas.draw_line(Point::new(ray_counter as i32, bottom_screen as i32), Point::new(ray_counter as i32, HEIGHT as i32)).unwrap();





            ray_angle += Self::degtorad(RAY_INCREAMENT_ANGLE);

        }
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


