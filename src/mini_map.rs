use sdl2::render::Canvas;
use sdl2::{pixels::Color, rect::Rect};
use sdl2::video::Window;

use crate::camera::Camera;
use crate::constants::*;

pub struct Minimap;


impl Minimap
{
    pub fn minimap_load(map: &Vec<Vec<u8>>) -> (Vec<Position>, Vec<[u8; 3]>)
    {
 
        let mut minimap_data = Vec::new();
        let mut colors = Vec::new();

        // MAP
        for i in 0..map.len() {
            
            for j in 0..map[0].len() {

                let mut color = [255, 155, 0];

                if map[i][j] == 1
                {
                    color = [155, 155, 155];
                }
                minimap_data.push(Position::new(i as f32 * SCALE_MINIMAP as f32, j as f32 * SCALE_MINIMAP as f32));
                colors.push(color);
        

            }
        }


        return (minimap_data, colors);
    }

    pub fn minimap_draw(canvas: &mut Canvas<Window>, minimap: &(Vec<Position>, Vec<[u8; 3]>), camera: Camera)
    {
        let wh = 1. / SCALE_MINIMAP;


        for i in 0..minimap.0.len()
        {
            let c = minimap.1[i];
            canvas.set_draw_color(Color::RGB(c[0], c[1], c[2]));


            let x = minimap.0[i].x * wh;
            let y = minimap.0[i].y * wh;


            let rect = Rect::new( 
                x as i32, y as i32,
                wh as u32,
                wh as u32
            );
            canvas.fill_rect(rect).unwrap();
        }


        // CAMERA
        canvas.set_draw_color(Color::RGB(255, 0, 0));

        let pos_x = camera.position.x;
        let pos_y = camera.position.y;

        let rect = Rect::new(
            pos_x as i32, pos_y as i32,
            wh as u32 / 2,
            wh as u32 / 2
        );

        canvas.fill_rect(rect).unwrap();
 
    }
}
  