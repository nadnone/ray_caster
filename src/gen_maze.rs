use std::{thread, time::Duration};
use rand::Rng;
use sdl2::EventPump;
use sdl2::rect::Rect;
use sdl2::{render::Canvas, pixels::Color};
use sdl2::video::Window;

use crate::constants::{FPS, INITIAL_MAZE_SIZE, SCALE_MINIMAP, SCALE_GEN_MAP};


pub struct Maze {
    matrice: Vec<Vec<[i16; 3]>>,
    current: [i16; 2],
    step: [i16; 2],
    pub map: Vec<Vec<u8>>,
}

impl Maze {


    fn gen_cells(&mut self, n: i16)
    {

        for _ in 0..n
        {
            let mut row: Vec<[i16; 3]> = vec![];
            for _ in 0..n
            {
                row.push([2, 2, 0]); // [Left Right walls, Up Down Walls, visited bool]
            }
            self.matrice.push(row);
        }

    }

    fn gen_rand_vector() -> [i16; 2]
    {
        let r = rand::thread_rng().gen_range(0..3);
        let rand_axe = rand::thread_rng().gen_bool(1./3.);

        if rand_axe
        {
            return [0, 1 - r];
        }
        else 
        {
            return [1 - r, 0];
        }
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

    fn generate_maze_loop(&mut self, canvas: &mut Canvas<Window>) -> bool
    {

        if self.check_visited() // si il n'y a plus de visited cell
        {
            return true; // stop the gen
        }

        self.step = self::Maze::gen_rand_vector();

        let px = self.step[0] + self.current[0];
        let py = self.step[1] + self.current[1];

        if px < 0 || py < 0 || px >= self.matrice.len() as i16 || py >= self.matrice[0].len() as i16
        {
            return false; // continue the gen
        }
        

        // width and height for animation
        let wh = SCALE_MINIMAP * self.matrice.len() as f32 * SCALE_GEN_MAP;

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

            // draw
            canvas.set_draw_color(Color::RGB(125, 125, 125));
            canvas.fill_rect(Rect::new( 
                (px as f32 * wh) as i32,
                (py as f32 * wh) as i32,
                wh as u32,
                wh as u32
            )).unwrap();
        }

          // draw the enclosure of the map generator view
          
          canvas.set_draw_color(Color::RGB(125, 0, 0));
          canvas.draw_rect(Rect::new( 
              0,
              0,
              wh as u32 * self.matrice.len() as u32,
              wh as u32 * self.matrice.len() as u32
          )).unwrap();

          canvas.present();
          

        self.current = [px, py];
        return false; // continue the gen
    }

    fn gen_map(&mut self, n: i16)
    {
        for i in 0..n*3 + 4 {

            let mut vec = vec![];

            for j in 0..n*3 + 4 {

                if i == 0 || j == 0 || i == n*3+3 || j == n*3+3
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
               
               // gauche droite
                if lr_wall != 0 && lr_wall != 2
                {
                    self.map[px - (lr_wall + 1) as usize][py] = 1;
                }
                else if lr_wall == 2
                {
                    self.map[px - 0][py] = 1;
                    self.map[px - 1][py] = 1;
                    self.map[px - 2][py] = 1;
                }

                // haut bas
                if ud_wall != 0 && ud_wall != 2
                {
                    self.map[px][py - (ud_wall + 1) as usize] = 1;
                }
                else if ud_wall == 2
                {
                    self.map[px][py - 0] = 1;
                    self.map[px][py - 1] = 1;
                    self.map[px][py - 2] = 1;
                }
            }
        }
    }

    pub fn gen_maze(&mut self, canvas: &mut Canvas<Window>, event: &mut EventPump)
    {
        let n = INITIAL_MAZE_SIZE;

        self.gen_cells(n);

        let r_x = rand::thread_rng().gen_range(0..(n -1));
        let r_y = rand::thread_rng().gen_range(0..(n -1));
        
        self.matrice[r_x as usize][r_y as usize][2] = 1; // mark as visited
        
        println!("[!] Maze Generation started, please wait..");

        self.current = [r_x, r_y];

        let t = std::time::Instant::now();
        let mut t_tmp = 0;
        loop {

            let t1 = t.elapsed().as_secs();
            if t_tmp != t1
            {
                println!("[?] t : {t1} sec", );
                t_tmp = t1;
            }
            
            // generation du labyrinthe
            if self.generate_maze_loop(canvas) // si le maze est generé
            {   
            
                self.gen_map(n);
                break;
            }


            // pour abandonner le chargement
            let ev = event.keyboard_state();
            if ev.is_scancode_pressed(sdl2::keyboard::Scancode::Escape)
            {
                std::process::exit(0);
            }
            event.pump_events();


            // on bride le cycle à FPS
            //thread::sleep(Duration::from_secs_f32(FPS));
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

 
}