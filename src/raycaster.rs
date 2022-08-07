use image::DynamicImage;
use image::GenericImageView;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::camera::Camera;
use crate::misc;
use crate::misc::*;

pub struct Raycaster;


impl Raycaster 
{

    pub fn engine(canvas: &mut Canvas<Window>, wall_texture: &DynamicImage, camera: &Camera) 
    {

        let mut ray_angle = camera.get_angle() - misc::degtorad(HALF_FOV);

        for x in 0..(WIDTH as i32)
        {

            let mut ray_x = camera.get_position().0;
            let mut ray_y = camera.get_position().1;



            // wall detection
            let mut wall = 0;
            while wall == 0
            {
                ray_x -= ray_angle.cos() / RAY_PRECISION as f32;
                ray_y -= ray_angle.sin() / RAY_PRECISION as f32;
                
                wall = MAP[ray_x as usize][ray_y as usize];
            }


            let mut distance = ((camera.get_position().0 - ray_x).powf(2.0) + (camera.get_position().1 - ray_y).powf(2.0)).sqrt();
            distance *= (ray_angle - camera.get_angle()).cos();


            let wallheight = (HALF_HEIGHT as f32 / distance) as i32; 

            let sky_limit = HALF_HEIGHT - wallheight;
            let floor_limit = HALF_HEIGHT + wallheight;

            // texture mapping

            let delta_x;
            let moyenne = (ray_x + ray_y) / 2.0;

            if moyenne < 0.5
            {
                // left side
                delta_x = moyenne.floor() + moyenne;
            }
            else
            {

                delta_x = moyenne.floor() + 1.0 - moyenne;
            }

            let tx = (delta_x * (wall_texture.width() - 1) as f32) as u32;


            for i in 0..(wallheight * 2) {


                let delta_y = i as f32 / (wallheight as f32 * 2.0);

                let ty = (delta_y * (wall_texture.height() - 1) as f32) as u32;

                let tex = wall_texture.get_pixel(tx, ty);

                // push data to buffer
                let y = sky_limit + i;

                // Draw the wall
                canvas.set_draw_color(Color::RGB(tex[0], tex[1], tex[2]));
                canvas.draw_rect(Rect::new(x as i32, y as i32, 1, 1)).unwrap();

            }


            //sky
            canvas.set_draw_color(Color::RGB(0, 0, 255));
            canvas.draw_line(Point::new(x as i32, 0), Point::new(x as i32, (HALF_HEIGHT - wallheight) as i32)).unwrap();
            
            // floor
            canvas.set_draw_color(Color::RGB(155, 155, 155));
            canvas.draw_line(Point::new(x as i32, floor_limit as i32), Point::new(x as i32, HEIGHT as i32)).unwrap();




            ray_angle += misc::degtorad(RAY_INCREAMENT_ANGLE);

        }
    }


}
  