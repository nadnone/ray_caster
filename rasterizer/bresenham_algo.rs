
use crate::rasterizer::misc::*;

pub fn bresenham_algo(m: &mut PixelsCoordinate, triangle: &Triangle)
{
    if triangle.data.len() == 0
    {
        println!("empty object");
        return;
    }


    bresenham_calculus(triangle.data, 0, m, 1, triangle.color);
    bresenham_calculus(triangle.data, 0, m, 2, triangle.color);

    bresenham_calculus(triangle.data, 1, m, 0, triangle.color);
    bresenham_calculus(triangle.data, 1, m, 2, triangle.color);

    bresenham_calculus(triangle.data, 2, m, 0, triangle.color);
    bresenham_calculus(triangle.data, 2, m, 1, triangle.color);



}
fn bresenham_calculus(objet: [Item; 3], low: usize, m: &mut PixelsCoordinate, upper: i32, color: [u8; 3])
{

    let u = upper as usize;
    let l = low; 

    
    let x0 = objet[l].x as f32;  let x1 = objet[u].x as f32;
    let y0 = objet[l].y as f32;  let y1 = objet[u].y as f32;


    for x in objet[l].x .. (objet[u].x + 1)
    {   
        let y = ( ( y1 - y0 ) / ( x1 - x0 ) * ( x as f32 - x0 ) ) + y0;

        m.coord.push( Item::new(x, y as i32, color));

    }



    for y in objet[l].y .. (objet[u].y + 1)
    {   
        let x = ( ( x1 - x0 ) /  ( y1 - y0 ) * ( y as f32 - y0 ) ) + x0;

        m.coord.push( Item::new(x as i32, y, color));



    }

    

}


