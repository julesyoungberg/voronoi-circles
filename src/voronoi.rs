use nannou::prelude::*;

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
pub fn get_points(time: f32, grid_res_x: usize, grid_res_y: usize) -> Vec<Vec<Vector2<f64>>> {
    let mut points = vec![vec![pt2(0.0 as f64, 0.0 as f64); grid_res_x]; grid_res_y];

    for y in 0..grid_res_y {
        for x in 0..grid_res_x {
            let bin_coord = pt2(x as f64, y as f64);
            let point = rand2(bin_coord);
            points[y][x] = bin_coord + animate_point(point, time);
        }
    }

    return points;
}

fn map_voronoi_point(
    point: Vector2<f64>,
    width: u32,
    height: u32,
    grid_res_x: usize,
    grid_res_y: usize,
) -> Vector2 {
    let normalized = pt2(
        point.x as f64 / grid_res_x as f64,
        point.y as f64 / grid_res_y as f64,
    );
    let mapped = pt2(
        normalized.x * width as f64 - width as f64 / 2.0,
        normalized.y * height as f64 - height as f64 / 2.0,
    );
    return pt2(mapped.x as f32, mapped.y as f32);
}

// compute the maximum radius for each point such that the circle is contained in the cell
pub fn get_radiuses(
    points: &Vec<Vec<Vector2<f64>>>,
    grid_res_x: usize,
    grid_res_y: usize,
) -> Vec<Vec<f64>> {
    let mut radiuses = vec![vec![0.0 as f64; grid_res_x]; grid_res_y];

    for y in 0..grid_res_y {
        for x in 0..grid_res_x {
            let center_point = points[y][x];
            let mut radius: f64 = 0.0;
            // let mut nearest_point = center_point;

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
                    if nx < 0 || ny < 0 || nx >= grid_res_x as i32 || ny >= grid_res_y as i32 {
                        continue;
                    }

                    let neighbor = points[ny as usize][nx as usize];

                    let diff = center_point - neighbor;
                    let hyp = diff.x * diff.x + diff.y * diff.y;
                    let dist = hyp.sqrt() / 2.0;

                    if radius == 0.0 || dist < radius {
                        radius = dist;
                        // nearest_point = neighbor;
                    }
                }
            }

            radiuses[y][x] = radius;

            // draw line to nearest
            // let start = map_voronoi_point(center_point, size);
            // let end = map_voronoi_point(nearest_point, size);
            // draw.line()
            //     .start(start)
            //     .end(end)
            //     .weight(1.0)
            //     .color(rgb(255 as u8, 0, 0));
        }
    }

    return radiuses;
}

// draw the voronoi points as circles with the corresponding radius
pub fn draw_circles(
    points: &Vec<Vec<Vector2<f64>>>,
    radiuses: &Vec<Vec<f64>>,
    draw: &Draw,
    width: u32,
    height: u32,
    grid_res_x: usize,
    grid_res_y: usize,
    cell_size: f64,
) {
    for y in 0..grid_res_y {
        for x in 0..grid_res_x {
            let point = points[y][x];
            let mapped = map_voronoi_point(point, width, height, grid_res_x, grid_res_y);
            let radius = radiuses[y][x] as f32 * cell_size as f32 * 2.0;

            let x = (point.x % 1.0).abs() * 255.0;
            let y = (point.y % 1.0).abs() * 255.0;
            let color = rgb(x as u8, y as u8, (radius * 4.0) as u8);

            draw.ellipse()
                .xy(mapped)
                .wh(pt2(radius, radius))
                .color(color);
        }
    }
}

pub fn draw_grid(draw: &Draw, width: u32, height: u32, grid_res_x: usize, grid_res_y: usize) {
    // draw horizontal lines
    for y in 1..grid_res_y {
        let start = map_voronoi_point(pt2(0.0, y as f64), width, height, grid_res_x, grid_res_y);
        let end = map_voronoi_point(
            pt2(grid_res_x as f64, y as f64),
            width,
            height,
            grid_res_x,
            grid_res_y,
        );
        draw.line()
            .start(start)
            .end(end)
            .weight(2.0)
            .color(rgb(255 as u8, 0, 0));
    }

    // draw vertical lines
    for x in 1..grid_res_x {
        let start = map_voronoi_point(pt2(x as f64, 0.0), width, height, grid_res_x, grid_res_y);
        let end = map_voronoi_point(
            pt2(x as f64, grid_res_y as f64),
            width,
            height,
            grid_res_x,
            grid_res_y,
        );
        draw.line()
            .start(start)
            .end(end)
            .weight(2.0)
            .color(rgb(255 as u8, 0, 0));
    }
}
