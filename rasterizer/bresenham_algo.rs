use crate::rasterizer::*;

pub fn bresenham_algo(m: &mut PixelsCoordinate, objet: &mut ObjectDraw)
{
    if objet.obj.len() < 1
    {
        return ;
    }

    for k in 0..(objet.obj.len() -1)  {
        
        bresenham_calculus_x(objet, k, m, 1, 0);
        bresenham_calculus_y(objet, k, m, 1, 0);

    }
    bresenham_calculus_x(objet, 0, m, objet.obj.len() - 1, 0);
    bresenham_calculus_y(objet, 0, m, objet.obj.len() - 1, 0);


}
fn bresenham_calculus_x(objet: &mut ObjectDraw, size: usize, m: &mut PixelsCoordinate, upper: usize, lower: usize)
{
    let l = 0; 
    let u = upper;
    let k = size - lower;

    let mut x0 = objet.obj[k+l].x as f32;  let mut x1 = objet.obj[k+u].x as f32;
    let y0 = objet.obj[k+l].y as f32;  let y1 = objet.obj[k+u].y as f32;

    if x0 > x1
    {
        let tmp = x0;
        x0 = x1;
        x1 = tmp;

    }

  

    for x in x0 as i32 .. (x1 as i32 + 1)
    {   
        let y = ( ( y1 - y0 ) / ( x1 - x0 ) * ( x as f32 - x0 ) ) + y0;

        m.coord.push(Item { x: x, y: y as i32, color: objet.obj[k+l].color });

    }

}

fn bresenham_calculus_y(objet: &mut ObjectDraw, size: usize, m: &mut PixelsCoordinate, upper: usize, lower: usize)
{
    let l = 0; 
    let u = upper;
    let k = size - lower;


    let x0 = objet.obj[k+l].x as f32;  let x1 = objet.obj[k+u].x as f32;
    let mut y0 = objet.obj[k+l].y as f32;  let mut y1 = objet.obj[k+u].y as f32;

    if y0 > y1
    {
        let tmp = y0;
        y0 = y1;
        y1 = tmp;
    }
    else if y0 == y1
    {
        return ;
    }




    for y in y0 as i32 .. (y1 as i32 + 1)
    {   
        let x = ( ( x1 - x0 ) /  ( y1 - y0 ) * ( y as f32 - y0 ) ) + x0;

        m.coord.push(Item { x: x as i32, y: y, color: objet.obj[k+l].color });


    }



}




