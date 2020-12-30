use nannou::prelude::*;

mod veroni;

static SIZE: u32 = 1024;

fn main() {
    nannou::sketch(view).size(SIZE, SIZE).run();
}

fn map_veroni_point(point: &Vector2<f64>) -> Vector2 {
    let res = veroni::GRID_RES as f32;
    let size = SIZE as f32;
    let normalized = pt2(point.x as f32 / res, point.y as f32 / res);
    return pt2(
        normalized.x * size - size / 2.0,
        normalized.y * size - size / 2.0,
    );
}

fn scale_veroni_value(v: f64) -> f32 {
    let size = SIZE as f64;
    let normalized = v / veroni::GRID_RES as f64;
    let mapped = normalized * size;
    return mapped as f32;
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    let points = veroni::get_points();
    let radiuses = veroni::get_radiuses(&points);
    for y in 0..veroni::GRID_RES {
        for x in 0..veroni::GRID_RES {
            let mapped = map_veroni_point(&points[x as usize][y as usize]);
            let radius = scale_veroni_value(radiuses[x as usize][y as usize]) / 2.0;

            draw.ellipse()
                .xy(mapped)
                .wh(pt2(radius, radius))
                .color(STEELBLUE);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
