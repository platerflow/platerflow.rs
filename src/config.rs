use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub superslicer: Superslicer,
    pub plater: Plater,
    pub moonraker: Moonraker,
}
#[derive(Debug, Deserialize)]
pub struct Plater {
    pub path: String,
    pub size_x: u8,
    pub size_y: u8,
    pub size_spacing: f32,
}
#[derive(Debug, Deserialize)]
pub struct Superslicer {
    pub path: String,
    pub config_printer: String,
    pub config_print: String,
    pub config_filament: String,
    pub accent_config_printer: String,
    pub accent_config_print: String,
    pub accent_config_filament: String,
}
#[derive(Debug, Deserialize)]
pub struct Moonraker {
    pub url: String,
}
pub mod init {
    use super::Config;
    use colored::*;
    use std::env;
    use std::io;
    use std::path::Path;
    use std::path::PathBuf;
    use std::process;

    pub fn check_paths(config: &super::Config) {
        if Path::new(&config.superslicer.path).exists() {
            println!("{}", "Superslicer path verified.".green().bold());
        } else {
            println!(
                "{}",
                "Superslicer path appears to be invalid/does not exist, exiting the program."
                    .red()
                    .bold()
            );
            process::exit(exitcode::OK);
        }
        if Path::new(&config.superslicer.config_printer).exists() {
            println!(
                "{}",
                "Superslicer config printer path verified.".green().bold()
            );
        } else {
            println!("{}", "Superslicer config printer path appears to be invalid/does not exist, exiting the program.".red().bold());
            process::exit(exitcode::OK);
        }
        if Path::new(&config.superslicer.config_print).exists() {
            println!(
                "{}",
                "Superslicer print config path verified.".green().bold()
            );
        } else {
            println!("{}", "Superslicer print config path appears to be invalid/does not exist, exiting the program.".red().bold());
            process::exit(exitcode::OK);
        }
        if Path::new(&config.superslicer.config_filament).exists() {
            println!(
                "{}",
                "Superslicer filament config path verified.".green().bold()
            );
        } else {
            println!("{}", "Superslicer filament config path appears to be invalid/does not exist, exiting the program.".red().bold());
            process::exit(exitcode::OK);
        }
        if Path::new(&config.plater.path).exists() {
            println!("{}", "Plater path verified.".green().bold());
        } else {
            println!(
                "{}",
                "Plater path appears to be invalid/does not exist, exiting the program."
                    .red()
                    .bold()
            );
            process::exit(exitcode::OK);
        }
    }

    pub fn check_present() -> bool {
        return std::path::Path::new(&get_config_path().unwrap()).exists();
    }

    fn get_config_path() -> io::Result<PathBuf> {
        let mut cfgfile = env::current_dir()?;
        cfgfile.push("config.toml");
        Ok(cfgfile)
    }

    pub fn read_config() -> Config {
        use std::fs::File;
        use std::io::Read;

        let f = File::open(&get_config_path().unwrap());
        let mut f = match f {
            Ok(file) => file,
            Err(error) => panic!("Problem opening the file: {:?}", error),
        };
        let mut data = String::new();
        f.read_to_string(&mut data).unwrap();
        let decoded: Config = toml::from_str(&data).unwrap();
        decoded
    }

    pub fn create_config() {
        use std::fs;

        let config_data_sample = r#"[plater]
size_x = 165
size_y = 165
size_spacing = 1
path = "D:\\Projects\\PlaterFlow\\platerbinary\\plater_cli_win.exe"
[superslicer]
path = "C:\\Users\\leand\\Desktop\\SuperSlicer_2.3.57.11_win64_220213\\superslicer_console.exe"
config_printer = "C:\\Users\\leand\\AppData\\Roaming\\SuperSlicer\\printer\\K3.ini"
config_filament = "C:\\Users\\leand\\AppData\\Roaming\\SuperSlicer\\filament\\FF Black K3 ASA.ini"
config_print = "C:\\Users\\leand\\AppData\\Roaming\\SuperSlicer\\print\\K3 ABS FF.ini"
accent_config_printer = "C:\\Users\\leand\\AppData\\Roaming\\SuperSlicer\\printer\\K3.ini"
accent_config_filament = "C:\\Users\\leand\\AppData\\Roaming\\SuperSlicer\\filament\\FF Black K3 ASA.ini"
accent_config_print = "C:\\Users\\leand\\AppData\\Roaming\\SuperSlicer\\print\\K3 ABS FF.ini"
[moonraker]
url = "http://192.168.0.248""#;

        let f = fs::write(&get_config_path().unwrap(), config_data_sample);
        let _f = match f {
            Ok(file) => file,
            Err(error) => panic!("Problem opening the file: {:?}", error),
        };
        self::read_config();
    }
}
