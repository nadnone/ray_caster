
use crate::rasterizer::*;
use crate::rasterizer::misc::{ObjectDraw, Item};

#[derive(Copy, Clone)]
struct Position {
    x: i32,
    y: i32
}

impl Position
{
    fn new(x: i32, y: i32) -> Position
    {
        return Position {
            x: x,
            y: y
        };
    }
}

#[derive(Copy, Clone)]
pub struct Camera {
    width: i32,
    height: i32,
    position: Position,
}

impl Camera {
    
    pub fn new(w:i32, h:i32, x:i32, y:i32) -> Camera
    {
        
        return Camera {
            width: w,
            height: h,
            position: Position::new(x , y),
        };
    }

    pub fn translate(&mut self, x: i32 , y: i32)
    {
        self.position.x += x;
        self.position.y += y;

    }

    pub fn draw(self)
    {
        let mut objet = ObjectDraw::new();

        let x = self.position.x as i32;
        let y = self.position.y as i32;

        let color = [255, 0, 0];


        objet.obj.push(Item { x: x, y: y, color: color });
        objet.obj.push(Item { x: x + self.width, y: y, color: color });
        objet.obj.push(Item { x: x + self.width, y: y + self.height, color: color });

        objet.obj.push(Item { x: x, y: y, color: color });
        objet.obj.push(Item { x: x, y: y + self.height, color: color });
        objet.obj.push(Item { x: x + self.width, y: y + self.height, color: color });

        Rasterizer::draw(&mut objet);
        objet.obj.clear();


    }

}