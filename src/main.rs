use nannou::prelude::*;

mod veroni;

static SIZE: u32 = 1024;

fn main() {
    nannou::sketch(view).size(SIZE, SIZE).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    let points = veroni::get_points(app.time);
    let radi = veroni::get_radiuses(&points, &draw, SIZE as f64);
    veroni::draw_circles(&points, &radi, &draw, SIZE as f64);
    // veroni::draw_grid(&draw, SIZE as f64);

    draw.to_frame(app, &frame).unwrap();
}
