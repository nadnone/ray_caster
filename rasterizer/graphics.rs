use fltk;


pub fn draw_line(x0: i32, x1: i32, y1: i32, c: [u8; 3], w: u32)
{

   
    fltk::draw::set_draw_color(fltk::enums::Color::from_rgb(c[0], c[1], c[2]));
    fltk::draw::draw_line(x0, y1, x1, y1);



}
