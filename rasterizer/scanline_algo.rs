use crate::rasterizer::*;

fn binary_search(y0: i32, m: &mut Vec<Item>) -> i32
{

    let mut r = m.len() as i32;
    let mut l = 0;


    while l < r {

        let n = (l + r) / 2;
        
        if m[n as usize].y < y0
        {
            l = n + 1;
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
    let mut m_tmp = m.coord.clone();

    if m.coord.len() == 0
    {
        return ;
    }

    for i in 0..m.coord.len() {
        
        
        let x0 = m.coord[i].x;
        let y0 = m.coord[i].y;



        let n = binary_search(y0, &mut m_tmp);


        if n as usize >= m.coord.len()
        {
            println!("error not found {i}");
            return;
        }

        let x1 = m.coord[n as usize].x;

        if x0 < x1
        {
            draw_line(x0, x1, y0, m.coord[i].color, m.width, canvas); 
        }  
        else
        {
            draw_line(x1, x0, y0, m.coord[i].color, m.width, canvas); 
        } 


        

    }

}