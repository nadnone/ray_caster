pub mod misc;
pub mod bresenham_algo;
pub mod scanline_algo;
pub mod bubble_sort_algo;
pub mod graphics;


use crate::misc::{WIDTH, HEIGHT};
use crate::rasterizer::misc::*;
use crate::rasterizer::bresenham_algo::bresenham_algo;
use crate::rasterizer::scanline_algo::scanline_algo;
use crate::rasterizer::bubble_sort_algo::bubble_sort_algo;
use crate::rasterizer::graphics::draw_line;

use fltk::window::DoubleWindow;
pub struct Rasterizer;

impl Rasterizer {

    pub fn draw(objet_in: &mut ObjectDraw)
    {

        let mut m = PixelsCoordinate::new(WIDTH, HEIGHT);

        bresenham_algo(&mut m, objet_in);

        bubble_sort_algo(&mut m);

        scanline_algo(&mut m);

    }

}
