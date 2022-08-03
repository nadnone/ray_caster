use crate::rasterizer::*;

pub fn draw_line(x0: i32, x1: i32, y: i32, c: [u8; 3], w: u32, canvas: &mut Pixels)
{
    for x in x0..(x1 + 1) {
        draw_pixel(x, y, c, w, canvas);
    }
}

fn draw_pixel(x: i32, y: i32, c: [u8; 3], w: u32, canvas: &mut Pixels)
{
    
    let frame = canvas.get_frame();
    
    let color = [c[0], c[1], c[2], 255];

  
    for pixel in frame.chunks_exact_mut(4).skip(ix(x, y, w)) {
       
        pixel.copy_from_slice(&color);
        break;
    }

   
}
