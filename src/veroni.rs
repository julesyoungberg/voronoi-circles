use nannou::prelude::*;

pub static GRID_RES: i32 = 8;

// classic GLSL-style random vector generator
fn rand2(c: Vector2<f64>) -> Vector2<f64> {
    let x = c.dot(pt2(127.1, 311.7)).sin();
    let y = c.dot(pt2(269.5, 183.3)).sin();
    let p = pt2(x, y) * 43758.5453 % 1.0;
    return pt2(p.x.abs(), p.y.abs());
}

// generates a grid of random points
pub fn get_points() -> Vec<Vec<Vector2<f64>>> {
    let size = GRID_RES as usize;
    let mut points = vec![vec![pt2(0.0 as f64, 0.0 as f64); size]; size];

    for y in 0..GRID_RES {
        for x in 0..GRID_RES {
            let bin_coord = pt2(x as f64, y as f64);
            let point = bin_coord + rand2(bin_coord);
            points[x as usize][y as usize] = point;
        }
    }

    return points;
}
