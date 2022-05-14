use subprocess::Exec;
use std::env;
use std::path::*;
use glob::*;
use crate::config::Config;
use std::process;
use rpt::*;
use color_eyre::*;

static mut CONTAINS_ACCENT: bool = false;

pub fn get_input_dir() -> PathBuf {
    let mut currdir: PathBuf = env::current_dir().unwrap();
    currdir.push("input/");
    return currdir
}
pub fn get_output_dir() -> PathBuf {
    let mut currdir: PathBuf = env::current_dir().unwrap();
    currdir.push("output/");
    return currdir
}
fn get_accent_conf() -> PathBuf {
    let mut currdir: PathBuf = env::current_dir().unwrap();
    currdir.push("output/");
    currdir.push("accent.conf");
    return currdir
}
fn get_main_conf() -> PathBuf {
    let mut currdir: PathBuf = env::current_dir().unwrap();
    currdir.push("output/");
    currdir.push("main.conf");
    return currdir
}

pub mod plater {
    pub fn list_files() {
        let mut _gid: String = super::get_input_dir().display().to_string();
        _gid.push_str("**/*.stl");
        let options = super::MatchOptions {
            case_sensitive: false,
            require_literal_separator: false,
            require_literal_leading_dot: false,
        };
        if super::glob_with(&_gid, options).expect("Failed to read glob pattern").count() > 0 {
            for entry in super::glob_with(&_gid, options).expect("Failed to read glob pattern") {
                match entry {
                    Ok(path) => write_plater_conf(path),
                    Err(e) => println!("{:#?}", e),
                }
            }
        } else {
            println!("No files detected in input");
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
        let mut number = 1u32;
        if analyze_name(&file).is_some() {
              number = analyze_name(&file).unwrap();
        }
        if file.starts_with("[a]") {
            if let Err(e) = writeln!(accentfile, "{} {}", filename.to_str().unwrap().to_string(), number) {
                println!("Error writing accentfile {:?} {}", super::get_accent_conf(), e);
            }
            unsafe {
                super::CONTAINS_ACCENT = true;
            }
        }
        else {
            if let Err(e) = writeln!(mainfile, "{} {}", filename.to_str().unwrap().to_string(), number) {
                println!("Error writing mainfile {:?} {}", super::get_main_conf(), e);
            }
        }
    }
    fn analyze_name(name: &str) -> Option<u32> {
        name
            .to_ascii_lowercase()
            .strip_suffix(".stl")?
            .rsplit_once("_x")?
            .1
            .parse()
            .ok()
    }
    pub fn run(config: &super::Config) {
        let cpus = num_cpus::get() / 2;
        println!("Running plater for the main color on {} cores", cpus);
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
                .arg("plater_main_%d")
                .arg(super::get_main_conf())
                .join()
                .unwrap();
        println!("Done.");
        unsafe {
            if super::CONTAINS_ACCENT {
                println!("Running plater for the accent color on {} cores", cpus);
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
                        .arg("plater_accent_%d")
                        .arg(super::get_accent_conf())
                        .join()
                        .unwrap();
                println!("Done.");
            } else {
                println!("No accent files detected, skipping.");
            }
        }
        let mut _gid: String = super::get_output_dir().display().to_string();
        _gid.push_str("**/*.stl");
        let options = super::MatchOptions {
            case_sensitive: false,
            require_literal_separator: false,
            require_literal_leading_dot: false,
        };
        for entry in super::glob_with(&_gid, options).expect("Failed to read glob pattern") {
            super::gen_thumb(entry.unwrap());
            /*match entry {
                Ok(path) => super::gen_thumb(path),
                Err(e) => println!("{:#?}", e),
            }*/
        }
    }
}
pub fn init_color_eyre()-> color_eyre::Result<()> {
    color_eyre::install()?;
    Ok(())
}
pub fn gen_thumb(path: PathBuf) -> color_eyre::Result<()> {
    use std::fs::File;
    let mut extension = path.clone();
    extension.set_extension("png");
    
    let mut scene = Scene::new();
    scene.add(
        Object::new(load_stl(File::open(path)?)?.scale(&glm::vec3(0.01, 0.01, 0.01)))
            .material(Material::diffuse(hex_color(0xB7CA79)))
    );
    /*scene.add(
        Object::new(plane(glm::vec3(0.0, 1.0, 0.0), -1.0))
            .material(Material::diffuse(hex_color(0xAAAAAA))),
    );*/
    scene.add(Light::Ambient(glm::vec3(0.01, 0.01, 0.01)));
    scene.add(Light::Object(
        Object::new(
            sphere()
                .scale(&glm::vec3(2.0, 2.0, 2.0))
                .translate(&glm::vec3(0.0, 20.0, 3.0)),
        )
        .material(Material::light(glm::vec3(1.0, 1.0, 1.0), 160.0)),
    ));
    scene.add(Light::Object(
        Object::new(
            sphere()
                .scale(&glm::vec3(0.05, 0.05, 0.05))
                .translate(&glm::vec3(-1.0, 0.71, 0.0)),
        )
        .material(Material::light(hex_color(0xFFAAAA), 400.0)),
    ));

    let camera = Camera::look_at(
        glm::vec3(5.0, 5.0, 5.0),
        glm::vec3(0.0, 0.0, 0.0),
        glm::vec3(-1.0, 0.0, 1.0),
        std::f64::consts::FRAC_PI_6,
    );
    Renderer::new(&scene, camera)
        /*.max_bounces(2)
        .num_samples(1)*/
        .render()
        .save(extension.as_path())?;

    Ok(())
}
pub mod superslicer {
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
                Ok(path) => slice(path, &config),
                Err(e) => println!("{:#?}", e),
            }
        }
    }
    fn slice(path: super::PathBuf, config: &super::Config) {
        let isaccent = path.clone().file_name().unwrap().to_str().unwrap().to_string();
        if isaccent.starts_with("plater_accent") {
            println!("Running SuperSlicer on {:?} with accent config", path);
            let _x = super::Exec::cmd(config.superslicer.path.to_string())
                    .arg("--load")
                    .arg(config.superslicer.accent_config_printer.to_string())
                    .arg("--load")
                    .arg(config.superslicer.accent_config_filament.to_string())
                    .arg("--load")
                    .arg(config.superslicer.accent_config_print.to_string())
                    .arg("-g")
                    .arg(path)
                    .join()
                    .unwrap();
        }
        else {
            println!("Running SuperSlicer on {:?} with standard config", path);
            let _x = super::Exec::cmd(config.superslicer.path.to_string())
                    .arg("--load")
                    .arg(config.superslicer.config_printer.to_string())
                    .arg("--load")
                    .arg(config.superslicer.config_filament.to_string())
                    .arg("--load")
                    .arg(config.superslicer.config_print.to_string())
                    .arg("-g")
                    .arg(path)
                    .join()
                    .unwrap();
        }
    }
}
