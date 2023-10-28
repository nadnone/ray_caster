use rand::Rng;

use crate::constants::{*, self};


#[derive(Copy, Clone)]
pub struct Camera {
    pub position: Position,
    pub angle: f32,
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

        if  map[(self.position.x + x) as usize][(self.position.y + y) as usize] != 1
        {
            self.position.x += x;
            self.position.y += y;
            self.angle = angle;
        }

    }


    pub fn get_rand_position(&mut self, map: &Vec<Vec<u8>>)
    {

        println!("[!] Looking for a camera position...");

        let t = std::time::Instant::now();
        let mut t_tmp = 0;

        loop {

            let t1 = t.elapsed().as_secs();
            if t_tmp != t1
            {
                println!("[?] t : {t1} sec", );
                t_tmp = t1;
            }

            let r_x = rand::thread_rng().gen_range(2..(map.len() -2));
            let r_y = rand::thread_rng().gen_range(2..(map[0].len() -2));

            // si libre, alors on choisi cette position
            if map[r_x][r_y] == 0
            {
                self.position.x = r_x as f32 + SPAWN_MARGE_MAP_DIST; // + 0.3 pour ne pas être dans un mur 
                self.position.y = r_y as f32 + SPAWN_MARGE_MAP_DIST;

                println!("[!] Position found !, you can play.");
                return;
            }
        }

    }

}


