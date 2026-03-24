/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] 
pub struct DawThingy{
}

impl Default for DawThingy {
    fn default() -> Self {
        Self {
        }
    }
}

impl DawThingy {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }
}

impl eframe::App for DawThingy {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menubar").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }
            });
        });
        
        egui::TopBottomPanel::top("buttons").show(ctx, |ui| {
             ui.label("This is where the buttons go")
        });
        
        egui::TopBottomPanel::top("tabs").show(ctx, |ui| {
            ui.label("This is where the tabs go"); 
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("This is where the editor goes"); 
        });
    }
}
