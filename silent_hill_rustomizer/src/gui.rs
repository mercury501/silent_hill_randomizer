use crate::data_structs;
use crate::data_structs::get_time_string;
use crate::egui;
use crate::mem_mgmt;
use chrono::Duration;
use rfd::FileDialog;
use std::process::Command;

impl data_structs::MyApp {
    pub fn main_ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        ui.horizontal(|ui| {
            ui.selectable_value(
                &mut self.selected_tab,
                data_structs::Tabs::SH3Probabilities,
                "SH3 Probabilities",
            );
            ui.selectable_value(
                &mut self.selected_tab,
                data_structs::Tabs::SH3InfoItems,
                "SH3 Info - Items",
            );

            ui.selectable_value(
                &mut self.selected_tab,
                data_structs::Tabs::SH2Probabilities,
                "SH2 Probabilities",
            );
            ui.selectable_value(
                &mut self.selected_tab,
                data_structs::Tabs::SH2InfoItems,
                "SH2 Info - Items",
            );
        });

        ui.separator();

        match self.selected_tab {
            data_structs::Tabs::SH3Probabilities => self.probs_ui(ui, frame),
            data_structs::Tabs::SH3InfoItems => self.info_ui(ui),
            data_structs::Tabs::SH2Probabilities => self.probs_ui(ui, frame), //TODO change for sh2
            data_structs::Tabs::SH2InfoItems => self.info_ui(ui),
        }
    }

    fn probs_ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let mut index = 0;
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    //nurse
                    ui.label(self.sh3_sliders[index].main_name.to_string());
                    ui.horizontal(|ui| {
                        ui.add(egui::Slider::new(
                            self.sh3_sliders[index].main_mut(),
                            0..=100,
                        ));
                        ui.label(&self.sh3_sliders[index].main_perc_string);
                    });

                    ui.label(
                        self.sh3_sliders[index].main_name.to_string()
                            + " - "
                            + &self.sh3_sliders[index].option_one_name.to_string(),
                    );
                    ui.horizontal(|ui| {
                        ui.add(egui::Slider::new(
                            &mut self.sh3_sliders[index].option_one,
                            0..=100,
                        ));
                        ui.label(&self.sh3_sliders[index].option_one_perc_string);
                    });
                    ui.label(
                        self.sh3_sliders[index].main_name.to_string()
                            + " - "
                            + &self.sh3_sliders[index].option_two_name.to_string(),
                    );
                    ui.horizontal(|ui| {
                        ui.add(egui::Slider::new(
                            &mut self.sh3_sliders[index].option_two,
                            0..=100,
                        ));
                        ui.label(&self.sh3_sliders[index].option_two_perc_string);
                    });

                    ui.label(
                        self.sh3_sliders[index].main_name.to_string()
                            + " - "
                            + &self.sh3_sliders[index].option_three_name.to_string(),
                    );
                    ui.horizontal(|ui| {
                        ui.add(egui::Slider::new(
                            &mut self.sh3_sliders[index].option_three,
                            0..=100,
                        ));
                        ui.label(&self.sh3_sliders[index].option_three_perc_string);
                    });

                    ui.label(
                        self.sh3_sliders[index].main_name.to_string()
                            + " - "
                            + &self.sh3_sliders[index].option_four_name.to_string(),
                    );
                    ui.horizontal(|ui| {
                        ui.add(egui::Slider::new(
                            &mut self.sh3_sliders[index].option_four,
                            0..=100,
                        ));
                        ui.label(&self.sh3_sliders[index].option_four_perc_string);
                    });

                    index += 1;

                    //pendulum
                    ui.label(self.sh3_sliders[index].main_name.to_string());
                    ui.horizontal(|ui| {
                        ui.add(egui::Slider::new(
                            self.sh3_sliders[index].main_mut(),
                            0..=100,
                        ));
                        ui.label(&self.sh3_sliders[index].main_perc_string);
                    });
                    ui.label(
                        self.sh3_sliders[index].main_name.to_string()
                            + " - "
                            + &self.sh3_sliders[index].option_one_name.to_string(),
                    );
                    ui.horizontal(|ui| {
                        ui.add(egui::Slider::new(
                            &mut self.sh3_sliders[index].option_one,
                            0..=100,
                        ));
                        ui.label(&self.sh3_sliders[index].option_one_perc_string);
                    });

                    ui.label(
                        self.sh3_sliders[index].main_name.to_string()
                            + " - "
                            + &self.sh3_sliders[index].option_two_name.to_string(),
                    );
                    ui.horizontal(|ui| {
                        ui.add(egui::Slider::new(
                            &mut self.sh3_sliders[index].option_two,
                            0..=100,
                        ));
                        ui.label(&self.sh3_sliders[index].option_two_perc_string);
                    });

                    ui.label(
                        self.sh3_sliders[index].main_name.to_string()
                            + " - "
                            + &self.sh3_sliders[index].option_three_name.to_string(),
                    );
                    ui.horizontal(|ui| {
                        ui.add(egui::Slider::new(
                            &mut self.sh3_sliders[index].option_three,
                            0..=100,
                        ));
                        ui.label(&self.sh3_sliders[index].option_three_perc_string);
                    });

                    ui.label(
                        self.sh3_sliders[index].main_name.to_string()
                            + " - "
                            + &self.sh3_sliders[index].option_four_name.to_string(),
                    );
                    ui.horizontal(|ui| {
                        ui.add(egui::Slider::new(
                            &mut self.sh3_sliders[index].option_four,
                            0..=100,
                        ));
                        ui.label(&self.sh3_sliders[index].option_four_perc_string);
                    });
                    index += 1;

                    //closer
                    ui.label(self.sh3_sliders[index].main_name.to_string());
                    ui.horizontal(|ui| {
                        ui.add(egui::Slider::new(
                            self.sh3_sliders[index].main_mut(),
                            0..=100,
                        ));
                        ui.label(&self.sh3_sliders[index].main_perc_string);
                    });

                    ui.label(
                        self.sh3_sliders[index].main_name.to_string()
                            + " - "
                            + &self.sh3_sliders[index].option_one_name.to_string(),
                    );
                    ui.horizontal(|ui| {
                        ui.add(egui::Slider::new(
                            &mut self.sh3_sliders[index].option_one,
                            0..=100,
                        ));
                        ui.label(&self.sh3_sliders[index].option_one_perc_string);
                    });

                    ui.label(
                        self.sh3_sliders[index].main_name.to_string()
                            + " - "
                            + &self.sh3_sliders[index].option_two_name.to_string(),
                    );
                    ui.horizontal(|ui| {
                        ui.add(egui::Slider::new(
                            &mut self.sh3_sliders[index].option_two,
                            0..=100,
                        ));
                        ui.label(&self.sh3_sliders[index].option_two_perc_string);
                    });

                    ui.label(
                        self.sh3_sliders[index].main_name.to_string()
                            + " - "
                            + &self.sh3_sliders[index].option_three_name.to_string(),
                    );
                    ui.horizontal(|ui| {
                        ui.add(egui::Slider::new(
                            &mut self.sh3_sliders[index].option_three,
                            0..=100,
                        ));
                        ui.label(&self.sh3_sliders[index].option_three_perc_string);
                    });

                    ui.label(
                        self.sh3_sliders[index].main_name.to_string()
                            + " - "
                            + &self.sh3_sliders[index].option_four_name.to_string(),
                    );
                    ui.horizontal(|ui| {
                        ui.add(egui::Slider::new(
                            &mut self.sh3_sliders[index].option_four,
                            0..=100,
                        ));
                        ui.label(&self.sh3_sliders[index].option_four_perc_string);
                    });
                });

                index += 1;
                ui.vertical(|ui| {
                    //numb_body
                    ui.label(self.sh3_sliders[index].main_name.to_string());
                    ui.horizontal(|ui| {
                        ui.add(egui::Slider::new(
                            self.sh3_sliders[index].main_mut(),
                            0..=100,
                        ));
                        ui.label(&self.sh3_sliders[index].main_perc_string);
                    });

                    ui.label(
                        self.sh3_sliders[index].main_name.to_string()
                            + " - "
                            + &self.sh3_sliders[index].option_one_name.to_string(),
                    );
                    ui.horizontal(|ui| {
                        ui.add(egui::Slider::new(
                            &mut self.sh3_sliders[index].option_one,
                            0..=100,
                        ));
                        ui.label(&self.sh3_sliders[index].option_one_perc_string);
                    });

                    ui.label(
                        self.sh3_sliders[index].main_name.to_string()
                            + " - "
                            + &self.sh3_sliders[index].option_two_name.to_string(),
                    );
                    ui.horizontal(|ui| {
                        ui.add(egui::Slider::new(
                            &mut self.sh3_sliders[index].option_two,
                            0..=100,
                        ));
                        ui.label(&self.sh3_sliders[index].option_two_perc_string);
                    });

                    ui.label(
                        self.sh3_sliders[index].main_name.to_string()
                            + " - "
                            + &self.sh3_sliders[index].option_three_name.to_string(),
                    );
                    ui.horizontal(|ui| {
                        ui.add(egui::Slider::new(
                            &mut self.sh3_sliders[index].option_three,
                            0..=100,
                        ));
                        ui.label(&self.sh3_sliders[index].option_three_perc_string);
                    });

                    index += 1;

                    //brown_slurper
                    ui.label(self.sh3_sliders[index].main_name.to_string());
                    ui.horizontal(|ui| {
                        ui.add(egui::Slider::new(
                            self.sh3_sliders[index].main_mut(),
                            0..=100,
                        ));
                        ui.label(&self.sh3_sliders[index].main_perc_string);
                    });

                    ui.label(
                        self.sh3_sliders[index].main_name.to_string()
                            + " - "
                            + &self.sh3_sliders[index].option_one_name.to_string(),
                    );
                    ui.horizontal(|ui| {
                        ui.add(egui::Slider::new(
                            &mut self.sh3_sliders[index].option_one,
                            0..=100,
                        ));
                        ui.label(&self.sh3_sliders[index].option_one_perc_string);
                    });

                    ui.label(
                        self.sh3_sliders[index].main_name.to_string()
                            + " - "
                            + &self.sh3_sliders[index].option_two_name.to_string(),
                    );
                    ui.horizontal(|ui| {
                        ui.add(egui::Slider::new(
                            &mut self.sh3_sliders[index].option_two,
                            0..=100,
                        ));
                        ui.label(&self.sh3_sliders[index].option_two_perc_string);
                    });

                    ui.label(
                        self.sh3_sliders[index].main_name.to_string()
                            + " - "
                            + &self.sh3_sliders[index].option_three_name.to_string(),
                    );
                    ui.horizontal(|ui| {
                        ui.add(egui::Slider::new(
                            &mut self.sh3_sliders[index].option_three,
                            0..=100,
                        ));
                        ui.label(&self.sh3_sliders[index].option_three_perc_string);
                    });

                    index += 1;

                    //insane_cancer
                    ui.label(self.sh3_sliders[index].main_name.to_string());
                    ui.horizontal(|ui| {
                        ui.add(egui::Slider::new(
                            self.sh3_sliders[index].main_mut(),
                            0..=100,
                        ));
                        ui.label(&self.sh3_sliders[index].main_perc_string);
                    });

                    ui.label(
                        self.sh3_sliders[index].main_name.to_string()
                            + " - "
                            + &self.sh3_sliders[index].option_one_name.to_string(),
                    );
                    ui.horizontal(|ui| {
                        ui.add(egui::Slider::new(
                            &mut self.sh3_sliders[index].option_one,
                            0..=100,
                        ));
                        ui.label(&self.sh3_sliders[index].option_one_perc_string);
                    });

                    ui.label(
                        self.sh3_sliders[index].main_name.to_string()
                            + " - "
                            + &self.sh3_sliders[index].option_two_name.to_string(),
                    );
                    ui.horizontal(|ui| {
                        ui.add(egui::Slider::new(
                            &mut self.sh3_sliders[index].option_two,
                            0..=100,
                        ));
                        ui.label(&self.sh3_sliders[index].option_two_perc_string);
                    });

                    index += 1;
                });
                ui.vertical(|ui| {
                    //one option
                    //dog
                    ui.label(self.sh3_sliders[index].main_name.to_string());
                    ui.horizontal(|ui| {
                        ui.add(egui::Slider::new(
                            self.sh3_sliders[index].main_mut(),
                            0..=100,
                        ));
                        ui.label(&self.sh3_sliders[index].main_perc_string);
                    });

                    index += 1;

                    //white_slurper
                    ui.label(self.sh3_sliders[index].main_name.to_string());
                    ui.horizontal(|ui| {
                        ui.add(egui::Slider::new(
                            self.sh3_sliders[index].main_mut(),
                            0..=100,
                        ));
                        ui.label(&self.sh3_sliders[index].main_perc_string);
                    });

                    index += 1;

                    //scraper
                    ui.label(self.sh3_sliders[index].main_name.to_string());
                    ui.horizontal(|ui| {
                        ui.add(egui::Slider::new(
                            self.sh3_sliders[index].main_mut(),
                            0..=100,
                        ));
                        ui.label(&self.sh3_sliders[index].main_perc_string);
                    });

                    index += 1;

                    //missionary
                    ui.label(self.sh3_sliders[index].main_name.to_string());
                    ui.horizontal(|ui| {
                        ui.add(egui::Slider::new(
                            self.sh3_sliders[index].main_mut(),
                            0..=100,
                        ));
                        ui.label(&self.sh3_sliders[index].main_perc_string);
                    });

                    index += 1;

                    //leonard
                    ui.label(self.sh3_sliders[index].main_name.to_string());
                    ui.horizontal(|ui| {
                        ui.add(egui::Slider::new(
                            self.sh3_sliders[index].main_mut(),
                            0..=100,
                        ));
                        ui.label(&self.sh3_sliders[index].main_perc_string);
                    });

                    index += 1;

                    //alessa
                    ui.label(self.sh3_sliders[index].main_name.to_string());
                    ui.horizontal(|ui| {
                        ui.add(egui::Slider::new(
                            self.sh3_sliders[index].main_mut(),
                            0..=100,
                        ));
                        ui.label(&self.sh3_sliders[index].main_perc_string);
                    });

                    index += 1;

                    //god
                    ui.label(self.sh3_sliders[index].main_name.to_string());
                    ui.horizontal(|ui| {
                        ui.add(egui::Slider::new(
                            self.sh3_sliders[index].main_mut(),
                            0..=100,
                        ));
                        ui.label(&self.sh3_sliders[index].main_perc_string);
                    });
                });
            });
            ui.label(format!("SH3 exe path: {}", self.sh3_path));
        });
    }

    fn info_ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.vertical(|ui| {
                    ui.label("Health Drinks: ");
                    ui.label(self.health_drinks.to_string());
                });
                ui.vertical(|ui| {
                    ui.label("High Score: ");
                    ui.label(self.sh3_high_score.to_string());
                });
                ui.vertical(|ui| {
                    ui.label("Bonus Points: ");
                    ui.label(self.bonus_points.to_string());
                });
                ui.vertical(|ui| {
                    ui.label("In Game Time: ");
                    ui.label(self.sh3_in_game_time.to_string());
                });
                if ui.button("Update Values").clicked() {
                    self.sh3_high_score =
                        mem_mgmt::read_u32(self.sh3_process_id, self.sh3_addresses.high_score)
                            as i32;
                    self.bonus_points =
                        mem_mgmt::read_f32(self.sh3_process_id, self.sh3_addresses.bonus_points)
                            as u32;
                    self.health_drinks =
                        mem_mgmt::read_u8(self.sh3_process_id, self.sh3_addresses.health_drinks);
                    let sh3_igt = Duration::seconds(mem_mgmt::read_f32(
                        self.sh3_process_id,
                        self.sh3_addresses.in_game_time,
                    ) as i64);
                    self.sh3_in_game_time = get_time_string(sh3_igt);
                }
            });
            ui.vertical(|ui| {
                if ui.button("Add 5 health drinks").clicked() {
                    let mut current_drinks =
                        mem_mgmt::read_u8(self.sh3_process_id, self.sh3_addresses.health_drinks);
                    current_drinks += 5;
                    if !(current_drinks >= 255) {
                        mem_mgmt::write_u8(
                            self.sh3_process_id,
                            self.sh3_addresses.health_drinks,
                            current_drinks as u8,
                        );
                    }
                }
            });
        });
    }

    pub fn bottom_ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("Update Probs").clicked() {
                self.set_probability();
            }

            if ui.button("Find SH3").clicked() {
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

                self.sh3_process_id = sh3_process.id();
            }
        });
    }

    pub fn main_bottom_ui(&mut self, ui: &mut egui::Ui, frame: eframe::egui::Frame) {
        match self.selected_tab {
            data_structs::Tabs::SH3Probabilities => self.bottom_probs_ui(ui, frame),
            data_structs::Tabs::SH3InfoItems => self.bottom_info_ui(ui, frame),
            data_structs::Tabs::SH2Probabilities => self.bottom_probs_ui(ui, frame), //TODO change for sh2
            data_structs::Tabs::SH2InfoItems => self.bottom_info_ui(ui, frame),
        }
    }

    pub fn bottom_probs_ui(&mut self, ui: &mut egui::Ui, _frame: eframe::egui::Frame) {
        ui.horizontal(|ui| {
            if ui.button("Update Probs").clicked() {
                self.set_probability();
            }

            if ui.button("Find SH3").clicked() {
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

                self.sh3_process_id = sh3_process.id();
            }
        });
    }

    pub fn bottom_info_ui(&mut self, ui: &mut egui::Ui, frame: eframe::egui::Frame) {
        ui.horizontal(|ui| {
            ui.label(format!("The SH3 exe path: {}", self.sh3_path));
        });
    }
}
