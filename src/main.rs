use eframe::egui;
use egui::Color32;
use egui::RichText;
use egui_file_dialog::FileDialog;
use std::ffi::OsStr;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Signal processing Toolbox",
        native_options,
        Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))),
    )
    .expect("Error loading the signal processing toolbox");
}

#[derive(Default)]
struct MyEguiApp {
    file_dialog: FileDialog,
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self {
            file_dialog: FileDialog::new(),
        }
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        //egui::style::Spacing::item_spacing::egui::Vec2::new(2.2, 2.2); //Define spacing between items

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label(
                    egui::RichText::new("Welcome to signal processing toolbox ! ")
                        .heading()
                        .color(egui::Color32::from_rgb(255, 255, 255)),
                );
                ui.separator();
                ui.label("Pick a mp3 file to start the transformations");
                if ui
                    .button(RichText::new("Search").color(Color32::WHITE))
                    .clicked()
                {
                    self.file_dialog.pick_file();
                }
                if let Some(path) = self.file_dialog.update(ctx).picked() {
                    if path.extension() != Some(OsStr::new("mp3")) {
                        ui.label(
                            RichText::new("Wrong file extension, try again").color(Color32::RED),
                        );
                    } else {
                        ui.label("File picked under this path : ");
                        ui.label(RichText::new(path.display().to_string()));
                        ui.columns(3, |columns| {
                            columns[0].button("Play original");
                            columns[1].button("Effet 2");
                            columns[2].button("Effet 3");
                        });
                    }
                }
            });
        });
    }
}
