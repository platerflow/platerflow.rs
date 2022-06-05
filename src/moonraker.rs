use crate::config::Config;
use crate::processes;
use glob::*;
use reqwest::blocking::multipart;
use std::path::*;
use colored::*;

pub fn run(config: &Config) {
    let mut _gid: String = processes::get_output_dir().display().to_string();
    _gid.push_str("**/*.gcode");
    let options = MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };
    for entry in glob_with(&_gid, options).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => upload(path, config),
            Err(e) => println!("{:#?}", e),
        }
    }
}
fn upload(path: PathBuf, config: &Config) {
    let path_str = path.to_str().unwrap();
    let moonraker_url = format!("{}/server/files/upload", config.moonraker.url);

    let form = multipart::Form::new()
        .text("name", "file")
        .text(
            "filename",
            Path::new(path_str)
                .file_name()
                .unwrap()
                .to_os_string()
                .into_string()
                .unwrap(),
        )
        .file("file", path_str)
        .unwrap();

    let client = reqwest::blocking::Client::new();
    let _resp = client.post(moonraker_url).multipart(form).send().unwrap();

    print!("{}", "Uploaded ".magenta());
    print!("{}", path_str.to_string().magenta());
    print!("{}", " to moonraker at: ".magenta());
    println!("{}", config.moonraker.url.magenta());
}
