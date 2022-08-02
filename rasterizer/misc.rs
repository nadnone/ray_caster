use pixels::Pixels;



pub struct Triangle {
    pub data: [Item; 3],
    pub color: [u8; 3]
}

impl Triangle {

    pub fn new(x: Item, y: Item, z: Item, color: [u8; 3]) -> Triangle
    {    
        return Triangle {
            data: [x, y ,z],
            color: color        
        };
    }
}

pub struct PixelsCoordinate {
    pub coord: Vec<Item>,
    pub height: u32,
    pub width: u32
}

impl PixelsCoordinate {

    pub fn new(canvas_width: u32, canvas_height: u32) -> PixelsCoordinate
    {
        return PixelsCoordinate {
            coord: Vec::new(),
            height: canvas_height,
            width: canvas_width
        };
    }
}


#[derive(Copy, Clone)]

pub struct Item {
    pub x: i32,
    pub y: i32,
    pub color: [u8; 3]
}

impl Item {

    pub fn new(x_: i32, y_: i32, color_: [u8; 3]) -> Item
    {
        return Item {
            x: x_,
            y: y_,
            color: color_
        };
    }
}

pub struct ObjectDraw {
    pub obj: Vec<Triangle>,
}

impl ObjectDraw {

    pub fn new() -> ObjectDraw
    {
        return ObjectDraw {
            obj: Vec::new(),
        };
    }
}


pub fn ix(x: i32, y: i32, w: u32) -> usize
{
    return ( x + y * w as i32) as usize;
}

pub fn draw_line(x0: i32, x1: i32, y: i32, c: [u8; 3], w: u32, canvas: &mut Pixels)
{
    for x in x0..x1 {
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
