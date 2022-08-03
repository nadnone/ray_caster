
use crate::misc::{HEIGHT, WIDTH};
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
    m_x: i32,
    m_y: i32
}

impl Camera {
    
    pub fn new(w:i32, h:i32, x:i32, y:i32) -> Camera
    {
        
        return Camera {
            width: w,
            height: h,
            position: Position::new(x , y),
            m_x: 0,
            m_y: 0
        };
    }

    pub fn translate(&mut self, x: i32 , y: i32)
    {
        self.position.x += x;
        self.position.y += y;

    }

    pub fn rotate(&mut self, m_x: i32, m_y: i32)
    {
        self.m_x = m_x;
        self.m_y = m_y;
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
 
        
        let p_center_x = self.position.x - WIDTH as i32;
        let p_center_y = self.position.y - HEIGHT as i32;

        
        // calculer le raycaster


        fltk::draw::set_draw_color(fltk::enums::Color::from_rgb(255, 0, 0));
        fltk::draw::draw_line(self.position.x + self.width/2, self.position.y + self.height/2, self.m_x as i32, self.m_y as i32);



    }

}