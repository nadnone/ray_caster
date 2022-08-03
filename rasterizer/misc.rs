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
    pub obj: Vec<Item>,
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