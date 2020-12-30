use nannou::prelude::*;

pub static GRID_RES: usize = 3;

// classic GLSL-style random vector generator
fn rand2(c: Vector2<f64>) -> Vector2<f64> {
    let x = c.dot(pt2(127.1, 311.7)).sin();
    let y = c.dot(pt2(269.5, 183.3)).sin();
    let p = pt2(x, y) * 43758.5453 % 1.0;
    return pt2(p.x.abs(), p.y.abs());
}

fn animate_point(p: Vector2<f64>, time: f32) -> Vector2<f64> {
    let x = time as f64 + p.x * 2.0 * PI as f64;
    let y = time as f64 + p.y * 2.0 * PI as f64;
    return pt2(x.sin() * 0.5 + 0.5, y.sin() * 0.5 + 0.5);
}

// generate a grid of random points
pub fn get_points(time: f32) -> Vec<Vec<Vector2<f64>>> {
    let mut points = vec![vec![pt2(0.0 as f64, 0.0 as f64); GRID_RES]; GRID_RES];

    for y in 0..GRID_RES {
        for x in 0..GRID_RES {
            let bin_coord = pt2(x as f64, y as f64);
            let point = rand2(bin_coord);
            points[x][y] = bin_coord + animate_point(point, time);
        }
    }

    return points;
}

fn map_veroni_point(point: Vector2<f64>, size: f64) -> Vector2 {
    let res = GRID_RES as f64;
    let normalized = pt2(point.x as f64 / res, point.y as f64 / res);
    let mapped = pt2(
        normalized.x * size - size / 2.0,
        normalized.y * size - size / 2.0,
    );
    return pt2(mapped.x as f32, mapped.y as f32);
}

fn scale_veroni_value(v: f64, size: f64) -> f32 {
    let normalized = v / GRID_RES as f64;
    let mapped = normalized * size;
    return mapped as f32;
}

// compute the maximum radius for each point such that the circle is contained in the cell
pub fn get_radiuses(points: &Vec<Vec<Vector2<f64>>>, draw: &Draw, size: f64) -> Vec<Vec<f64>> {
    let mut radiuses = vec![vec![0.0 as f64; GRID_RES]; GRID_RES];

    for y in 0..GRID_RES {
        for x in 0..GRID_RES {
            let center_point = points[x][y];
            let mut radius: f64 = 0.0;
            let mut nearest_point = center_point;

            // search neighbors for nearest
            for yoff in 0..3 {
                let yoffset = yoff - 1;

                for xoff in 0..3 {
                    let xoffset = xoff - 1;

                    if yoffset == 0 && xoffset == 0 {
                        continue;
                    }

                    let nx = x as i32 + xoffset;
                    let ny = y as i32 + yoffset;
                    if nx < 0 || ny < 0 || nx >= GRID_RES as i32 || ny >= GRID_RES as i32 {
                        continue;
                    }

                    let neighbor = points[nx as usize][ny as usize];

                    let diff = center_point - neighbor;
                    let hyp = diff.x * diff.x + diff.y * diff.y;
                    let dist = hyp.sqrt() / 2.0;

                    if radius == 0.0 || dist < radius {
                        radius = dist;
                        nearest_point = neighbor;
                    }
                }
            }

            radiuses[x as usize][y as usize] = radius;

            let mapped = map_veroni_point(center_point, size);
            let csize = scale_veroni_value(radius, size);

            let cx = (center_point.x % 1.0).abs() * 255.0;
            let cy = (center_point.y % 1.0).abs() * 255.0;
            let color = rgb(0, cx as u8, cy as u8);

            draw.ellipse().xy(mapped).wh(pt2(csize, csize)).color(color);

            if x == 1 && y == 0 {
                let start = map_veroni_point(center_point, size);
                let end = map_veroni_point(nearest_point, size);
                draw.line()
                    .start(start)
                    .end(end)
                    .weight(1.0)
                    .color(rgb(255 as u8, 0, 0));
            }
        }
    }

    return radiuses;
}

// draw the veroni points as circles with the corresponding radius
pub fn draw_circles(
    points: &Vec<Vec<Vector2<f64>>>,
    radiuses: &Vec<Vec<f64>>,
    draw: &Draw,
    size: f64,
) {
    for y in 0..GRID_RES {
        for x in 0..GRID_RES {
            let point = points[x as usize][y as usize];
            let mapped = map_veroni_point(point, size);
            let radius = scale_veroni_value(radiuses[x as usize][y as usize], size);

            let x = (point.x % 1.0).abs() * 255.0;
            let y = (point.y % 1.0).abs() * 255.0;
            let color = rgb(0, x as u8, y as u8);

            draw.ellipse()
                .xy(mapped)
                .wh(pt2(radius, radius))
                .color(color);
        }
    }
}

pub fn draw_grid(draw: &Draw, size: f64) {
    let res = GRID_RES as f64;

    // draw horizontal lines
    for y in 1..GRID_RES {
        let start = map_veroni_point(pt2(0.0, y as f64), size);
        let end = map_veroni_point(pt2(res, y as f64), size);
        draw.line()
            .start(start)
            .end(end)
            .weight(2.0)
            .color(rgb(255 as u8, 0, 0));
    }

    // draw vertical lines
    for x in 1..GRID_RES {
        let start = map_veroni_point(pt2(x as f64, 0.0), size);
        let end = map_veroni_point(pt2(x as f64, res), size);
        draw.line()
            .start(start)
            .end(end)
            .weight(2.0)
            .color(rgb(255 as u8, 0, 0));
    }
}
