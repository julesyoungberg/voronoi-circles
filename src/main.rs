use nannou::prelude::*;

mod voronoi;

const SIZE: u32 = 1080;

fn main() {
    nannou::sketch(view).size(SIZE, SIZE).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    let points = voronoi::get_points(app.time);
    let radi = voronoi::get_radiuses(&points);
    voronoi::draw_circles(&points, &radi, &draw, SIZE as f64);
    voronoi::draw_grid(&draw, SIZE as f64);

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
