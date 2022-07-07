//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] 

mod data_structs;
mod memory_management;

use eframe::egui;
use std::{process::Command, borrow::{Borrow, BorrowMut}};
use rfd::FileDialog;

fn main() {
	//println!("{}", sh3_prob_map.len());
	let options = eframe::NativeOptions::default();
	
	let mut my_app: data_structs::MyApp = data_structs::MyApp::default();
	my_app.init();

    my_app.testerino.bidello.insert("lello".to_string(), data_structs::miniTest{lello: 5});

    
    
    eframe::run_native(
        "Silent Hill Rustomizer",
        options,
        Box::new(|_cc| Box::new(my_app)),
    );


}

impl eframe::App for data_structs::MyApp {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Silent Hill Rustomizer");
           
            ui.horizontal(|ui| {

				ui.vertical(|ui|{
					ui.label(self.sliders[0].main_name.to_string());
					ui.add(egui::Slider::new(
                        &mut self.sliders[0].main, 
                        0..=100)
                        .text("Likelihood"));
					
					ui.label("HighScore: ");
                    ui.label(self.high_score.to_string());
				});
            });
           


			if ui.button("Find SH3").clicked(){
				if let Some(path) = FileDialog::new()
                    .add_filter("Executable", &["exe"])
                    .pick_file()
                {
                    self.sh3_exe_name = path.file_name().unwrap().to_str().unwrap().to_owned();
                    self.sh3_path = path.display().to_string();
                }
			}

			if ui.button("Click to start SH3").clicked() {
                let sh3_process = Command::new(&self.sh3_path)
				.spawn()
				.expect("failed to execute process");

                
            }

			if ui.button("Update Probs").clicked() {
                self.set_probability();
                self.high_score = memory_management::read_highscore();
            }
           
            ui.label(format!("Hello '{}', age {}", self.name, self.age));

            ui.label(format!("The SH3 exe path: {}", self.sh3_path));
        });
    }

    fn on_exit_event(&mut self) -> bool {
        true
    }

    fn on_exit(&mut self, _gl: &eframe::glow::Context) {}

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(30)
    }

    fn max_size_points(&self) -> egui::Vec2 {
        egui::Vec2::INFINITY
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> egui::Rgba {
        // NOTE: a bright gray makes the shadows of the windows look weird.
        // We use a bit of transparency so that if the user switches on the
        // `transparent()` option they get immediate results.
        egui::Color32::from_rgba_unmultiplied(12, 12, 12, 180).into()

        // _visuals.window_fill() would also be a natural choice
    }

    fn persist_native_window(&self) -> bool {
        true
    }

    fn persist_egui_memory(&self) -> bool {
        true
    }

    fn warm_up_enabled(&self) -> bool {
        false
    }
}
