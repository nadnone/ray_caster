use rand::Rng;
use sdl2::EventPump;
use sdl2::rect::Rect;
use sdl2::{render::Canvas, pixels::Color};
use sdl2::video::Window;

use crate::constants::{INITIAL_MAZE_SIZE, SCALE_GEN_MAP, MAX_DEPTH};


pub struct Maze {
    matrice: Vec<Vec<[i16; 3]>>,
    current: [i16; 2],
    stack: Vec<[i16; 2]>,
    cycle: i16,
    state: String,
    depth: i16,
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
                row.push([0, 0, 0]); // [right, down, visited]
            }
            self.matrice.push(row);
        }

    }

    fn gen_rand_vector() -> [i16; 2]
    {
        let r = rand::thread_rng().gen_range(0..3);
        let rand_axe = rand::thread_rng().gen_bool(1./5.);

        if rand_axe
        {
            return [0, 1 - r];
        }
        else 
        {
            return [1 - r, 0];
        }
    }

    fn check_matrice_visited(&mut self) -> bool
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

    fn check_adjacent_visited(&mut self) -> bool
    {

        for i in -1..=1 {
            for j in -1..=1 {

                let x = self.current[0] + i;
                let y = self.current[1] + j;

                if x >= self.matrice.len() as i16 || y >= self.matrice[0].len() as i16 || x < 0 || y < 0
                {
                    continue;
                }

                if self.matrice[x as usize][y as usize][2] == 0
                {
                    return false;
                }
            }        
        }

        return true;
    }

    fn new_path_recursive(&mut self, canvas: &mut Canvas<Window>)
    {

        let pos = self.stack.len() as i16 - self.cycle;

        if pos <= 0 || pos >= self.stack.len() as i16
        {
            // si rien trouvé, on continue au pif
            self.cycle = 0;
            self.state = "failure back".to_string();
            return;
        }

        let tmp_pos = self.stack[pos as usize];

        if !self.check_adjacent_visited()
        {
            self.current = tmp_pos;

            self.cycle = 0; // on reset les tentatives
            self.depth = 0; // on reset la profondeur du chemin choisi
            self.state = "unvisited".to_string();
            self.generate_maze_recursive(canvas);
        }
        else
        {
            self.cycle += 1;
            self.state = "back increament".to_string();
            self.new_path_recursive(canvas)
        }
    }

    fn generate_maze_recursive(&mut self, canvas: &mut Canvas<Window>) -> bool
    {

        if self.check_matrice_visited() // si il n'y a plus de visited cell
        {
            return true; // stop the gen
        }

        // on choisi un vector aléatoire pour trouver un voisin
        let [rx, ry] = Maze::gen_rand_vector(); 
        let random_wall = [rx + self.current[0], ry + self.current[1]];

        // on verifie les limites et les nombres aléatoires
        if random_wall[0] < 0 || random_wall[1] < 0 || random_wall[0] >= self.matrice.len() as i16 || random_wall[1] >= self.matrice[0].len() as i16 || rx+ry == 0
        {

            if !self.state.contains("back") 
            {  
                self.state = "failure bound".to_string();
            }
            else 
            {
                self.state = "back failure".to_string();
            }

            return false // on continue
        }

        // si pas visité
        if self.matrice[random_wall[0] as usize][random_wall[1] as usize][2] == 0
        {


            // on incrémente la profondeur
            self.depth += 1;
            // on push dans le stack
            self.stack.push(random_wall);

            // on verifie si le mur n'est pas déjà ouvert
            if  
                self.matrice[self.current[0] as usize][self.current[1] as usize][0] > 0 && rx < 0 || // right
                self.matrice[random_wall[0] as usize][random_wall[1] as usize][0] > 0 && rx > 0 || // left
                self.matrice[self.current[0] as usize][self.current[1] as usize][1] > 0 && ry < 0 || // down
                self.matrice[random_wall[0] as usize][random_wall[1] as usize][1] > 0 && ry > 0    // up
            {
                self.state = "failure check".to_string();
                return false
            }

            // on casse le mur
            if rx > 0
            {
                self.matrice[random_wall[0] as usize][random_wall[1] as usize][0] = 1; // left
            }
            if rx < 0
            {
                self.matrice[self.current[0] as usize][self.current[1] as usize][0] = 1; // right
            }      
            if ry > 0
            {
                self.matrice[random_wall[0] as usize][random_wall[1] as usize][1] = 1; // down
            }
            if ry < 0
            {
                self.matrice[self.current[0] as usize][self.current[1] as usize][1] = 1; // up
            }


            // on dit qu'on est passé par là
            self.matrice[random_wall[0] as usize][random_wall[1] as usize][2] = 1;
            self.state = "active".to_string();
        }    

        self.current = random_wall;

        // si tout les adjacents n'ont pas été visité + ce n'est pas le résultat du backtrace
        if !self.check_adjacent_visited() && self.depth < MAX_DEPTH && !self.state.contains("back")
        {
            self.generate_maze_recursive(canvas);
        }
        else // sinon on créer un nouveau chemin
        {
            self.new_path_recursive(canvas);
        }



        // width and height for animation
        let wh = self.matrice.len() as f32 * SCALE_GEN_MAP;

        for px in 0..self.matrice.len() 
        {
            for py in 0..self.matrice.len() 
            {
                

                let cell = self.matrice[px][py];
                               
                // draw
                canvas.set_draw_color(Color::RGB(125, 125, 125));
                canvas.fill_rect(Rect::new( 
                    (px as f32 * wh) as i32,
                    (py as f32 * wh) as i32,
                    wh as u32,
                    wh as u32
                )).unwrap();



                if cell[2] == 1
                {          
                    canvas.set_draw_color(Color::RGB(0, 255, 0));
                    canvas.fill_rect(Rect::new( 
                        (px as f32 * wh) as i32,
                        (py as f32 * wh) as i32,
                        wh as u32,
                        wh as u32
                    )).unwrap();
                }

                canvas.set_draw_color(Color::RGB(255, 0, 0));
                canvas.fill_rect(Rect::new( 
                    (self.current[0] as f32 * wh) as i32,
                    (self.current[1] as f32 * wh) as i32,
                    wh as u32,
                    wh as u32
                )).unwrap();
               
                // draw the enclosure of the map generator view
                canvas.set_draw_color(Color::RGB(125, 0, 0));
                canvas.draw_rect(Rect::new( 
                    0,
                    0,
                    wh as u32 * self.matrice.len() as u32,
                    wh as u32 * self.matrice.len() as u32
                )).unwrap();


            }
        }  

        return false; // continue the gen

        
    }

    fn gen_map(&mut self)
    {

        let n = self.matrice.len();

        for i in 0..=n*2 + 1 {

            let mut vec = vec![];

            for j in 0..=n*2 + 1 {

                if i == 0 || j == 0 || i == n*2+1 || j == n*2+1
                {
                    vec.push(1); // mur limites
                }
                else 
                {
                    vec.push(0)
                }
            }
            self.map.push(vec);
        }


        for x in (1..self.map.len() - 1).step_by(2)
        {
            
            for y in (1..self.map[x].len() - 1).step_by(2)
            {
            
                let px = ((x as f32 / 2.) - 1.).floor() as usize;
                let py = ((y as f32 / 2.) - 1.).floor() as usize;

                let right_wall = self.matrice[px + 0][py + 0][0];
                let left_wall =  self.matrice[px + 1][py + 0][0];

                let down_wall =  self.matrice[px + 0][py + 1][1];
                let up_wall =    self.matrice[px + 0][py + 0][1];
                
                self.map[x][y] = 1; // par défaut il y a un mur

                match 1 {

                    k if k == (down_wall | up_wall) & down_wall => {
                        self.map[x + 0][y + 1] = 1; // down
                    },
                    k if k == (down_wall | up_wall) & up_wall => {
                        self.map[x + 0][y - 1] = 1; // up
                    },
                    k if k == (right_wall | left_wall) & right_wall => {
                        self.map[x - 1][y + 0] = 1; // right
                    },
                    k if k == (right_wall | left_wall) & left_wall => {
                        self.map[x + 1][y + 0] = 1; // left
                    },

                    // L + R
                    k if k == right_wall & left_wall => {
                        self.map[x - 1][y] = 1; // right
                        self.map[x + 0][y] = 0; // middle
                        self.map[x + 1][y] = 1; // left
                    },

                    // Up + L | R
                    k if k == right_wall & up_wall => {
                        self.map[x - 1][y + 0] = 1; // right
                        self.map[x + 0][y + 0] = 0; // middle
                        self.map[x + 0][y - 1] = 1; // up
                    },
                    k if k == left_wall & up_wall => {
                        self.map[x + 1][y + 0] = 1; // left
                        self.map[x + 0][y + 0] = 0; // middle
                        self.map[x + 0][y - 1] = 1; // up
                    },

                    // Down + L | R
                    k if k == right_wall & down_wall => { 
                        self.map[x - 1][y + 0] = 1; // right
                        self.map[x + 0][y + 0] = 0; // middle
                        self.map[x + 0][y + 1] = 1; // down 
                    },
                    k if k == left_wall & down_wall => {
                        self.map[x + 1][y + 0] = 1; // left
                        self.map[x + 0][y + 0] = 0; // middle
                        self.map[x + 0][y + 1] = 1; // down 
                    },

                    // Up & Down
                    k if k == up_wall & down_wall => {
                        self.map[x + 0][y - 1] = 1; // up
                        self.map[x + 0][y + 0] = 0; // middle
                        self.map[x + 0][y + 1] = 1; // down
                    },
      
                    _ => {} 
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
            if self.generate_maze_recursive(canvas) // si le maze est generé
            {   
                self.gen_map();
                break;
            }


            // pour abandonner le chargement
            let ev = event.keyboard_state();
            if ev.is_scancode_pressed(sdl2::keyboard::Scancode::Escape)
            {
                std::process::exit(0);
            }
            event.pump_events();


            canvas.present();
            
            // on bride le cycle à FPS
            //std::thread::sleep(Duration::from_secs_f32(FPS));
        }

        println!("[!] Maze Generation finished !");

    }

    pub fn init() -> Maze
    {
        return Maze {
            matrice: vec![],
            current: [0, 0],
            map: vec![],
            stack: vec![],
            cycle: 1,
            state: "start".to_string(),
            depth: 0
        };
    }

 
}