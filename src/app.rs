const LOREM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Etiam mattis semper pharetra. Nulla commodo lectus vel purus varius aliquet. Interdum et malesuada fames ac ante ipsum primis in faucibus. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae; Phasellus eros sapien, ullamcorper ut sapien facilisis, volutpat cursus libero. Vivamus in purus rhoncus, facilisis mauris non, interdum tellus. In aliquam vitae lorem eget tristique. Fusce id velit eget leo sollicitudin malesuada. Vivamus a massa vestibulum, tempor nunc at, iaculis lacus. Nulla non velit commodo, cursus enim sit amet, ullamcorper ipsum. Donec varius leo tortor, finibus sollicitudin magna mollis et. Pellentesque magna purus, pulvinar id varius at, tincidunt tincidunt felis.";

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
#[derive(Default)] // if we add new fields, give them default values when deserializing old state
pub struct FishTool {
    vessel_parameters: VesselParameters,
}
#[derive(serde::Deserialize, serde::Serialize)]
struct VesselParameters {
    name: String,
    length: f32,
    beam: f32,
}

impl Default for VesselParameters {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            length: f32::default(),
            beam: f32::default(),
        }
    }
}

impl FishTool {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for FishTool {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui
        egui::SidePanel::left("properties_panel")
            .resizable(false)
            .min_width(250.0)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.heading("Vessel Properties");
                    ui.horizontal(|ui| {
                        ui.label("Name:");
                        ui.add(
                            egui::TextEdit::singleline(&mut self.vessel_parameters.name)
                                .hint_text("Vessel name"),
                        );
                    });
                    ui.horizontal(|ui| {
                        ui.label("Length:");
                        ui.add(egui::widgets::DragValue::new(
                            &mut self.vessel_parameters.length,
                        ));
                    });
                    ui.horizontal(|ui| {
                        ui.label("Beam:");
                        ui.add(egui::widgets::DragValue::new(
                            &mut self.vessel_parameters.beam,
                        ));
                    });
                    ui.add_space(20.0);
                    ui.heading("Operational Properties")
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.vertical_centered(|ui| {
                ui.heading("PONTOS Fish Lightning");
                ui.label("A tool for determining feasibility of electrifying fishing vessels");
            });
            ui.separator();
            ui.vertical_centered(|ui| {
                ui.heading(format!("Report - {}", &self.vessel_parameters.name));
            });
            ui.heading("Regulatory information");
            ui.label(LOREM);
            ui.add_space(20.0);
            ui.heading("Technical information");
            ui.label(LOREM);
            ui.add_space(20.0);
            ui.heading("Environmental information");
            ui.label(LOREM);
            ui.add_space(20.0);
            ui.heading("Economic information");
            ui.label(LOREM);
        });
    }
}
