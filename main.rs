use fltk::draw::Offscreen;
use fltk::{enums};
use fltk::{app, prelude::*, window::Window};

mod rasterizer;

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;


mod misc;
use crate::map::Map;
use crate::misc::*;

mod keyboard_input;
use crate::keyboard_input::keyboard_input;

mod camera;
use crate::camera::Camera;

mod map;

pub fn main()
{
    

    println!("Loading window");

    let app = app::App::default();
    let mut wind = Window::default()
            .with_size(WIDTH as i32, HEIGHT as i32)
            .with_label("RayCaster")
            .center_screen();

    wind.set_color(enums::Color::Black);
    wind.end();
    wind.show();


            
    println!("Initialization.");
    let mut camera = Camera::new(25, 25, 100, 100);

    // 3d map;
    let offs = Offscreen::new(wind.width(), wind.height()).unwrap();

    offs.begin();
    Map::load_2d_map(FACTOR_RATIO);
    offs.end();

    let offs = Rc::from(RefCell::from(offs));


    println!("Start!");

    let mut t = 0.0;

    wind.draw(move | wind| {
          
        let t0 = Instant::now();


        // redraw map

        let mut offs = offs.borrow_mut();
        if offs.is_valid()
        {
            offs.rescale();
            offs.copy(0, 0, wind.width(), wind.height(), 0, 0);
        }
        else
        {
            offs.begin();
            offs.copy(0, 0, wind.width(), wind.height(), 0, 0);
            offs.end();
        }




        // objects
        camera.draw();









        // Event
        keyboard_input(&mut camera);



  



        //thread::sleep(Duration::from_millis(100));
        t = t0.elapsed().as_secs_f32();

        println!("dt: {t}");
    });


   


    app::add_idle3(move |_| 
    {
        wind.redraw();
        app::sleep(0.001);
    });
   



    app.run().unwrap();
 

}