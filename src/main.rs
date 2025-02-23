mod hh;
mod rate;
mod rk4;

mod ui;

use egui::{DragValue, FontId, Grid, RichText, Window, widgets};
use egui_plot::{Legend, Line, Plot, PlotPoints};

use std::{cell::RefCell, rc::Rc};

type Float = f64;

fn main() {
    let sim_state = Rc::<RefCell<hh::State>>::new(RefCell::new(hh::State::default()));

    let app = move |_mq_ctx: &mut dyn miniquad::RenderingBackend, egui_ctx: &egui::Context| {
        Window::new("Console").title_bar(true).show(egui_ctx, |ui| {
            Grid::new("console grid")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    ui.label("Theme");
                    widgets::global_dark_light_mode_buttons(ui);
                    ui.end_row();

                    ui.label("Source code");
                    ui.hyperlink_to(
                        format!("{0} Open on GitHub", egui::special_emojis::GITHUB),
                        "https://github.com/Richardn2002/hodgkin-huxley-playground",
                    );
                    ui.end_row();

                    ui.label("Powered by");
                    ui.hyperlink_to("Rust", "https://www.rust-lang.org/");
                    ui.end_row();
                    ui.label("");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.end_row();
                    ui.label("");
                    ui.hyperlink_to("miniquad", "https://github.com/not-fl3/miniquad");
                    ui.end_row();
                })
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

        Window::new("First-order Kinetics").show(egui_ctx, |ui| {
            ui.label("For a large collection of identical gates, define");
            ui.label(RichText::new("τ_x = 1/(α_x + β_x)").font(FontId::proportional(20.0)));
            ui.label(RichText::new("x_∞ = α_x/(α_x + β_x)").font(FontId::proportional(20.0)));
            ui.label("Then gate dynamics follow,");
            ui.label(RichText::new("τ_x dx/dt = -x + x_∞").font(FontId::proportional(20.0)));

            let plot = Plot::new("rate function plot")
                .height(ui.available_height())
                .legend(Legend::default());
            plot.show(ui, |plot_ui| {
                plot_ui.line(
                    Line::new(PlotPoints::from_explicit_callback(
                        hh::tau_m,
                        -150.0..100.0,
                        250,
                    ))
                    .name("τ_m"),
                );
                plot_ui.line(
                    Line::new(PlotPoints::from_explicit_callback(
                        hh::m_inf,
                        -150.0..100.0,
                        250,
                    ))
                    .name("m_∞"),
                );
                plot_ui.line(
                    Line::new(PlotPoints::from_explicit_callback(
                        hh::tau_h,
                        -150.0..100.0,
                        250,
                    ))
                    .name("τ_h"),
                );
                plot_ui.line(
                    Line::new(PlotPoints::from_explicit_callback(
                        hh::h_inf,
                        -150.0..100.0,
                        250,
                    ))
                    .name("h_∞"),
                );
                plot_ui.line(
                    Line::new(PlotPoints::from_explicit_callback(
                        hh::tau_n,
                        -150.0..100.0,
                        250,
                    ))
                    .name("τ_n"),
                );
                plot_ui.line(
                    Line::new(PlotPoints::from_explicit_callback(
                        hh::n_inf,
                        -150.0..100.0,
                        250,
                    ))
                    .name("n_∞"),
                );
            });
        });

        Window::new("Full Simulation").show(egui_ctx, |ui| {
            let mut state = sim_state.borrow_mut();

            Grid::new("settings grid")
                .num_columns(8)
                .spacing([20.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    ui.label("Initial voltage");
                    ui.add(
                        DragValue::new(&mut state.setup.v0)
                            .range(-25.0..=100.0)
                            .speed(0.5),
                    );
                    ui.label("Simulation end");
                    ui.add(
                        DragValue::new(&mut state.setup.end)
                            .range(0.0..=50.0)
                            .speed(0.5),
                    );
                    ui.label("Step");
                    ui.add(
                        DragValue::new(&mut state.setup.dt)
                            .range(0.0..=1.0)
                            .speed(0.001),
                    );
                    ui.label("Steps per frame");
                    ui.add(
                        DragValue::new(&mut state.setup.steps_per_frame)
                            .range(1..=10000)
                            .speed(100),
                    );
                    ui.end_row();

                    ui.label("Pulse settings");
                    ui.label("");
                    ui.label("Start");
                    let limit = state.setup.pulse.end;
                    ui.add(
                        DragValue::new(&mut state.setup.pulse.start)
                            .range(0.0..=limit)
                            .speed(0.1),
                    );
                    ui.label("End");
                    let (start_limit, end_limit) = (state.setup.pulse.start, state.setup.end);
                    ui.add(
                        DragValue::new(&mut state.setup.pulse.end)
                            .range(start_limit..=end_limit)
                            .speed(0.1),
                    );
                    ui.label("Magnitude");
                    ui.add(
                        DragValue::new(&mut state.setup.pulse.magnitude)
                            .range(-20.0..=20.0)
                            .speed(1.0),
                    );
                    ui.end_row();
                });
            ui.separator();

            let plot = Plot::new("pulse plot");
            plot.show(ui, |plot_ui| {
                plot_ui.line(Line::new(PlotPoints::from_parametric_callback(
                    |t| match t {
                        0.0 => (0.0, 0.0),
                        1.0 => (state.setup.pulse.start, 0.0),
                        2.0 => (state.setup.pulse.start, state.setup.pulse.magnitude),
                        3.0 => (state.setup.pulse.end, state.setup.pulse.magnitude),
                        4.0 => (state.setup.pulse.end, 0.0),
                        5.0 => (state.setup.end, 0.0),
                        _ => panic!("Impossible t value."),
                    },
                    0.0..=5.0,
                    6,
                )));
            });
        });
    };

    miniquad::start(
        miniquad::conf::Conf {
            high_dpi: true,
            fullscreen: true,
            ..Default::default()
        },
        move || Box::new(ui::Backend::new(app)),
    );
}
