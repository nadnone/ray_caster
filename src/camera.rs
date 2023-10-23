use crate::constants::{*, self};


#[derive(Copy, Clone)]
pub struct Camera {
    pub position: Position,
    angle: f32,
}

impl Camera {
    
    pub fn new(x:f32, y:f32, angle: f32) -> Camera
    {
        return Camera {
            position: Position::new(x , y),
            angle: constants::degtorad(angle),
        };
    }


    pub fn translate(&mut self, x: f32 , y: f32, angle: f32, map: &Vec<Vec<u8>>)
    {
        if map[(self.position.x + x) as usize][(self.position.y + y) as usize] != 1
        {
            self.position.x += x;
            self.position.y += y;
            self.angle = angle;
        }

    }


    pub fn get_angle(self) -> f32
    {
        return self.angle;
    }

    pub fn get_position(self) -> (f32, f32)
    {
        return (self.position.x, self.position.y);
    }


}


