use std::{thread, time::{Duration, Instant}};
use pixels::*;


use crate::rasterizer::misc::*;
use crate::rasterizer::*;

use crate::misc::*;

pub fn game_loop(m: &mut PixelsCoordinate, canvas: &mut Pixels)
{



    loop {
        
        let t0 = Instant::now();

        Rasterizer::draw(m, canvas);        

        thread::sleep(Duration::from_secs_f32(FPS));

        let t = t0.elapsed().as_secs_f32();


        if canvas.render().is_err()
        {
            println!("error");
            return ;
        }

        println!("dt: {t}");

    }
    


}