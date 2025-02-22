mod rate;
mod rk4;

mod ui;

use egui::{Window, widgets};
use egui_plot::{Legend, Line, Plot, PlotPoints};

type Float = f64;

fn main() {
    let conf = miniquad::conf::Conf {
        high_dpi: true,
        fullscreen: true,
        ..Default::default()
    };

    let app = |_mq_ctx: &mut dyn miniquad::RenderingBackend, egui_ctx: &egui::Context| {
        Window::new("Console").title_bar(true).show(egui_ctx, |ui| {
            ui.label("Theme");
            widgets::global_dark_light_mode_buttons(ui);
            ui.end_row();

            ui.label("Source code");
            ui.hyperlink_to(
                format!("{0} Open on GitHub", egui::special_emojis::GITHUB),
                "https://github.com/Richardn2002/hodgkin-huxley-playground",
            );
            ui.end_row();
        });

        Window::new("Rate Functions").show(egui_ctx, |ui| {
            let plot = Plot::new("rate function plot").legend(Legend::default());
            plot.show(ui, |plot_ui| {
                plot_ui.line(
                    Line::new(PlotPoints::from_explicit_callback(
                        rate::alpha_m,
                        -50.0..150.0,
                        200,
                    ))
                    .name("α_m"),
                );
                plot_ui.line(
                    Line::new(PlotPoints::from_explicit_callback(
                        rate::beta_m,
                        -50.0..150.0,
                        200,
                    ))
                    .name("β_m"),
                );
                plot_ui.line(
                    Line::new(PlotPoints::from_explicit_callback(
                        rate::alpha_h,
                        -50.0..150.0,
                        200,
                    ))
                    .name("α_h"),
                );
                plot_ui.line(
                    Line::new(PlotPoints::from_explicit_callback(
                        rate::beta_h,
                        -50.0..150.0,
                        200,
                    ))
                    .name("β_h"),
                );
                plot_ui.line(
                    Line::new(PlotPoints::from_explicit_callback(
                        rate::alpha_n,
                        -50.0..150.0,
                        200,
                    ))
                    .name("α_n"),
                );
                plot_ui.line(
                    Line::new(PlotPoints::from_explicit_callback(
                        rate::beta_n,
                        -50.0..150.0,
                        200,
                    ))
                    .name("β_n"),
                );
            });
        });
    };

    miniquad::start(conf, move || Box::new(ui::Backend::new(app)));
}
