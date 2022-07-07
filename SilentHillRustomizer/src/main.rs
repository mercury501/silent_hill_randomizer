#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] 

use eframe::egui;
use std::process::Command;
use rfd::FileDialog;

fn main() {
    let options = eframe::NativeOptions::default();
    
    let sh3_path = String::from("D:/Games/Silent Hill 3/sh3.exe");

    
    
    eframe::run_native(
        "Silent Hill Rustomizer",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );

    

}

struct MyApp {
    name: String,
    age: u32,
    sh3_path: String,
	sh3_exe_name: String,
}


impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
            sh3_path: "D:/Games/Silent Hill 3/sh3.exe".to_owned(),
			sh3_exe_name : "sh3.exe".to_owned(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Silent Hill Rustomizer");
           
            ui.horizontal(|ui| {
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name);
            });
           
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
           
            if ui.button("Click each year").clicked() {
                self.age += 1;
            }

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
           
            ui.label(format!("Hello '{}', age {}", self.name, self.age));

            ui.label(format!("The SH3 exe path: {}", self.sh3_path));
        });
    }
}