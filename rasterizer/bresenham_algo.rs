use crate::rasterizer::*;

pub fn bresenham_algo(m: &mut PixelsCoordinate, objet: &mut ObjectDraw)
{
    if objet.obj.len() < 1
    {
        return ;
    }

    for k in 0..(objet.obj.len() -1)  {
        
        bresenham_calculus(objet, k, m, 1, 0);

    }
    bresenham_calculus(objet, objet.obj.len(), m, 1, 2);


}


fn bresenham_calculus(objet: &mut ObjectDraw, size: usize, m: &mut PixelsCoordinate, upper: usize, lower: usize)
{

    let mut l = 0; 
    let mut u = upper;
    let k = size - lower;

    if objet.obj[k].x > objet.obj[k+u].x
    {
         l = upper;
         u = 0;
    }
    
    let mut x0 = objet.obj[k+l].x as f32;  let mut x1 = objet.obj[k+u].x as f32;
    let mut y0 = objet.obj[k+l].y as f32;  let mut y1 = objet.obj[k+u].y as f32;

  

    for x in x0 as i32 .. (x1 as i32 + 1)
    {   
        let y = ( ( y1 - y0 ) / ( x1 - x0 ) * ( x as f32 - x0 ) ) + y0;

        m.coord.push(Item { x: x, y: y as i32, color: objet.obj[k+l].color });

    }

   
    if objet.obj[k+l].y > objet.obj[k+u].y
    {
            l = upper;
            u = 0;
    }


    x0 = objet.obj[k+l].x as f32;  x1 = objet.obj[k+u].x as f32;
    y0 = objet.obj[k+l].y as f32;  y1 = objet.obj[k+u].y as f32;


    for y in y0 as i32 .. (y1 as i32 + 1)
    {   
        let x = ( ( x1 - x0 ) /  ( y1 - y0 ) * ( y as f32 - y0 ) ) + x0;

        m.coord.push(Item { x: x as i32, y: y, color: objet.obj[k+l].color });


    }


}




