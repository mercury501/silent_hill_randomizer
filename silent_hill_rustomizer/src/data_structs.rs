use crate::mem_mgmt;
use chrono::Duration;
use rand::prelude::SliceRandom;

pub struct SH3Addresses {
    pub high_score: usize,    
    pub health_drinks: usize, 
    pub bonus_points: usize,
    pub in_game_time: usize,  
}

impl Default for SH3Addresses {
    fn default() -> Self {
        Self {
            high_score: 0x070E66F0, 	//i32
            health_drinks: 0x0712CAB2, 	//u8
            bonus_points: 0x0712C59C, 	//f32
            in_game_time: 0x070E66F4,   //f32, in seconds
        }
    }
}

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

pub struct SH3MobPercStrings {
    pub main_str: [String; 13],
    pub option_one_str: [String; 13],
    pub option_two_str: [String; 13],
    pub option_three_str: [String; 13],
    pub option_four_str: [String; 13],
}

impl Default for SH3MobPercStrings {
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
    pub sh3_high_score: i32,
    pub sh3_process_id: u32,
    pub bonus_points: u32,
    pub health_drinks: u8,
    pub sh3_perc_strings: SH3MobPercStrings,
	pub sh3_addresses: SH3Addresses,
	pub sh3_randomizable_type_id: Vec<i32>,
	pub sh3_not_randomizable_gid: Vec<i32>,
    pub sh3_in_game_time: String,

    pub sh2_regions: [i32; 59],
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            selected_tab: Tabs::SH3Probabilities,
            sh3_path: "D:/Games/Silent Hill 3/sh3.exe".to_owned(),
            sh3_exe_name: "sh3.exe".to_owned(),
            sh3_prob_map: Vec::new(),
            sh3_sliders: Vec::new(),
            sh3_high_score: 0,
            sh3_process_id: 0,
            bonus_points: 0,
            health_drinks: 0,
            sh3_perc_strings: SH3MobPercStrings::default(),
			sh3_addresses: SH3Addresses::default(),
			sh3_randomizable_type_id: vec![0x200, 0x201, 0x202, 0x203, 0x204, 0x205, 0x206, 0x20A, 0x20B, 0x211, 0x213, 0x215],
			sh3_not_randomizable_gid: vec![17, 240, 310, 397],
            sh3_in_game_time: Default::default(),

            sh2_regions: [0x0, 0x8F18A0, 0x8EAB50, 0x8EA718, 0x8EA208, 0x8EA208, 0x8E65D0, 0x8E5A58, 0x8E41A8, 0x8E3AC0,
            0x8E30F8, 0x8E2708, 0x8E1F08, 0x8E1698, 0x8E08F0, 0x8DD888, 0x8DD380, 0x8DD238, 0x8DCD48, 0x8DC180,
            0x8DB288, 0x8DA7D0, 0x8D9D68, 0x8D9798, 0x8D9218, 0x8D8CA8, 0x8D8130, 0x8D7870, 0x8D7620, 0x8D7318,
            0x8D6ED8, 0x8D61E8, 0x8D5408, 0x8D4918, 0x8D4140, 0x8D2F00, 0x8D1FA8, 0x8CFCC8, 0x8CF618, 0x8CF310,
            0x8CE9C8, 0x8CD0A0, 0x8CBE78, 0x8CB8F0, 0x8CAEC8, 0x8CA668, 0x8C9ED0, 0x8C9A58, 0x7B3D80, 0x7B4A58,
            0x7B3058, 0x8C9610, 0x8C8C18, 0x8C8AB0, 0x8C8640, 0x8C7218, 0x8C6140, 0x8C5648, 0x8C5130],
        }
    }
}

