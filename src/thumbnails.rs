use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::*;
use std::{fs, io::Write};
pub fn get_thumb(path: PathBuf) {
    let mut extension = path.clone();
    extension.set_extension("png");
    let stl_render_config = stl_thumb::config::Config {
        stl_filename: path.display().to_string(),
        img_filename: Some(extension.as_path().display().to_string()),
        width: 300,
        height: 300,
        ..Default::default()
    };
    stl_thumb::render_to_file(&stl_render_config).expect("Error in run function");
    get_thumb_from_file(extension.as_path().display().to_string(), path);
}
fn get_thumb_from_file(path: String, gcode_path: PathBuf) {
    let mut gcode_path = gcode_path.clone();
    gcode_path.set_extension("gcode");
    let mut f = File::open(path).expect("could not open file");
    let mut buf = Vec::new();
    f.read_to_end(&mut buf).expect("could not read file");
    let b64buffer = base64::encode(&buf);
    let mut b64buffer_to_write: String = ";\r\n".to_owned();
    b64buffer_to_write.push_str("; thumbnail begin 300x300 ");
    b64buffer_to_write.push_str(b64buffer.len().to_string().as_str());
    b64buffer_to_write.push_str("\r\n");
    b64buffer.as_bytes().chunks(78).for_each(|s| {
        b64buffer_to_write.push_str("; ");
        b64buffer_to_write.push_str(unsafe { std::str::from_utf8_unchecked(s) });
        b64buffer_to_write.push_str("\r\n");
    });
    b64buffer_to_write.push_str("; thumbnail end\r\n;\r\n");
    let mut tmp_path = gcode_path.clone();
    tmp_path.set_extension("temp");
    let mut tmp = File::create(&tmp_path).expect("Opening temp path failed");
    let mut src = File::open(&gcode_path).expect("Opening gcode file failed");
    tmp.write_all(b64buffer_to_write.as_bytes())
        .expect("Writing to temp failed");
    io::copy(&mut src, &mut tmp).expect("Copy failed");
    fs::remove_file(&gcode_path).expect("Remove failed");
    fs::rename(&tmp_path, &gcode_path).expect("Rename failed");
    println!("Thumbnail finished");
}
