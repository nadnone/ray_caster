use crate::rasterizer::*;

fn binary_search(y0: i32, m: &PixelsCoordinate) -> usize
{

    let mut r = m.coord.len();
    let mut l = 0;


    while l < r {

        let n = ((l + r) / 2) as usize;
        
        if m.coord[n].y < y0
        {
            l = n + 1;
        }
        else if y0 == m.coord[n].y
        {
            return n;
        }        
        else
        {
            r = n;    
        }
    }

    return l;
}

pub fn scanline_algo(m: &mut PixelsCoordinate, canvas: &mut Pixels)
{

    if m.coord.len() == 0
    {
        return ;
    }

    for i in 0..m.coord.len() {
        
        
        let x0 = m.coord[i].x;
        let y0 = m.coord[i].y;



        let n = binary_search(y0, &m);


        if n >= m.coord.len()
        {
            println!("error not found {i}");
            return;
        }

        let x1 = m.coord[n].x;


        let color0 = m.coord[i].color;
        let color1 = m.coord[n].color;

        // interpoler les couleurs


        if x0 < x1
        {
            draw_line(x0, x1, y0, color0, m.width, canvas); 
        }  
        else
        {
            draw_line(x1, x0, y0, color0, m.width, canvas); 
        }    

    }

}