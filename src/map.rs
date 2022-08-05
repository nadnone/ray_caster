use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::misc::*;

pub struct Map;

impl Map {

    pub fn load_2d_map(canvas: &mut Canvas<Window>)
    {
      
    
        for j in 0..MAP.len()
        {
            
            for i in 0..MAP[j].len()
            {
                    let x = i as i32 * MAP_WALL_SIZE as i32;
                    let y = j as i32 * MAP_WALL_SIZE as i32;
    
                    let mut color = [0, 0, 255];
    
                    if MAP[j][i] == 1
                    {
                        color = [0, 255, 0];
                    }

                    canvas.set_draw_color(Color::RGB(color[0], color[1], color[2]));


                    let rect = Rect::new(x, y, MAP_WALL_SIZE as u32, MAP_WALL_SIZE as u32);
                    canvas.fill_rect(rect).unwrap();


                }

    
        }

        
    


    }
    
    
   
}
