use sdl2::render::Canvas;
use sdl2::{pixels::Color, rect::Rect};
use sdl2::video::Window;

use crate::camera::Camera;
use crate::misc::*;

pub struct Minimap;

impl Minimap
{
    pub fn minimap_load() -> (Vec<Position>, Vec<[u8; 3]>)
    {
 
        let mut minimap_data = Vec::new();
        let mut colors = Vec::new();

        // MAP
        for j in 0..MAP[0].len() {
            
            for i in 0..MAP.len() {

                let mut color = [255, 155, 0];

                if MAP[i][j] == 1
                {
                    color = [155, 155, 155];
                }
                minimap_data.push(Position::new(i as f32 * 10.0, j as f32* 10.0));
                colors.push(color);
        

            }
        }

        return (minimap_data, colors);
    }

    pub fn minimap_draw(canvas: &mut Canvas<Window>, minimap: &(Vec<Position>, Vec<[u8; 3]>), camera: Camera)
    {


        for i in 0..minimap.0.len()
        {
            let c = minimap.1[i];
            canvas.set_draw_color(Color::RGB(c[0], c[1], c[2]));

            let rect = Rect::new( minimap.0[i].x as i32, minimap.0[i].y as i32, 10, 10);
            canvas.fill_rect(rect).unwrap();
        }


        // CAMERA
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        let rect = Rect::new(((camera.get_position().0 / MAP_LEN) * 60.0) as i32, (camera.get_position().1 / MAP_LEN * 60.0) as i32, 5, 5);
        canvas.fill_rect(rect).unwrap();
 
    }
}
  