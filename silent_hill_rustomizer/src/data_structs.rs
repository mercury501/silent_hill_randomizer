use std::{collections::HashMap};

use vmemory::ProcessMemory;

pub struct SH3MobData{
    pub main: i32,
    pub option_one: i32,
    pub option_two: i32,
    pub option_three: i32,
    pub option_four: i32,

    pub main_name: String,
    pub option_one_name: String, 
    pub option_two_name: String,
    pub option_three_name: String,
    pub option_four_name: String,

    pub main_perc: f32,
    pub option_one_perc: f32,
    pub option_two_perc: f32,
    pub option_three_perc: f32,
    pub option_four_perc: f32,

	pub main_perc_string: String,
    pub option_one_perc_string: String,
    pub option_two_perc_string: String,
    pub option_three_perc_string: String,
    pub option_four_perc_string: String,

	pub type_id: u32,
	pub option_one_id: u32,
	pub option_two_id: u32,
	pub option_three_id: u32,
	pub option_four_id: u32,
	
}

impl SH3MobData {
    pub fn main_mut(&mut self) -> &mut i32 {
        &mut self.main
    }
}
/*
impl std::fmt::Display for SH3MobData{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {        
        write!(f, "[Name: {}, opts: {}, {}, {}, {}; Sliders: {}, {}, {}, {}, {}; Probs: {}, {}, {}, {}, {}]", 
        self.main_name,
        self.option_one_name,
        self.option_two_name,
        self.option_three_name,
        self.option_four_name,
        self.main,
        self.option_one,
        self.option_two,
        self.option_three,
        self.option_four,
        self.main_perc,
        self.option_one_perc,
        self.option_two_perc,
        self.option_three_perc,
        self.option_four_perc
    )
    }
}*/

impl Default for SH3MobData {
    fn default() -> Self {
        Self {
            main: 0,
            option_one: 0,
            option_two: 0,
            option_three:0,
            option_four: 0,

            main_name: "".to_owned(),
            option_one_name: "".to_owned(),
            option_two_name: "".to_owned(),
            option_three_name: "".to_owned(),
            option_four_name: "".to_owned(),

			main_perc: 1.0,
			option_one_perc: 1.0,
			option_two_perc: 0.0,
			option_three_perc: 0.0,
			option_four_perc: 0.0,

			main_perc_string: "".to_owned(),
			option_one_perc_string: "".to_owned(),
			option_two_perc_string: "".to_owned(),
			option_three_perc_string: "".to_owned(),
			option_four_perc_string: "".to_owned(),

			type_id: 0,
			option_one_id: 0,
			option_two_id: 0,
			option_three_id: 0,
			option_four_id: 0,
        }
    }
}

pub struct SH3Mob{
	pub type_id: u32,
	pub option_id: u32,
}

impl Default for SH3Mob{
	fn default() -> Self {
        Self {
            type_id: 0,
			option_id: 0,
        }
    }
}

impl std::fmt::Display for SH3Mob{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {        
        write!(f, "[TypeID: {:#04x}, OptionID: {:#04x}]", 
        self.type_id,
		self.option_id
    )
    }
}

pub struct MyApp {
    pub sh3_path: String,
	pub sh3_exe_name: String,
	sh3_prob_map: Vec<SH3Mob>,
	pub sliders: Vec<SH3MobData>,
	pub high_score: i32,
	pub sh3_process_id: u32,
	pub bonus_points: u32,

}


impl Default for MyApp {
    fn default() -> Self {
        Self {
            sh3_path: "D:/Games/Silent Hill 3/sh3.exe".to_owned(),
			sh3_exe_name: "sh3.exe".to_owned(),
			sh3_prob_map: Vec::new(),
			sliders: Vec::new(),
			high_score: 0,
			sh3_process_id: 0,
			bonus_points: 0,
        }
    }
}