impl MyApp {
    pub fn set_probability(&mut self) {
		self.sh3_prob_map.clear();
        let total_probabilities: f32 = get_total_probabilities(&self.sh3_sliders) as f32;
        for item in &mut self.sh3_sliders {

            let mut current_mob_probability: f32 = item.main as f32 / total_probabilities;
            current_mob_probability = f32::trunc(current_mob_probability * 100.0) / 100.0; //to limit to 2 decimal places
            current_mob_probability *= 100.0; //to have a percentage

            let current_mob_total_options_probability: i32 =
                item.option_one + item.option_two + item.option_three + item.option_four;
  
            if current_mob_total_options_probability == 0 {
                continue
             }
			
            let option_one_normalized: f32 =
                item.option_one as f32 / current_mob_total_options_probability as f32;
            let option_two_normalized: f32 =
                item.option_two as f32 / current_mob_total_options_probability as f32;
            let option_three_normalized: f32 =
                item.option_three as f32 / current_mob_total_options_probability as f32;
            let option_four_normalized: f32 =
                item.option_four as f32 / current_mob_total_options_probability as f32;
            
			item.main_perc_string = current_mob_probability.to_string();
			item.main_perc_string.push('%');
			item.option_one_perc_string = option_one_normalized.to_string();
			item.option_one_perc_string.push('%');
			item.option_two_perc_string = option_two_normalized.to_string();
			item.option_two_perc_string.push('%');
			item.option_three_perc_string = option_three_normalized.to_string();
			item.option_three_perc_string.push('%');
			item.option_four_perc_string = option_four_normalized.to_string();
            item.option_four_perc_string.push('%');
            /*
			println!("Current mob: {}, percs: {}, {}, {}\n" , item.main_name, current_mob_probability.to_string() ,
				option_one_normalized.to_string() , option_two_normalized.to_string());
			println!("current perc strings {} {} {} {} {}", item.main_perc_string, item.option_one_perc_string, item.option_two_perc_string,
					item.option_three_perc_string, item.option_four_perc_string);
            */
            #[allow(unused)]
                for i in 0..(option_one_normalized * current_mob_probability) as i32 {
                    self.sh3_prob_map.push(SH3Mob {
                        type_id: item.type_id,
                        option_id: item.option_one_id,
                    });
                }
                for i in 0..(option_two_normalized * current_mob_probability) as i32 {
                    self.sh3_prob_map.push(SH3Mob {
                        type_id: item.type_id,
                        option_id: item.option_two_id,
                    });
                }
                for i in 0..(option_three_normalized * current_mob_probability) as i32 {
                    self.sh3_prob_map.push(SH3Mob {
                        type_id: item.type_id,
                        option_id: item.option_three_id,
                    });
                }
                for i in 0..(option_four_normalized * current_mob_probability) as i32 {
                    self.sh3_prob_map.push(SH3Mob {
                        type_id: item.type_id,
                        option_id: item.option_four_id,
                    });
                }
            
        }
        //TODO fixare la mappa
		self.inject_values();
        /*
        for l in 0..self.sh3_prob_map.len(){
            println!("{}", self.sh3_prob_map[l]);
        }*/
    }

	fn can_randomize_gid(&mut self, gid: &i32) -> bool{
		if self.sh3_not_randomizable_gid.contains(&gid){
			return false;
		}
		if gid < &0x101 || gid > &0x119 {
			return true;
		}
		false
	}

	fn inject_values(&mut self) {
		let addr: usize = 0x006cf7d0;
		let mut rng = rand::thread_rng();
		let mut ents_addr: usize;
		let mut ptr_addr: usize;
		let mut type_id: i32;
		let mut gid: i32;
		let mut random_mob: &SH3Mob;

        if self.sh3_process_id == 0{
            return;
        }
		for offset in 0..40 {
				ptr_addr = mem_mgmt::read_u32(self.sh3_process_id, addr + (offset * 4)) as usize;
				if ptr_addr == 0{
					continue
				}

				ents_addr = mem_mgmt::read_u32(self.sh3_process_id, ptr_addr + 16) as usize;
				if ents_addr == 0{
					continue
				}

				loop {//TODO test
					type_id = mem_mgmt::read_u16(self.sh3_process_id, ents_addr) as i32;
					gid = mem_mgmt::read_u16(self.sh3_process_id, ents_addr + 2) as i32;

					if type_id == 0{
						break
					}

					if (self.sh3_randomizable_type_id.contains(&type_id) && self.can_randomize_gid(&gid)){
						random_mob = self.sh3_prob_map.choose(&mut rng).unwrap();
                        
                        #[cfg(debug_assertions)]
                        println!("Writing id {:#04x}, opt {:#04x}", random_mob.type_id, random_mob.option_id);

						mem_mgmt::write_u16(self.sh3_process_id, ents_addr, random_mob.type_id as u16);
						mem_mgmt::write_u16(self.sh3_process_id, ents_addr + 0x16, random_mob.option_id as u16);
					}

					ents_addr += 0x18;
				}

		}
	}
}

fn get_total_probabilities(vcmd: &Vec<SH3MobData>) -> u32 {
    let mut probs: u32 = 0;

    for item in vcmd {
        probs = probs + item.main as u32;
    }

    probs
}

pub fn get_time_string(dur: Duration) -> String {
    let mut str: String = Default::default();
    let hours: i64 = dur.num_hours();
    let minutes: i64 = dur.num_minutes() - (hours * 60);
    let seconds: i64 = dur.num_seconds() - (((hours * 60) + minutes) * 60);



    str.push_str(&hours.to_string());
    str.push(':');
    str.push_str(&minutes.to_string());
    str.push(':');
    str.push_str(&seconds.to_string());

    str
}