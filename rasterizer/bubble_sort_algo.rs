use crate::rasterizer::*;

fn sort_fun(m: &PixelsCoordinate, j: usize) -> (i32, i32, [u8; 3])
{

    let x1 = m.coord[j+1].x;
    let y1 = m.coord[j+1].y;

    let x0 = m.coord[j].x;
    let y0 = m.coord[j].y;

    if x0 > x1
    {
        return (x1, y1, m.coord[j+1].color);
    }
    else 
    {
        return (x0, y0, m.coord[j].color);
    }

}

pub fn bubble_sort_algo(m: &mut PixelsCoordinate)
{

    if m.coord.len() == 0
    {
        println!("empty array bubble_sort");
        return;
    }

    for j in 0..(m.coord.len() - 1) {
        
        let (x, y, color) = sort_fun(m, j);
        m.coord[j] = Item::new(x, y, color);

    }

}