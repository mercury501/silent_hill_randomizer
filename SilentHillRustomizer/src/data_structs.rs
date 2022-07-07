use std::fmt;

pub struct SH3Sliders{
	pub nurse: SH3MobData,
    pub pendulum: SH3MobData,
    pub closer: SH3MobData,
    
    //3 options
    pub numb_body: SH3MobData,
    pub brown_slurper: SH3MobData,
    pub insane_cancer: SH3MobData,

    //only main 
    pub double_head: SH3MobData,
    pub white_slurper: SH3MobData,
    pub scraper: SH3MobData,
    pub missionary: SH3MobData,
    pub leonard: SH3MobData,
    pub alessa: SH3MobData,
    pub god: SH3MobData,

}

impl Default for SH3Sliders{
    fn default() -> Self {
        Self {
            nurse: SH3MobData {
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
            },
            pendulum: SH3MobData {
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
            },
            closer: SH3MobData {
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
            },
            //3 option 
            numb_body: SH3MobData {
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
            },
            brown_slurper: SH3MobData {
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
            },
            //2 options
            insane_cancer: SH3MobData {
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
            },
            //only main 
                double_head: SH3MobData {
                main: 100,
                option_one: 100,
                option_two: 0,
                option_three: 0,
                option_four: 0,
                main_name: "Double Head".to_owned(),
                option_one_name: "".to_owned(),
                option_two_name: "".to_owned(),
                option_three_name: "".to_owned(),
                option_four_name: "".to_owned(),
            },
            white_slurper: SH3MobData {
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
            },
            scraper: SH3MobData {
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
            },
            missionary: SH3MobData {
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
            },
            leonard: SH3MobData {
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
            },
            alessa: SH3MobData {
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
            },
            god: SH3MobData {
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
            },
        }
    }

}

impl std::fmt::Display for SH3Sliders{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {        
        write!(f, "{},\n{},\n{},\n{},\n{},\n{},\n{},\n{},\n{},\n{},\n{},\n{},\n{},\n]", 
        self.nurse,
        self.pendulum,
        self.closer,
        self.numb_body,
        self.brown_slurper,
        self.insane_cancer,
        self.double_head,
        self.white_slurper,
        self.scraper,
        self.missionary,
        self.leonard,
        self.alessa,
        self.god)
    }
}

pub struct SH3MobData{
    pub main: u32,
    pub option_one: u32,
    pub option_two: u32,
    pub option_three: u32,
    pub option_four: u32,

    pub main_name: String,
    pub option_one_name: String, 
    pub option_two_name: String,
    pub option_three_name: String,
    pub option_four_name: String,
}

impl std::fmt::Display for SH3MobData{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {        
        write!(f, "[Name: {}, opts: {}, {}, {}, {}; Probs: {}, {}, {}, {}, {}]", 
        self.main_name,
        self.option_one_name,
        self.option_two_name,
        self.option_three_name,
        self.option_four_name,
        self.main,
        self.option_one,
        self.option_two,
        self.option_three,
        self.option_four)
    }
}

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
        }
    }
}

pub struct MyApp {
    pub name: String,
    pub age: u32,
    pub sh3_path: String,
	pub sh3_exe_name: String,
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