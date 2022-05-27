use std::path::*;
use stl_thumb::*;

pub fn get_thumb(path: PathBuf) {
    let mut extension = path.clone();
    extension.set_extension("png");
    let stlRenderConfig = stl_thumb::config::Config {
        stl_filename: path.display().to_string(),
        img_filename: Some(extension.as_path().display().to_string()),
        width: 300,
        height: 300,
        ..Default::default()
    };
    stl_thumb::render_to_file(&stlRenderConfig).expect("Error in run function");
}