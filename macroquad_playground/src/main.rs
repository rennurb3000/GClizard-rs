use macroquad::prelude::*;
use lib_lizardrs::hello;
#[macroquad::main("helllo")]
async fn main() {
    hello();
    loop {
        clear_background(LIGHTGRAY);

        // Going 3d!

        set_camera(&Camera3D {
            position: vec3(-20., 15., 0.),
            up: vec3(0., 1., 0.),
            target: vec3(0., 0., 0.),
            ..Default::default()
        });


        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("egui ❤ macroquad")
                .show(egui_ctx, |ui| {
                    ui.label("Test");
                });
        });

        draw_grid(20, 1., BLACK, GRAY);
        draw_sphere_ex(vec3(0.0,0.0,0.0),1.0,None,BLUE,DrawSphereParams{rings:4,slices:4,draw_mode:DrawMode::Triangles,});
        draw_line_3d(vec3(-1.0,0.5,-1.0),vec3(1.0,-0.5,1.0),YELLOW);

        egui_macroquad::draw();
        next_frame().await
    }
}
