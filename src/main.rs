mod config;
mod moonraker;
mod processes;
mod thumbnails;

use config::Config;
use std::fs;
use std::path::Path;
use std::process;
use colored::*;

fn main() {
    if Path::new(&processes::get_output_dir()).exists() {
        println!("{}", "Deleting output folder.".magenta());
        fs::remove_dir_all(processes::get_output_dir()).unwrap();
    }
    if !Path::new(&processes::get_input_dir()).exists() {
        println!("{}", "Creating input folder.".magenta());
        fs::create_dir(processes::get_input_dir()).unwrap();
    }
    println!("{}", "Creating output folder.".magenta());
    fs::create_dir(processes::get_output_dir()).unwrap();
    if config::init::check_present() {
        println!("{}", "Config found.".green().bold());
    } else {
        println!("{}", "No config found, creating one.".red().bold());
        config::init::create_config();
        println!("{}", 
            "Due to current development, we're closing this app for the time being so you can edit".red().bold()
        );
        println!("{}", "the config.toml currently created in the directory of which this app is located. Fire me up when ready! ;)".red().bold());
        process::exit(exitcode::OK);
    }
    let config: &Config = &config::init::read_config();
    config::init::check_paths(config);
    processes::plater::list_files();
    processes::plater::run(config);
    processes::superslicer::run(config);
    moonraker::run(config);
    println!("{}", "We're done!".green().bold());
}
