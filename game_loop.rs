use std::{thread, time::{Duration, Instant}};
use pixels::*;

use crate::rasterizer::misc::*;
use crate::rasterizer::*;

use crate::misc::*;

fn load_2d_map(canvas: &mut Pixels) -> ObjectDraw
{
    let mut objet = ObjectDraw::new();


    let factor: [i32; 2] = [WIDTH as i32 / 10, HEIGHT as i32 / 10];

    for j in 0..5
    {
        
        for i in 0..5
        {
                let x = i as i32 * factor[0];
                let y = j as i32 * factor[1];

                let mut color = [0, 0, 255];

                if MAP[j][i] == 1
                {
                    color = [0, 255, 0];
                }
                objet.obj.push(Item { x: x, y: y, color: color });
                objet.obj.push(Item { x: x + factor[0], y: y, color: color });
                objet.obj.push(Item { x: x + factor[0], y: y + factor[1], color: color });

                objet.obj.push(Item { x: x, y: y, color: color });
                objet.obj.push(Item { x: x, y: y + factor[1], color: color });
                objet.obj.push(Item { x: x + factor[0], y: y + factor[1], color: color });

                Rasterizer::draw(canvas, &mut objet);        
                objet.obj.clear();
                
            }

    }
    





    return objet;
}
pub fn game_loop(canvas: &mut Pixels)
{


    load_2d_map(canvas);


    loop {
        
        let t0 = Instant::now();


        thread::sleep(Duration::from_secs_f32(FPS));

        let t = t0.elapsed().as_secs_f32();


        println!("dt: {t}");

    }
    


}