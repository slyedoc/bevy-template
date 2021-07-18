extern crate spritesheet_generator;
use spritesheet_generator::spritesheet_generator::generate;
use spritesheet_generator::spritesheet_generator_config::SpritesheetGeneratorConfig;

use std::env::var;

fn main() {

    let config_home = var("XDG_CONFIG_HOME")
        .or_else(|_| var("HOME").map(|home|format!("{}/.config", home)));
    println!("{:?}", config_home);

    let kenney = "/home/slyedoc/Code/kenney/".to_string();

    // let config = SpritesheetGeneratorConfig {
    //     max_width: 5000,
    //     max_height: 5000,
    //     border_padding: 4,
    //     input_folder: kenney.clone() + "kenneyDungeonPack_2.3/Isometric/",
    //     output_folder: kenney.clone() + "generated/",
    //     output_file_name: "dungeon-isometric".to_string(),
    //     allow_rotation: false,
    // };
    // generate(config);
    
    // let config = SpritesheetGeneratorConfig {
    //     max_width: 5000,
    //     max_height: 5000,
    //     border_padding: 4,
    //     input_folder: kenney.clone() + "kenneyDungeonPack_2.3/Angle/",
    //     output_folder: kenney.clone() + "generated/",
    //     output_file_name: "angle".to_string(),
    //     allow_rotation: false,
    // };
    // generate(config);
}

