pub mod misc;
pub mod bresenham_algo;
pub mod scanline_algo;
pub mod bubble_sort_algo;

use crate::rasterizer::misc::*;
use crate::rasterizer::bresenham_algo::bresenham_algo;
use crate::rasterizer::scanline_algo::scanline_algo;
use crate::rasterizer::bubble_sort_algo::bubble_sort_algo;

use pixels::Pixels;


pub struct Rasterizer {
    pub m: PixelsCoordinate,
}

impl Rasterizer {

    pub fn init(m: &mut PixelsCoordinate, objet_in: ObjectDraw)
    {

        for triangle in objet_in.obj {


            bresenham_algo(m, &triangle);


        }
        
        bubble_sort_algo(m);

    }
    pub fn draw(m: &mut PixelsCoordinate, canvas: &mut Pixels)
    {

        scanline_algo(m, canvas);

        if canvas.render().is_err()
        {
            println!("error");
            return ;
        }

    }

}
