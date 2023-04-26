use crate::config::Config;
use glob::*;
use regex::Regex;
use std::env;
use std::path::*;
use std::process;
use std::{thread, time};
use subprocess::Exec;
static mut CONTAINS_ACCENT: bool = false;

pub fn get_input_dir() -> PathBuf {
    let mut currdir: PathBuf = env::current_dir().unwrap();
    currdir.push("input/");
    currdir
}
pub fn get_output_dir() -> PathBuf {
    let mut currdir: PathBuf = env::current_dir().unwrap();
    currdir.push("output/");
    currdir
}
fn get_accent_conf() -> PathBuf {
    let mut currdir: PathBuf = env::current_dir().unwrap();
    currdir.push("output/");
    currdir.push("accent.conf");
    currdir
}
fn get_main_conf() -> PathBuf {
    let mut currdir: PathBuf = env::current_dir().unwrap();
    currdir.push("output/");
    currdir.push("main.conf");
    currdir
}

pub mod plater {
    use colored::*;
    pub fn list_files() {
        let mut _gid: String = super::get_input_dir().display().to_string();
        _gid.push_str("**/*.stl");
        let options = super::MatchOptions {
            case_sensitive: false,
            require_literal_separator: false,
            require_literal_leading_dot: false,
        };
        if super::glob_with(&_gid, options)
            .expect("Failed to read glob pattern")
            .count()
            > 0
        {
            for entry in super::glob_with(&_gid, options).expect("Failed to read glob pattern") {
                match entry {
                    Ok(path) => write_plater_conf(path),
                    Err(e) => println!("{:#?}", e),
                }
            }
        } else {
            println!("{}", "No files detected in input".red().bold());
            super::process::exit(exitcode::OK);
        }
    }

    pub fn write_plater_conf(filename: super::PathBuf) {
        use std::fs::OpenOptions;
        use std::io::prelude::*;
        let mut accentfile = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(super::get_accent_conf())
            .unwrap();
        let mut mainfile = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(super::get_main_conf())
            .unwrap();
        let file = filename.file_name().unwrap().to_str().unwrap().to_string();
        let number = analyze_name(&file);

        if file.starts_with("[a]") {
            if let Err(e) = writeln!(accentfile, "{} {}", filename.to_str().unwrap(), number) {
                println!(
                    "Error writing accentfile {:?} {}",
                    super::get_accent_conf(),
                    e
                );
            }
            unsafe {
                super::CONTAINS_ACCENT = true;
            }
        } else if let Err(e) = writeln!(mainfile, "{} {}", filename.to_str().unwrap(), number) {
            println!("Error writing mainfile {:?} {}", super::get_main_conf(), e);
        }
    }
    fn analyze_name(name: &str) -> &str {
        let re = super::Regex::new(r"_x([0-9]+)").unwrap();
        match re.captures(name) {
            Some(x) => x.get(1).unwrap().as_str(),
            None => "1",
        }
    }
    pub fn run(config: &super::Config) {
        let cpus = num_cpus::get();
        print!("{}", "Running plater for the main color on ".blue().bold());
        print!("{} ", cpus.to_string().blue().bold());
        println!("{}", "cores".blue().bold());
        let path = &config.plater.path;
        let _exec = super::Exec::cmd(&path)
            .arg("-W")
            .arg(config.plater.size_x.to_string())
            .arg("-H")
            .arg(config.plater.size_y.to_string())
            .arg("-s")
            .arg(config.plater.size_spacing.to_string())
            .arg("-t")
            .arg(cpus.to_string())
            .arg("-o")
            .arg("platerflow_main_%d")
            .arg(super::get_main_conf())
            .capture()
            .unwrap();
        println!("{}", "Done.".green().bold());
        unsafe {
            if super::CONTAINS_ACCENT {
                print!("{}", "Running plater for the accent color on ".blue().bold());
                print!("{} ", cpus.to_string().blue().bold());
                println!("{}", "cores".blue().bold());
                let _exec = super::Exec::cmd(&path)
                    .arg("-W")
                    .arg(config.plater.size_x.to_string())
                    .arg("-H")
                    .arg(config.plater.size_y.to_string())
                    .arg("-s")
                    .arg(config.plater.size_spacing.to_string())
                    .arg("-t")
                    .arg(cpus.to_string())
                    .arg("-o")
                    .arg("platerflow_accent_%d")
                    .arg(super::get_accent_conf())
                    .capture()
                    .unwrap();
                println!("{}", "Done.".green().bold());
            } else {
                println!("{}", "No accent files detected, skipping.".magenta());
            }
        }
    }
}
pub mod superslicer {
    use colored::*;
    pub fn run(config: &super::Config) {
        let mut _gid: String = super::get_output_dir().display().to_string();
        _gid.push_str("**/*.stl");
        let options = super::MatchOptions {
            case_sensitive: false,
            require_literal_separator: false,
            require_literal_leading_dot: false,
        };
        for entry in super::glob_with(&_gid, options).expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => slice(path, config),
                Err(e) => println!("{:#?}", e),
            }
        }
    }
    fn slice(path: super::PathBuf, config: &super::Config) {
        let isaccent = path.file_name().unwrap().to_str().unwrap().to_string();
        let tnpath = path.clone();
        if isaccent.starts_with("platerflow_accent") {
            print!(
                "{}",
                "Running SuperSlicer on "
                    .blue()
                    .bold()
            );
            print!("{}", path.display().to_string().blue().bold());
            println!("{}", " with accent config".blue().bold());
            let _x = super::Exec::cmd(&config.superslicer.path)
                .arg("--load")
                .arg(&config.superslicer.accent_config_printer)
                .arg("--load")
                .arg(&config.superslicer.accent_config_filament)
                .arg("--load")
                .arg(&config.superslicer.accent_config_print)
                .arg("-g")
                .arg(path)
                .capture()
                .unwrap();
        } else {
            print!(
                "{}",
                "Running SuperSlicer on "
                    .blue()
                    .bold()
            );
            print!("{}", path.display().to_string().blue().bold());
            println!("{}", " with standard config".blue().bold());
            let _x = super::Exec::cmd(&config.superslicer.path)
                .arg("--load")
                .arg(&config.superslicer.config_printer)
                .arg("--load")
                .arg(&config.superslicer.config_filament)
                .arg("--load")
                .arg(&config.superslicer.config_print)
                .arg("-g")
                .arg(path)
                .capture()
                .unwrap();
        }
        println!("{}", "Generating thumbnail".blue().bold());
        let ten_seconds = super::time::Duration::from_secs(10);
        super::thread::sleep(ten_seconds);
        super::super::thumbnails::get_thumb(tnpath);
    }
}