impl MyApp {
	pub fn init(&mut self){
		let mut nurse = SH3MobData {
			main: 100,
			option_one: 100,
			option_two: 0,
			option_three: 0,
			option_four: 0,
			main_name: "Nurse".to_owned(),
			option_one_name: "With Pipe".to_owned(),
			option_two_name: "With Gun".to_owned(),
			option_three_name: "Jerking With Pipe".to_owned(),
			option_four_name: "Jerking With Gun".to_owned(),
			main_perc: 0.0,
			option_one_perc: 0.0,
			option_two_perc: 0.0,
			option_three_perc: 0.0,
			option_four_perc: 0.0,
			main_perc_string: "".to_owned(),
			option_one_perc_string: "".to_owned(),
			option_two_perc_string: "".to_owned(),
			option_three_perc_string: "".to_owned(),
			option_four_perc_string: "".to_owned(),
			type_id: 0x203,
			option_one_id: 0xB,
			option_two_id: 0xC,
			option_three_id: 0xD,
			option_four_id: 0xE,

		};
		let mut pendulum = SH3MobData {
			main: 100,
			option_one: 100,
			option_two: 0,
			option_three: 0,
			option_four: 0,
			main_name: "Pendulum".to_owned(),
			option_one_name: "Big Walking".to_owned(),
			option_two_name: "Big Flying".to_owned(),
			option_three_name: "Small Walking".to_owned(),
			option_four_name: "Small Flying".to_owned(),
			main_perc: 0.0,
			option_one_perc: 0.0,
			option_two_perc: 0.0,
			option_three_perc: 0.0,
			option_four_perc: 0.0,
			main_perc_string: "".to_owned(),
			option_one_perc_string: "".to_owned(),
			option_two_perc_string: "".to_owned(),
			option_three_perc_string: "".to_owned(),
			option_four_perc_string: "".to_owned(),
			type_id: 0x205,
			option_one_id: 0xF,
			option_two_id: 0x10,
			option_three_id: 0x11,
			option_four_id: 0x12,
		};
		let mut closer = SH3MobData {
			main: 100,
			option_one: 100,
			option_two: 0,
			option_three: 0,
			option_four: 0,
			main_name: "Closer".to_owned(),
			option_one_name: "Normal".to_owned(),
			option_two_name: "Late Game".to_owned(),
			option_three_name: "Dead".to_owned(),
			option_four_name: "Rising Up".to_owned(),
			main_perc: 0.0,
			option_one_perc: 0.0,
			option_two_perc: 0.0,
			option_three_perc: 0.0,
			option_four_perc: 0.0,
			main_perc_string: "".to_owned(),
			option_one_perc_string: "".to_owned(),
			option_two_perc_string: "".to_owned(),
			option_three_perc_string: "".to_owned(),
			option_four_perc_string: "".to_owned(),
			type_id: 0x202,
			option_one_id: 0x1,
			option_two_id: 0x2,
			option_three_id: 0x3,
			option_four_id: 0x4,
		};
		//3 option 
		let mut numb_body = SH3MobData {
			main: 100,
			option_one: 100,
			option_two: 0,
			option_three: 0,
			option_four: 0,
			main_name: "Numb Body".to_owned(),
			option_one_name: "Small".to_owned(),
			option_two_name: "Medium".to_owned(),
			option_three_name: "Large".to_owned(),
			option_four_name: "".to_owned(),
			main_perc: 0.0,
			option_one_perc: 0.0,
			option_two_perc: 0.0,
			option_three_perc: 0.0,
			option_four_perc: 0.0,
			main_perc_string: "".to_owned(),
			option_one_perc_string: "".to_owned(),
			option_two_perc_string: "".to_owned(),
			option_three_perc_string: "".to_owned(),
			option_four_perc_string: "".to_owned(),
			type_id: 0x201,
			option_one_id: 0x5,
			option_two_id: 0x6,
			option_three_id: 0x7,
			option_four_id: 0,
		};
		let mut brown_slurper = SH3MobData {
			main: 100,
			option_one: 100,
			option_two: 0,
			option_three: 0,
			option_four: 0,
			main_name: "Brown Slurper".to_owned(),
			option_one_name: "Normal".to_owned(),
			option_two_name: "Dead".to_owned(),
			option_three_name: "Faking Death".to_owned(),
			option_four_name: "".to_owned(),
			main_perc: 0.0,
			option_one_perc: 0.0,
			option_two_perc: 0.0,
			option_three_perc: 0.0,
			option_four_perc: 0.0,
			main_perc_string: "".to_owned(),
			option_one_perc_string: "".to_owned(),
			option_two_perc_string: "".to_owned(),
			option_three_perc_string: "".to_owned(),
			option_four_perc_string: "".to_owned(),
			type_id: 0x20A,
			option_one_id: 0x16,
			option_two_id: 0x17,
			option_three_id: 0x18,
			option_four_id: 0,
		};
		//2 options
		let mut insane_cancer = SH3MobData {
			main: 100,
			option_one: 100,
			option_two: 0,
			option_three: 0,
			option_four: 0,
			main_name: "Insane Cancer".to_owned(),
			option_one_name: "Sitting".to_owned(),
			option_two_name: "Sleeping".to_owned(),
			option_three_name: "".to_owned(),
			option_four_name: "".to_owned(),
			main_perc: 0.0,
			option_one_perc: 0.0,
			option_two_perc: 0.0,
			option_three_perc: 0.0,
			option_four_perc: 0.0,
			main_perc_string: "".to_owned(),
			option_one_perc_string: "".to_owned(),
			option_two_perc_string: "".to_owned(),
			option_three_perc_string: "".to_owned(),
			option_four_perc_string: "".to_owned(),
			type_id: 0x204,
			option_one_id: 0x9,
			option_two_id: 0xA,
			option_three_id: 0,
			option_four_id: 0,
		};
		//only main 
			let mut dog = SH3MobData {
			main: 100,
			option_one: 100,
			option_two: 0,
			option_three: 0,
			option_four: 0,
			main_name: "Dog".to_owned(),
			option_one_name: "".to_owned(),
			option_two_name: "".to_owned(),
			option_three_name: "".to_owned(),
			option_four_name: "".to_owned(),
			main_perc: 0.0,
			option_one_perc: 0.0,
			option_two_perc: 0.0,
			option_three_perc: 0.0,
			option_four_perc: 0.0,
			main_perc_string: "".to_owned(),
			option_one_perc_string: "".to_owned(),
			option_two_perc_string: "".to_owned(),
			option_three_perc_string: "".to_owned(),
			option_four_perc_string: "".to_owned(),
			type_id: 0x200,
			option_one_id: 0x8,
			option_two_id: 0,
			option_three_id: 0,
			option_four_id: 0,
		};
		let mut white_slurper = SH3MobData {
			main: 100,
			option_one: 100,
			option_two: 0,
			option_three: 0,
			option_four: 0,
			main_name: "White Slurper".to_owned(),
			option_one_name: "".to_owned(),
			option_two_name: "".to_owned(),
			option_three_name: "".to_owned(),
			option_four_name: "".to_owned(),
			main_perc: 0.0,
			option_one_perc: 0.0,
			option_two_perc: 0.0,
			option_three_perc: 0.0,
			option_four_perc: 0.0,
			main_perc_string: "".to_owned(),
			option_one_perc_string: "".to_owned(),
			option_two_perc_string: "".to_owned(),
			option_three_perc_string: "".to_owned(),
			option_four_perc_string: "".to_owned(),
			type_id: 0x20B,
			option_one_id: 0,
			option_two_id: 0,
			option_three_id: 0,
			option_four_id: 0,
		};
		let mut scraper = SH3MobData {
			main: 100,
			option_one: 100,
			option_two: 0,
			option_three: 0,
			option_four: 0,
			main_name: "Scraper".to_owned(),
			option_one_name: "".to_owned(),
			option_two_name: "".to_owned(),
			option_three_name: "".to_owned(),
			option_four_name: "".to_owned(),
			main_perc: 0.0,
			option_one_perc: 0.0,
			option_two_perc: 0.0,
			option_three_perc: 0.0,
			option_four_perc: 0.0,
			main_perc_string: "".to_owned(),
			option_one_perc_string: "".to_owned(),
			option_two_perc_string: "".to_owned(),
			option_three_perc_string: "".to_owned(),
			option_four_perc_string: "".to_owned(),
			type_id: 0x206,
			option_one_id: 0,
			option_two_id: 0,
			option_three_id: 0,
			option_four_id: 0,
		};
		let mut missionary = SH3MobData {
			main: 100,
			option_one: 100,
			option_two: 0,
			option_three: 0,
			option_four: 0,
			main_name: "Missionary".to_owned(),
			option_one_name: "".to_owned(),
			option_two_name: "".to_owned(),
			option_three_name: "".to_owned(),
			option_four_name: "".to_owned(),
			main_perc: 0.0,
			option_one_perc: 0.0,
			option_two_perc: 0.0,
			option_three_perc: 0.0,
			option_four_perc: 0.0,
			main_perc_string: "".to_owned(),
			option_one_perc_string: "".to_owned(),
			option_two_perc_string: "".to_owned(),
			option_three_perc_string: "".to_owned(),
			option_four_perc_string: "".to_owned(),
			type_id: 0x211,
			option_one_id: 0,
			option_two_id: 0,
			option_three_id: 0,
			option_four_id: 0,
		};
		let mut leonard = SH3MobData {
			main: 100,
			option_one: 100,
			option_two: 0,
			option_three: 0,
			option_four: 0,
			main_name: "Leonard".to_owned(),
			option_one_name: "".to_owned(),
			option_two_name: "".to_owned(),
			option_three_name: "".to_owned(),
			option_four_name: "".to_owned(),
			main_perc: 0.0,
			option_one_perc: 0.0,
			option_two_perc: 0.0,
			option_three_perc: 0.0,
			option_four_perc: 0.0,
			main_perc_string: "".to_owned(),
			option_one_perc_string: "".to_owned(),
			option_two_perc_string: "".to_owned(),
			option_three_perc_string: "".to_owned(),
			option_four_perc_string: "".to_owned(),
			type_id: 0x213,
			option_one_id: 0,
			option_two_id: 0,
			option_three_id: 0,
			option_four_id: 0,
		};
		let mut alessa = SH3MobData {
			main: 100,
			option_one: 100,
			option_two: 0,
			option_three: 0,
			option_four: 0,
			main_name: "Alessa".to_owned(),
			option_one_name: "".to_owned(),
			option_two_name: "".to_owned(),
			option_three_name: "".to_owned(),
			option_four_name: "".to_owned(),
			main_perc: 0.0,
			option_one_perc: 0.0,
			option_two_perc: 0.0,
			option_three_perc: 0.0,
			option_four_perc: 0.0,
			main_perc_string: "".to_owned(),
			option_one_perc_string: "".to_owned(),
			option_two_perc_string: "".to_owned(),
			option_three_perc_string: "".to_owned(),
			option_four_perc_string: "".to_owned(),
			type_id: 0x215,
			option_one_id: 0,
			option_two_id: 0,
			option_three_id: 0,
			option_four_id: 0,
		};
		let mut god = SH3MobData {
			main: 100,
			option_one: 100,
			option_two: 0,
			option_three: 0,
			option_four: 0,
			main_name: "God".to_owned(),
			option_one_name: "".to_owned(),
			option_two_name: "".to_owned(),
			option_three_name: "".to_owned(),
			option_four_name: "".to_owned(),
			main_perc: 0.0,
			option_one_perc: 0.0,
			option_two_perc: 0.0,
			option_three_perc: 0.0,
			option_four_perc: 0.0,
			main_perc_string: "".to_owned(),
			option_one_perc_string: "".to_owned(),
			option_two_perc_string: "".to_owned(),
			option_three_perc_string: "".to_owned(),
			option_four_perc_string: "".to_owned(),
			type_id: 0x214,
			option_one_id: 0,
			option_two_id: 0,
			option_three_id: 0,
			option_four_id: 0,

		};

		self.sliders.push(nurse);
		self.sliders.push(pendulum);
		self.sliders.push(closer);
		self.sliders.push(numb_body);
		self.sliders.push(brown_slurper);
		self.sliders.push(insane_cancer);
		self.sliders.push(dog);
		self.sliders.push(white_slurper);
		self.sliders.push(scraper);
		self.sliders.push(missionary);
		self.sliders.push(leonard);
		self.sliders.push(alessa);
		self.sliders.push(god);

		self.set_probability();

	}

