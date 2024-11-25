#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{egui};
use ehttp;

use std::thread;
use std::thread::JoinHandle;
use log::debug;
use crate::audio_player::{play_test_audio};

use crate::lib::AppState;

pub(crate) fn main_window(db_uri: &String) -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    let mut app = MyApp {
        name: "Arthur".to_owned(),
        age: 42,
        state: AppState {
            api_thread: None,
            broker_thread: None,
            player_thread: None
        }
    };
    app.state.start(db_uri.to_string());

    eframe::run_native(
        "Soundshouter",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::new(app)
        }),
    )
}

struct MyApp {
    name: String,
    age: u32,
    state: AppState
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Soundshouter");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Increment").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));

            // ui.image(egui::include_image!(
            //     "../../../crates/egui/assets/ferris.png"
            // ));

            ui.heading("Status");
            if self.state.api_thread.is_some() { ui.label("MQTT Broker running"); }
            else { ui.label("MQTT Broker stopped"); }
            if self.state.api_thread.is_some() { ui.label("API running"); }
            else { ui.label("API stopped"); }
            if self.state.api_thread.is_some() { ui.label("Audio running"); }
            else { ui.label("Audio stopped"); }

            if ui.button("test sound").clicked() {
                self.state.new_sound_thread();
            }

            if ui.button("test api request").clicked() {
                let request = ehttp::Request::post(
                    "http://127.0.0.1:8000/api/v1/play/1", vec![]);
                ehttp::fetch(request, move |result: ehttp::Result<ehttp::Response>| {

                    debug!("Status code: {:?}", result.unwrap().status);
                });
            }
        });
    }
}
