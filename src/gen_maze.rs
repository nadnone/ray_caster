use rand::Rng;
use sdl2::EventPump;
use sdl2::rect::Rect;
use sdl2::{render::Canvas, pixels::Color};
use sdl2::video::Window;

use crate::constants::{INITIAL_MAZE_SIZE, SCALE_GEN_MAP, MAX_DEPTH, STATES};


pub struct Maze {
    matrice: Vec<Vec<[bool; 3]>>,
    current: [i16; 2],
    stack: Vec<[i16; 2]>,
    cycle: i16,
    state: STATES,
    depth: i16,
    pub map: Vec<Vec<u8>>,
}

impl Maze {


    fn gen_cells(&mut self, n: i16)
    {

        for _ in 0..n
        {
            let mut row: Vec<[bool; 3]> = vec![];
            for _ in 0..n
            {
                row.push([false, false, false]); // [right, down, visited]
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
                
                if self.matrice[i][j][2] == false // not visited
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

                if self.matrice[x as usize][y as usize][2] == false
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
            // si rien trouvé, on recommence
            self.cycle = 1; 
            self.state = STATES::BACK;
            return;
        }

        let tmp_pos = self.stack[pos as usize];

        if !self.check_adjacent_visited() // nouvelle position trouvée
        {
            self.current = tmp_pos; // on prend l'ancienne position trouvée
            self.cycle = 1; // on reset les tentatives
            self.depth = 0; // on reset la profondeur du chemin choisi
            self.state = STATES::ACTIVE;
            self.generate_maze_recursive(canvas);
        }
        else
        {
            // on continue d'aller en arrière
            self.cycle += 1;
            self.state = STATES::BACK;
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
            self.state = STATES::FAILURE;
            return false // on continue
        }

        // si pas visité
        if self.matrice[random_wall[0] as usize][random_wall[1] as usize][2] == false
        {

            

            // on incrémente la profondeur
            self.depth += 1;
            // on push dans le stack
            self.stack.push(random_wall);

            // on verifie si le mur n'est pas déjà ouvert
            if  
                self.matrice[self.current[0] as usize][self.current[1] as usize][0] && rx < 0 || // right
                self.matrice[random_wall[0] as usize][random_wall[1] as usize][0] && rx > 0 || // left
                self.matrice[self.current[0] as usize][self.current[1] as usize][1] && ry < 0 || // down
                self.matrice[random_wall[0] as usize][random_wall[1] as usize][1] && ry > 0    // up
            {
                self.state = STATES::FAILURE;
                return false
            }

            // on casse le mur
            if rx > 0
            {
                self.matrice[random_wall[0] as usize][random_wall[1] as usize][0] = true; // left
            }
            if rx < 0
            {
                self.matrice[self.current[0] as usize][self.current[1] as usize][0] = true; // right
            }      
            if ry > 0
            {
                self.matrice[random_wall[0] as usize][random_wall[1] as usize][1] = true; // down
            }
            if ry < 0
            {
                self.matrice[self.current[0] as usize][self.current[1] as usize][1] = true; // up
            }


            // on dit qu'on est passé par là
            self.matrice[random_wall[0] as usize][random_wall[1] as usize][2] = true;
            self.state = STATES::ACTIVE;
        }    

        // le curseur devient la nouvelle cellule
        self.current = random_wall;

        // si tout les adjacents n'ont pas été visité + ce n'est pas le résultat du backtrace
        if !self.check_adjacent_visited() && self.depth < MAX_DEPTH && self.state != STATES::BACK
        {
            self.generate_maze_recursive(canvas);
        }
        else // sinon on créer un nouveau chemin
        {
            self.new_path_recursive(canvas);
        }

        
        return false; // continue the gen
    }

    fn cell_animation_loading(&mut self, canvas: &mut Canvas<Window>)
    {

        // width and height for animation
        let wh = self.matrice.len() as f32 * SCALE_GEN_MAP;

        for px in 0..self.matrice.len() 
        {
            for py in 0..self.matrice.len() 
            {
                

                let cell = self.matrice[px][py];
                               

                // draw the visited cell
                if cell[2] == true
                {          
                    canvas.set_draw_color(Color::RGB(0, 128, 0));
                    canvas.fill_rect(Rect::new( 
                        (px as f32 * wh) as i32,
                        (py as f32 * wh) as i32,
                        wh as u32,
                        wh as u32
                    )).unwrap();
                }
                else
                {
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


            }
        }  


        
    }

    fn gen_map(&mut self)
    {
        let start = 3;
        let end = 2;

        let n = self.matrice.len();

        for i in 0..=n*2 + end {

            let mut vec = vec![];

            for j in 0..=n*2 + end {

                if i == 0 || j == 0 || i == n*2+end || j == n*2+end
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



        for x in (start..self.map.len() - end).step_by(2)
        {
            
            for y in (start..self.map[x].len() - end).step_by(2)
            {
            
                let px = ((x as f32 / 2.) - start as f32).floor() as usize;
                let py = ((y as f32 / 2.) - start as f32).floor() as usize;

                let right_wall = self.matrice[px + 0][py + 0][0];
                let left_wall =  self.matrice[px + 1][py + 0][0];

                let down_wall =  self.matrice[px + 0][py + 1][1];
                let up_wall =    self.matrice[px + 0][py + 0][1];
                
                self.map[x][y] = 1; // par défaut il y a un mur

                match true {
                    
                    // aucun mur
                    k if k == down_wall & up_wall & left_wall & right_wall => {
                        self.map[x + 0][y + 0] = 0; 
                    },

                    // uniquement un mur
                    k if k == !down_wall & up_wall => {
                        self.map[x + 0][y + 1] = 1; // down
                    },
                    k if k == down_wall & !up_wall => {
                        self.map[x + 0][y - 1] = 1; // up
                    },
                    k if k == !right_wall & left_wall => {
                        self.map[x - 1][y + 0] = 1; // right
                    },
                    k if k == right_wall & !left_wall => {
                        self.map[x + 1][y + 0] = 1; // left
                    },

                    // L + R
                    k if k == !right_wall & !left_wall => {
                        self.map[x - 1][y] = 1; // right
                        self.map[x + 1][y] = 1; // left
                    },

                    // Up + L | R
                    k if k == !right_wall & !up_wall => {
                        self.map[x - 1][y + 0] = 1; // right
                        self.map[x + 0][y - 1] = 1; // up
                    },
                    k if k == !left_wall & !up_wall => {
                        self.map[x + 1][y + 0] = 1; // left
                        self.map[x + 0][y - 1] = 1; // up
                    },

                    // Down + L | R
                    k if k == !right_wall & !down_wall => { 
                        self.map[x - 1][y + 0] = 1; // right
                        self.map[x + 0][y + 0] = 1; // middle
                        self.map[x + 0][y + 1] = 1; // down 
                    },
                    k if k == !left_wall & !down_wall => {
                        self.map[x + 1][y + 0] = 1; // left
                        self.map[x + 0][y + 0] = 1; // middle
                        self.map[x + 0][y + 1] = 1; // down 
                    },

                    // Up & Down
                    k if k == !up_wall & !down_wall => {
                        self.map[x + 0][y - 1] = 1; // up
                        self.map[x + 0][y + 0] = 1; // middle
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
        
        self.matrice[r_x as usize][r_y as usize][2] = true; // mark as visited
        
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
            
            // pour animer le chargement
            self.cell_animation_loading(canvas);

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
            cycle: 1, // 1 car la len() max n'est pas l'index max
            state: STATES::ACTIVE,
            depth: 0
        };
    }

 
}