	pub fn set_probability(&mut self){
		
		for item in &self.sliders {

		
			
			let total_probabilities: f32 = get_total_probabilities(&self.sliders);

			let mut current_mob_probability: f32 = item.main as f32 / total_probabilities;
			current_mob_probability = f32::trunc(current_mob_probability  * 100.0) / 100.0; //to limit to 2 decimal places
			current_mob_probability *= 100.0; //to have a percentage

			let current_mob_total_options_probability: i32 = item.option_one + item.option_two + item.option_three + item.option_four;

			let option_one_normalized:f32 = item.option_one as f32 / current_mob_total_options_probability as f32;
			let option_two_normalized:f32 = item.option_two as f32 / current_mob_total_options_probability as f32;
			let option_three_normalized:f32 = item.option_three as f32 / current_mob_total_options_probability as f32;
			let option_four_normalized:f32 = item.option_four as f32 / current_mob_total_options_probability as f32;


			for n in 0..current_mob_probability as u32{

				for i in 0..option_one_normalized as i32{
					self.sh3_prob_map.push(SH3Mob { type_id: item.type_id, option_id: item.option_one_id });
				}
				for i in 0..option_two_normalized as i32{
					self.sh3_prob_map.push(SH3Mob { type_id: item.type_id, option_id: item.option_two_id });
				}
				for i in 0..option_three_normalized as i32{
					self.sh3_prob_map.push(SH3Mob { type_id: item.type_id, option_id: item.option_three_id });
				}
				for i in 0..option_four_normalized as i32{
					self.sh3_prob_map.push(SH3Mob { type_id: item.type_id, option_id: item.option_four_id });
				}

			}
		
		}
		/*
		for l in 0..self.sh3_prob_map.len(){
			println!("{}", self.sh3_prob_map[l]);
		}*/
		
		
	}

}

fn get_total_probabilities(vcmd: &Vec<SH3MobData>) -> f32 {

	let mut probs: f32 = 0.0;
	for item in vcmd{
		probs = probs + item.main as f32;
	}

	return probs;
}