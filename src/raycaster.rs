use image::DynamicImage;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::camera::Camera;
use crate::misc;
use crate::misc::*;
use crate::texture_mapper;


pub struct Raycaster;

impl Raycaster 
{

  

    pub fn raycasterize(canvas: &mut Canvas<Window>, wall_texture: &DynamicImage, camera: &Camera) 
    {
        



        let mut ray_angle = camera.get_angle() - misc::degtorad(HALF_FOV);

        for ray_counter in 0..(WIDTH as i32)
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
            texture_mapper::texture_mapper(ray_x as usize, ray_y as usize, wall_texture);


            //sky
            canvas.set_draw_color(Color::RGB(0, 0, 255));
            canvas.draw_line(Point::new(ray_counter as i32, 0), Point::new(ray_counter as i32, (HALF_HEIGHT - wallheight) as i32)).unwrap();
            // Draw the wall
            canvas.set_draw_color(Color::RGB(0, 255, 0));
            canvas.draw_line(Point::new(ray_counter as i32, sky_limit as i32), Point::new(ray_counter as i32, floor_limit as i32)).unwrap();
            // floor
            canvas.set_draw_color(Color::RGB(155, 155, 155));
            canvas.draw_line(Point::new(ray_counter as i32, floor_limit as i32), Point::new(ray_counter as i32, HEIGHT as i32)).unwrap();





            ray_angle += misc::degtorad(RAY_INCREAMENT_ANGLE);

        }
    }


}
  