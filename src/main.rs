use eframe::{egui::CentralPanel, run_native, App, NativeOptions};
use egui::{Layout, Color32};
use egui::{Id, SidePanel, TopBottomPanel};

// Setup Card Stuff
#[derive(serde::Deserialize, serde::Serialize)]
struct Card {
    name: String,
    uri: String,
    image_uri: String,
    release_date: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
struct CardCollection {
    name: String,
    cards: Vec<Card>,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct Perseus {
    // Example stuff:
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,

    default_collection: CardCollection,
}

impl Default for Perseus {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            default_collection: CardCollection {
                name: "Default".to_string(),
                cards: vec![
                Card{
                    name: "Firesong and Sunspeaker".to_string(),
                    uri: "https://api.scryfall.com/cards/298f4325-32be-41be-99f3-bd7c27226918".to_string(),
                    image_uri: "https://cards.scryfall.io/png/front/2/9/298f4325-32be-41be-99f3-bd7c27226918.jpg?1681411651".to_string(),
                    release_date: "2022-07-08".to_string()
                },
                Card{
                    name: "Niv-Mizzet, Parun".to_string(),
                    uri: "https://api.scryfall.com/cards/3035a97f-5104-4b56-84a4-5206e75607fc".to_string(),
                    image_uri: 	"https://cards.scryfall.io/png/front/3/0/3035a97f-5104-4b56-84a4-5206e75607fc.jpg?1674142512".to_string(),
                    release_date: "2022-06-10".to_string()
                },
                Card{
                    name: "Painful Lesson".to_string(),
                    uri: "https://api.scryfall.com/cards/94bc63b7-a6d8-40b9-86f1-111f8421af8c".to_string(),
                    image_uri: "https://cards.scryfall.io/png/front/9/4/94bc63b7-a6d8-40b9-86f1-111f8421af8c.jpg?1619739406".to_string(),
                    release_date: "2021-04-24".to_string()
                },
                
                ]
            }
        }
    }
}

impl Perseus {
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

impl App for Perseus {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        // Side
        SidePanel::new(egui::panel::Side::Left, Id::new("LeftSidePanel")).show(ctx, |ui| {
            ui.add_space(10.0);
            ui.label("+ Create a new collection");


            // Side Bottom
            TopBottomPanel::bottom(Id::new("SideBottomPanel")).show(ctx, |ui|
            {
                ui.label("Created By Nyako");
            })
        });
        // Middle
        CentralPanel::default().show(ctx, |ui| {
            egui::Grid::new("Card_Grid").show(ui, |ui| {
                let mut it_time = 1;
                for c in &self.default_collection.cards {
                    if it_time > 3 {
                        ui.end_row();
                        it_time = 0;
                    }
                    ui.group(|ui| {
                        ui.with_layout(Layout::top_down(egui::Align::Center), |ui| {
                            let img = egui::Image::new(&c.image_uri).bg_fill(Color32::TRANSPARENT);
                            ui.add(img.fit_to_original_size(0.5));
                            ui.label(&c.name);
                            //ui.label(&c.uri);
                            ui.label(&c.release_date);
                            
                        });
                    }).response.on_hover_ui(|ui| {
                        ui.set_max_width(200.0);
                        ui.with_layout(Layout::top_down(egui::Align::Center), |ui| {
                            let moveto = ui.button("-> Move to Collection");
                            let delete = ui.button("X Delete");
                            if delete.clicked() {
                                println!("Clicked Delete");
                            }
                            if moveto.clicked() {
                                println!("Clicked Move")
                            }
                        });
                    });
                    it_time += 1;
                }
            });
        });
    }
}

fn main() {
    env_logger::init();
    let win_opt = NativeOptions::default();
    let _ = run_native(
        "Perseus",
        win_opt,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::new(Perseus::new(cc))
        }),
    );
}
