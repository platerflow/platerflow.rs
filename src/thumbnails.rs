use std::path::*;
use stl_thumb::*;
use base64::{encode, decode};
use std::io;
use std::io::prelude::*;
use std::fs::File;

pub fn get_thumb(path: PathBuf) {
    let mut extension = path.clone();
    extension.set_extension("png");
    let stlRenderConfig = stl_thumb::config::Config {
        stl_filename: path.display().to_string(),
        img_filename: Some(extension.as_path().display().to_string()),
        width: 500,
        height: 500,
        ..Default::default()
    };
    stl_thumb::render_to_file(&stlRenderConfig).expect("Error in run function");
    get_thumb_from_file(extension.as_path().display().to_string());
}
fn get_thumb_from_file(path: String) {
    let mut f = File::open(path).expect("could not open file");
    let mut buf = Vec::new();
    f.read_to_end(&mut buf).expect("could not read file");
    let mut b64buffer = base64::encode(&buf);
    //println!("{:#?}", b64buffer);
    let mut b64buffer = b64buffer.as_bytes()
        .chunks(78)
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap();
    
    println!("{:?}", b64buffer);
}
