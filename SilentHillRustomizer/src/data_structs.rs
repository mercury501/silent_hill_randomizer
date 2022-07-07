use std::collections::HashMap;


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
//println!("{:#04x}", 10);
impl std::fmt::Display for SH3Mob{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {        
        write!(f, "[TypeID: {:#04x}, OptionID: {:#04x}]", 
        self.type_id,
		self.option_id
    )
    }
}

impl SH3Mob{
	fn clone(&self) -> SH3Mob{
		let ret = SH3Mob{type_id: self.type_id, option_id: self.option_id};
		return ret;
	}
}

pub struct MyApp {
    pub name: String,
    pub age: u32,
    pub sh3_path: String,
	pub sh3_exe_name: String,
	sh3_prob_map: Vec<SH3Mob>,
	pub sliders: HashMap<String, SH3MobData>,

}


impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
            sh3_path: "D:/Games/Silent Hill 3/sh3.exe".to_owned(),
			sh3_exe_name: "sh3.exe".to_owned(),
			sh3_prob_map: Vec::new(),
			sliders: HashMap::new(),
        }
    }
}

impl MyApp{

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

		self.sliders.insert("nurse".to_string(), nurse);
		self.sliders.insert("pendulum".to_string(), pendulum);
		self.sliders.insert("closer".to_string(), closer);
		self.sliders.insert("numb_body".to_string(), numb_body);
		self.sliders.insert("brown_slurper".to_string(), brown_slurper);
		self.sliders.insert("insane_cancer".to_string(), insane_cancer);
		self.sliders.insert("dog".to_string(), dog);
		self.sliders.insert("white_slurper".to_string(), white_slurper);
		self.sliders.insert("scraper".to_string(), scraper);
		self.sliders.insert("missionary".to_string(), missionary);
		self.sliders.insert("leonard".to_string(), leonard);
		self.sliders.insert("alessa".to_string(), alessa);
		self.sliders.insert("god".to_string(), god);

		self.set_probability();

	}

	fn get_total_probabilities(&self) -> f32 {

		let mut probs: f32 = 0.0;
		for key in self.sliders.keys(){
			probs = probs + self.sliders.get(key).unwrap().main as f32;
		}
	
		return probs;
	}
	
	pub fn set_probability(&mut self){
		
		for key in self.sliders.keys(){

		
			
			let total_probabilities: f32 = self.get_total_probabilities();

			let mut current_mob_probability: f32 = self.sliders.get(key).unwrap().main as f32 / total_probabilities;
			current_mob_probability = f32::trunc(current_mob_probability  * 100.0) / 100.0; //to limit to 2 decimal places
			current_mob_probability *= 100.0; //to have a percentage

			let current_mob_total_options_probability: i32 = self.sliders.get(key).unwrap().option_one + self.sliders.get(key).unwrap().option_two + self.sliders.get(key).unwrap().option_three + self.sliders.get(key).unwrap().option_four;

			let option_one_normalized:f32 = self.sliders.get(key).unwrap().option_one as f32 / current_mob_total_options_probability as f32;
			let option_two_normalized:f32 = self.sliders.get(key).unwrap().option_two as f32 / current_mob_total_options_probability as f32;
			let option_three_normalized:f32 = self.sliders.get(key).unwrap().option_three as f32 / current_mob_total_options_probability as f32;
			let option_four_normalized:f32 = self.sliders.get(key).unwrap().option_four as f32 / current_mob_total_options_probability as f32;


			for n in 0..current_mob_probability as u32{

				for i in 0..option_one_normalized as i32{
					self.sh3_prob_map.push(SH3Mob { type_id: self.sliders.get(key).unwrap().type_id, option_id: self.sliders.get(key).unwrap().option_one_id });
				}
				for i in 0..option_two_normalized as i32{
					self.sh3_prob_map.push(SH3Mob { type_id: self.sliders.get(key).unwrap().type_id, option_id: self.sliders.get(key).unwrap().option_two_id });
				}
				for i in 0..option_three_normalized as i32{
					self.sh3_prob_map.push(SH3Mob { type_id: self.sliders.get(key).unwrap().type_id, option_id: self.sliders.get(key).unwrap().option_three_id });
				}
				for i in 0..option_four_normalized as i32{
					self.sh3_prob_map.push(SH3Mob { type_id: self.sliders.get(key).unwrap().type_id, option_id: self.sliders.get(key).unwrap().option_four_id });
				}

			}
		
		}
		for l in 0..self.sh3_prob_map.len(){
			println!("{}", self.sh3_prob_map[l]);
		}
		
		
	}
}