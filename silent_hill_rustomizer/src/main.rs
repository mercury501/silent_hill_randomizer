#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod data_structs;
mod gui;
mod mem_mgmt;
mod data_storage;

use eframe::egui;
use crate::egui::Vec2;

fn main() {
	let mut options = eframe::NativeOptions::default();
	options.initial_window_size = Option::from(Vec2::new(600 as f32, 680 as f32));

	let mut my_app: data_structs::MyApp = data_structs::MyApp::default();
	my_app.init();
	eframe::run_native(
		"Silent Hill Rustomizer",
		options,
		Box::new(|ctx: &eframe::CreationContext| {
			let mut style = egui::Style::default();
			style.visuals.dark_mode = true;
			ctx.egui_ctx.set_style(style);

			Box::new(my_app)
		}),
	);
}

impl eframe::App for data_structs::MyApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		let main_frame = egui::containers::Frame {
			rounding: egui::Rounding::none(),
			..egui::containers::Frame::window(&egui::Style::default())
		};

		egui::CentralPanel::default()
			.frame(main_frame)
			.show(ctx, |ui| {
				self.main_ui(ui, _frame);
			});

		egui::TopBottomPanel::bottom("bottom")
			.frame(main_frame)
			.show(ctx, |ui| {
				self.main_bottom_ui(ui, main_frame);
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
		egui::Color32::from_rgba_unmultiplied(12, 12, 12, 180).into()

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
