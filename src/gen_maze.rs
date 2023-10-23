use std::{thread, time::Duration};

use rand::{Rng};

use crate::{constants::FPS, camera::{self, Camera}};


pub struct Maze {
    matrice: Vec<Vec<[i16; 3]>>,
    current: [i16; 2],
    step: [i16; 2],
    map: Vec<Vec<u8>>,
}

impl Maze {


    fn gen_cells(&mut self, n: i16)
    {

        for i in 0..n
        {
            let mut row: Vec<[i16; 3]> = vec![];
            for j in 0..n
            {
                row.push([2, 2, 0]); // [Left Right walls, Up Down Walls, visited bool]
            }
            self.matrice.push(row);
        }

    }

    fn gen_rand_vector() -> [i16; 2]
    {
        let r_x = rand::thread_rng().gen_range(0..3);
        let r_y = rand::thread_rng().gen_range(0..3);

        return [1 - r_x, 1 - r_y];
    }

    fn check_visited(&mut self) -> bool
    {

        for i in 0..self.matrice.len() {
            for j in 0..self.matrice[i].len() {
                
                if self.matrice[i][j][2] == 0 // not visited
                {
                    return false;
                }
            }
        }
        
        return true;
    }

    fn generate_maze_loop(&mut self) -> bool
    {

        if self.check_visited() // si il n'y a plus de visited cell
        {
            println!("all visited");
            return true; // stop the gen
        }

        self.step = self::Maze::gen_rand_vector();

        let px = self.step[0] + self.current[0];
        let py = self.step[1] + self.current[1];

        if px < 0 || py < 0 || px >= self.matrice.len() as i16 || py >= self.matrice[0].len() as i16
        {
            return false; // continue the gen
        }
        

        if self.matrice[px as usize][py as usize][2] == 0 // if not visited
        {
            // remove walls
            if (px - self.current[0] as i16) != 0
            {
                self.matrice[px as usize][py as usize][0] = px - self.current[0] as i16;
            }
            if (py - self.current[1] as i16) != 0
            {
                self.matrice[px as usize][py as usize][1] = py - self.current[1] as i16;
            }


            // mark as visited 
            self.matrice[px as usize][py as usize][2] = 1;

        }

        self.current = [px, py];
        return false; // continue the gen
    }

    pub fn gen_maze(&mut self, camera: &mut Camera)
    {
        let n = 6 as i16; 
        self.gen_cells(n);


        let r_x = rand::thread_rng().gen_range(0..(n -1));
        let r_y = rand::thread_rng().gen_range(0..(n -1));
        
        self.matrice[r_x as usize][r_y as usize][2] = 1; // mark as visited
        
        println!("[!] Maze Generation started, please wait..");

        self.current = [r_x, r_y];

        // on set la position de départ de la caméra
        camera.position.x = r_x as f32;
        camera.position.y = r_y as f32;

        loop {
            

            if self.generate_maze_loop()
            {   

                for i in 0..=n*3 + 2 {

                    let mut vec = vec![];

                    for j in 0..=n*3 + 2 {

                        if i == 0 || j == 0 || i == n*3+2 || j == n*3+2
                        {
                            vec.push(1);
                        }
                        else 
                        {
                            vec.push(0);
                        }
                    }
                    self.map.push(vec);
                }



                for x in 1..=n {
                    for y in 1..=n {

                        let px = x as usize * 3;
                        let py = y as usize * 3;

                        let lr_wall = self.matrice[x as usize - 1][y as usize - 1][0];
                        let ud_wall = self.matrice[x as usize - 1][y as usize - 1][1];
                       
                        if lr_wall != 0 && lr_wall != 2
                        {
                            self.map[px - (lr_wall + 1) as usize][py] = 1;
                        }
                        else {
                            self.map[px - 0][py] = 1;
                            self.map[px - 1][py] = 1;
                            self.map[px - 2][py] = 1;
                        }
                        if ud_wall != 0 && ud_wall != 2
                        {
                            self.map[px][py - (ud_wall + 1) as usize] = 1;
                        }
                        else {
                            self.map[px][py - 0] = 1;
                            self.map[px][py - 1] = 1;
                            self.map[px][py - 2] = 1;
                        }
                    }
                }

                break;

            }

            // generation du labyrinthe
            thread::sleep(Duration::from_secs_f32(FPS));
        }

        println!("[!] Maze Generation finished !");
    }

    pub fn init() -> Maze
    {
        return Maze {
            matrice: vec![],
            current: [0, 0],
            step: [0,0],
            map: vec![]
        };
    }

    pub fn get_map(self) -> Vec<Vec<u8>>
    {
        return self.map;
    }
}