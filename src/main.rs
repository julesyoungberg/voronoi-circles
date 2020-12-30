use nannou::prelude::*;

mod veroni;

static SIZE: u32 = 1024;

fn main() {
    nannou::sketch(view).size(SIZE, SIZE).run();
}

fn map_veroni_point(point: Vector2<f64>) -> Vector2 {
    let res = veroni::GRID_RES as f32;
    let size = SIZE as f32;
    let normalized = pt2(point.x as f32 / res, point.y as f32 / res);
    return pt2(
        normalized.x * size - size / 2.0,
        normalized.y * size - size / 2.0,
    );
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    let points = veroni::get_points();
    for row in points {
        for point in row {
            let mapped = map_veroni_point(point);
            draw.ellipse().xy(mapped).color(STEELBLUE);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
