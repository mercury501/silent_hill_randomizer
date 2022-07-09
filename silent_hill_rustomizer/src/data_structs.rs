pub struct SH3MobData {
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

impl Default for SH3MobData {
    fn default() -> Self {
        Self {
            main: 0,
            option_one: 0,
            option_two: 0,
            option_three: 0,
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

pub struct SH3Mob {
    pub type_id: u32,
    pub option_id: u32,
}

impl Default for SH3Mob {
    fn default() -> Self {
        Self {
            type_id: 0,
            option_id: 0,
        }
    }
}

impl std::fmt::Display for SH3Mob {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "[TypeID: {:#04x}, OptionID: {:#04x}]",
            self.type_id, self.option_id
        )
    }
}

pub struct SH3MobPercStrings{
	pub main_str: [String;13],
	pub option_one_str: [String; 13],
	pub option_two_str: [String; 13],
	pub option_three_str: [String; 13],
	pub option_four_str: [String; 13],
}

impl Default for SH3MobPercStrings{
	fn default() -> Self {
		Self { 
			main_str: Default::default(),
			option_one_str: Default::default(),
			option_two_str: Default::default(),
			option_three_str: Default::default(),
			option_four_str: Default::default(),
		 }
	}
}

#[derive(Debug, std::cmp::PartialEq)]
pub enum Tabs {
    SH3Probabilities,
    SH3InfoItems,
	SH2Probabilities,
	SH2InfoItems,
}

pub struct MyApp {
    pub selected_tab: Tabs,
    pub sh3_path: String,
    pub sh3_exe_name: String,
    pub sh3_prob_map: Vec<SH3Mob>,
    pub sh3_sliders: Vec<SH3MobData>,
    pub high_score: i32,
    pub sh3_process_id: u32,
    pub bonus_points: u32,
    pub health_drinks: u8,
	pub sh3_perc_strings: SH3MobPercStrings,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            selected_tab: Tabs::SH3Probabilities,
            sh3_path: "D:/Games/Silent Hill 3/sh3.exe".to_owned(),
            sh3_exe_name: "sh3.exe".to_owned(),
            sh3_prob_map: Vec::new(),
            sh3_sliders: Vec::new(),
            high_score: 0,
            sh3_process_id: 0,
            bonus_points: 0,
            health_drinks: 0,
			sh3_perc_strings: SH3MobPercStrings::default()
        }
    }
}

impl MyApp {
    pub fn set_probability(&mut self) {
        for item in &self.sh3_sliders {
            let total_probabilities: f32 = get_total_probabilities(&self.sh3_sliders);

            let mut current_mob_probability: f32 = item.main as f32 / total_probabilities;
            current_mob_probability = f32::trunc(current_mob_probability * 100.0) / 100.0; //to limit to 2 decimal places
            current_mob_probability *= 100.0; //to have a percentage

            let current_mob_total_options_probability: i32 =
                item.option_one + item.option_two + item.option_three + item.option_four;

            let option_one_normalized: f32 =
                item.option_one   as f32 / current_mob_total_options_probability as f32;
            let option_two_normalized: f32 =
                item.option_two   as f32 / current_mob_total_options_probability as f32;
            let option_three_normalized: f32 =
                item.option_three as f32 / current_mob_total_options_probability as f32;
            let option_four_normalized: f32 =
                item.option_four  as f32 / current_mob_total_options_probability as f32;
			#[allow(unused)]
            for n in 0..current_mob_probability as u32 {
                for i in 0..option_one_normalized as i32 {
                    self.sh3_prob_map.push(SH3Mob {
                        type_id: item.type_id,
                        option_id: item.option_one_id,
                    });
                }
                for i in 0..option_two_normalized as i32 {
                    self.sh3_prob_map.push(SH3Mob {
                        type_id: item.type_id,
                        option_id: item.option_two_id,
                    });
                }
                for i in 0..option_three_normalized as i32 {
                    self.sh3_prob_map.push(SH3Mob {
                        type_id: item.type_id,
                        option_id: item.option_three_id,
                    });
                }
                for i in 0..option_four_normalized as i32 {
                    self.sh3_prob_map.push(SH3Mob {
                        type_id: item.type_id,
                        option_id: item.option_four_id,
                    });
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

    for item in vcmd {
        probs = probs + item.main as f32;
    }

    probs
}