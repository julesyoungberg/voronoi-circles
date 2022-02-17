use nannou::prelude::*;

mod voronoi;

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 576;
const CELL_SIZE: f64 = 54.0;

fn main() {
    nannou::sketch(view).size(WIDTH, HEIGHT).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    let grid_res_x = (WIDTH as f64 / CELL_SIZE).ceil() as usize;
    let grid_res_y = (HEIGHT as f64 / CELL_SIZE).ceil() as usize;

    let points = voronoi::get_points(app.time, grid_res_x, grid_res_y);
    let radi = voronoi::get_radiuses(&points, grid_res_x, grid_res_y);
    voronoi::draw_circles(
        &points, &radi, &draw, WIDTH, HEIGHT, grid_res_x, grid_res_y, CELL_SIZE,
    );
    // voronoi::draw_grid(&draw, WIDTH, HEIGHT, grid_res_x, grid_res_y);

    draw.to_frame(app, &frame).unwrap();

    // Capture the frame!
    let file_path = captured_frame_path(app, &frame);
    app.main_window().capture_frame(file_path);
}

fn captured_frame_path(app: &App, frame: &Frame) -> std::path::PathBuf {
    // Create a path that we want to save this frame to.
    app.project_path()
        .expect("failed to locate `project_path`")
        // Capture all frames to a directory called `/<path_to_nannou>/nannou/simple_capture`.
        .join("frames")
        // Name each file after the number of the frame.
        .join(format!("{:03}", frame.nth()))
        // The extension will be PNG. We also support tiff, bmp, gif, jpeg, webp and some others.
        .with_extension("png")
}
