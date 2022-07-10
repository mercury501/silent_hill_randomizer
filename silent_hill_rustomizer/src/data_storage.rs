use crate::data_structs;

impl data_structs::MyApp {
    pub fn init(&mut self) {
        #[allow(unused_mut)]
        let mut nurse = data_structs::SH3MobData {
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
        #[allow(unused_mut)]
        let mut pendulum = data_structs::SH3MobData {
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
        #[allow(unused_mut)]
        let mut closer = data_structs::SH3MobData {
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
        #[allow(unused_mut)]
        let mut numb_body = data_structs::SH3MobData {
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
        #[allow(unused_mut)]
        let mut brown_slurper = data_structs::SH3MobData {
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
        #[allow(unused_mut)]
        let mut insane_cancer = data_structs::SH3MobData {
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
        #[allow(unused_mut)]
        let mut dog = data_structs::SH3MobData {
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
        #[allow(unused_mut)]
        let mut white_slurper = data_structs::SH3MobData {
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
        #[allow(unused_mut)]
        let mut scraper = data_structs::SH3MobData {
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
        #[allow(unused_mut)]
        let mut missionary = data_structs::SH3MobData {
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
        #[allow(unused_mut)]
        let mut leonard = data_structs::SH3MobData {
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
        #[allow(unused_mut)]
        let mut alessa = data_structs::SH3MobData {
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
        #[allow(unused_mut)]
        let mut god = data_structs::SH3MobData {
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

        self.sh3_sliders.push(nurse);
        self.sh3_sliders.push(pendulum);
        self.sh3_sliders.push(closer);
        self.sh3_sliders.push(numb_body);
        self.sh3_sliders.push(brown_slurper);
        self.sh3_sliders.push(insane_cancer);
        self.sh3_sliders.push(dog);
        self.sh3_sliders.push(white_slurper);
        self.sh3_sliders.push(scraper);
        self.sh3_sliders.push(missionary);
        self.sh3_sliders.push(leonard);
        self.sh3_sliders.push(alessa);
        self.sh3_sliders.push(god);

        self.set_sh3_probability();

        //Silent Hill 2
        #[allow(unused_mut)]
        let mut lying_figure = data_structs::SH2MobData {
            main: 100,
            main_name: "Lying Figure".to_string(),
            main_perc_string: Default::default(),
            type_id: 0x200,
        };

        #[allow(unused_mut)]
        let mut nurse = data_structs::SH2MobData {
            main: 100,
            main_name: "Nurse".to_string(),
            main_perc_string: Default::default(),
            type_id: 0x207,
        };

        #[allow(unused_mut)]
        let mut mannequin = data_structs::SH2MobData {
            main: 100,
            main_name: "Mannequin".to_string(),
            main_perc_string: Default::default(),
            type_id: 0x201,
        };

        #[allow(unused_mut)]
        let mut dark_nurse = data_structs::SH2MobData {
            main: 100,
            main_name: "Dark Nurse".to_string(),
            main_perc_string: Default::default(),
            type_id: 0x20B,
        };

        #[allow(unused_mut)]
        let mut creeper = data_structs::SH2MobData {
            main: 100,
            main_name: "Creeper".to_string(),
            main_perc_string: Default::default(),
            type_id: 0x202,
        };

        #[allow(unused_mut)]
        let mut pyramid_head = data_structs::SH2MobData {
            main: 100,
            main_name: "Pyramid Head".to_string(),
            main_perc_string: Default::default(),
            type_id: 0x208,
        };

        self.sh2_sliders.push(lying_figure);
        self.sh2_sliders.push(nurse);
        self.sh2_sliders.push(mannequin);
        self.sh2_sliders.push(dark_nurse);
        self.sh2_sliders.push(creeper);
        self.sh2_sliders.push(pyramid_head);

        self.set_sh2_probability(false);
    }
}
