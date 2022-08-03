
use crate::rasterizer;
use crate::rasterizer::misc::*;

use crate::misc::*;


pub struct Map;

impl Map {

    pub fn load_2d_map(factor: [i32; 2])
    {
        let mut objet = ObjectDraw::new();
    
    
    
        for j in 0..MAP.len()
        {
            
            for i in 0..MAP[j].len()
            {
                    let x = i as i32 * factor[0];
                    let y = j as i32 * factor[1];
    
                    let mut color = [0, 0, 255];
    
                    if MAP[j][i] == 1
                    {
                        color = [0, 255, 0];
                    }
                    objet.obj.push(Item { x: x, y: y, color: color });
                    objet.obj.push(Item { x: x + factor[0], y: y, color: color });
                    objet.obj.push(Item { x: x + factor[0], y: y + factor[1], color: color });
    
                    objet.obj.push(Item { x: x, y: y, color: color });
                    objet.obj.push(Item { x: x, y: y + factor[1], color: color });
                    objet.obj.push(Item { x: x + factor[0], y: y + factor[1], color: color });
                   
                    rasterizer::Rasterizer::draw(&mut objet);
                    objet.obj.clear();
                
                }
    
        }
        
    
    
    
    
    }
    
    
   
}
