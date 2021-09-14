use nannou::prelude::*;

mod voronoi;

static SIZE: u32 = 1024;

fn main() {
    nannou::sketch(view).size(SIZE, SIZE).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    let points = voronoi::get_points(app.time);
    let radi = voronoi::get_radiuses(&points, &draw, SIZE as f64);
    voronoi::draw_circles(&points, &radi, &draw, SIZE as f64);
    // voronoi::draw_grid(&draw, SIZE as f64);

    draw.to_frame(app, &frame).unwrap();
}
