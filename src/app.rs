use std::{
    fmt::*
};

#[derive(serde::Deserialize, serde::Serialize, Default, PartialEq)]
pub enum Tab {
    #[default]
    SongEditor,
    PatternEditor,
    OscView,
    Mixer,
    FreeView
}

impl Display for Tab {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f,"{}", match self {
            Tab::SongEditor     => "Song editor",   
            Tab::PatternEditor  => "Pattern editor",   
            Tab::OscView        => "Osc. View",   
            Tab::Mixer          => "Mixer",   
            Tab::FreeView       => "Windowed view",   
        })?;
        Ok(())
    }
}

const TABS: [Tab;5] = [Tab::SongEditor, Tab::PatternEditor, Tab::OscView, Tab::Mixer, Tab::FreeView];

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] 
pub struct DawThingy{
    cur_tab: Tab
}

impl Default for DawThingy {
    fn default() -> Self {
        Self {
            cur_tab: Tab::SongEditor
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
            ui.horizontal_wrapped(|ui| {
                for t in TABS{
                    if ui.selectable_label(t == self.cur_tab, t.to_string()).clicked() {
                        self.cur_tab = t;
                    } 
                }
            })
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.cur_tab {
                Tab::SongEditor     => ui.label("This is the song editor!"),
                Tab::PatternEditor  => ui.label("I can already forsee all the hours spent in this pattern editor!"),
                Tab::OscView        => ui.label("I'm sure this will be used for a lot of youtube uploads!"),
                Tab::Mixer          => ui.label("The bane of my existence, the mixer!"),
                Tab::FreeView       => ui.label("there will be windows here..."),
            }
        });
    }
}
