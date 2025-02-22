mod rk4;
mod ui;

type Float = f64;

fn main() {
    let conf = miniquad::conf::Conf {
        high_dpi: true,
        fullscreen: true,
        ..Default::default()
    };

    let app = |_mq_ctx: &mut dyn miniquad::RenderingBackend, egui_ctx: &egui::Context| {
        egui::Window::new("egui ❤ miniquad")
            .title_bar(true)
            .show(egui_ctx, |ui| {
                egui::widgets::global_dark_light_mode_buttons(ui);
            });
    };

    miniquad::start(conf, move || Box::new(ui::Backend::new(app)));
}
