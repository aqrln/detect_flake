use eframe::egui;

use crate::opt::Opt;

pub fn start(opt: Opt) -> Result<(), eframe::Error> {
    let native_options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        ..Default::default()
    };

    eframe::run_native(
        "detect_flake",
        native_options,
        Box::new(|_| Box::new(App::new(opt))),
    )
}

struct App {
    opt: Opt,
    success_count: u32,
    failure_count: u32,
    running: bool,
    output: String,
}

impl App {
    pub fn new(opt: Opt) -> Self {
        Self {
            opt,
            success_count: 0,
            failure_count: 0,
            running: false,
            output: String::with_capacity(4096),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let completed = self.success_count + self.failure_count;
        let total = self.opt.runs_per_thread * self.opt.threads;
        let progress = completed as f32 / total as f32;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label("Command:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.opt.command)
                            .desired_width(f32::INFINITY),
                    );
                });

                ui.separator();

                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.label("Threads");
                            ui.add(egui::DragValue::new(&mut self.opt.threads));
                        });
                        ui.horizontal(|ui| {
                            ui.label("Runs per thread");
                            ui.add(egui::DragValue::new(&mut self.opt.runs_per_thread));
                        });

                    });

                    ui.separator();

                    ui.add(
                        egui::TextEdit::multiline(&mut self.output)
                            .desired_width(f32::INFINITY)
                            .desired_rows(20),
                    );
                });

                ui.separator();

                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::default()), |ui| {
                        let button_label = if self.running { "Stop" } else { "Start" };
                        let button_color = if self.running {
                            egui::Color32::LIGHT_RED
                        } else {
                            egui::Color32::LIGHT_BLUE
                        };

                        if ui
                            .add(
                                egui::Button::new(button_label)
                                    .min_size(egui::vec2(40.0, 0.0))
                                    .fill(button_color),
                            )
                            .clicked()
                        {
                            self.running = !self.running;
                        }

                        ui.add(egui::ProgressBar::new(progress).text(format!(
                            "{completed}/{total} ({}%)",
                            (progress * 100.0).round()
                        )));
                    });
                });
            });
        });
    }
}
