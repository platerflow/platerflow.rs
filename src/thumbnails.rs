use colored::*;
use glob::*;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::*;
use std::time::Duration;
use std::{fs, io::Write};
use base64::{Engine as _, engine::general_purpose};

struct Settings {
    recalculate_normals: bool,
    size_hint: bool,
    grid: bool,
    cam_elevation: f32,
    cam_azimuth: f32,
    timeout: Option<Duration>,
}
pub fn get_thumb(path: PathBuf) {
    let mut extension = path.clone();
    extension.set_extension("png");
    let settings = Settings {
        recalculate_normals: true,
        size_hint: false,
        grid: false,
        cam_elevation: 35.0,
        cam_azimuth: 45.0,
        timeout: None,
    };
    let input = path.as_path().display().to_string();
    let output = extension.as_path().display().to_string();

    let width = 500;
    let height = 500;
    let mut parser = stl2thumbnail::parser::Parser::from_file(&input, settings.recalculate_normals)
        .unwrap_or_else(|error| {
            panic!("Parser problem: {:?}", error);
        });
    let parsed_mesh = parser.read_all().unwrap_or_else(|error| {
        panic!("Mesher problem: {:?}", error);
    });
    create_still(width, height, &parsed_mesh, &output, &settings);

    get_thumb_from_file(extension.as_path().display().to_string(), path);
}
fn create_still(
    width: u32,
    height: u32,
    mesh: impl IntoIterator<Item = stl2thumbnail::mesh::Triangle> + Copy,
    path: &str,
    settings: &Settings,
) {
    let _elevation_angle = settings.cam_elevation * std::f32::consts::PI / 180.0;
    let mut backend = stl2thumbnail::rasterbackend::RasterBackend::new(width, height);
    backend.render_options.grid_visible = settings.grid;

    backend.render_options.view_pos = stl2thumbnail::mesh::Vec3::new(
        settings.cam_azimuth.to_radians().cos(),
        settings.cam_azimuth.to_radians().sin(),
        -settings.cam_elevation.to_radians().tan(),
    );

    let (aabb, scale) = backend.fit_mesh_scale(mesh);
    backend.render_options.zoom = 1.05;
    backend.render_options.draw_size_hint = settings.size_hint;

    backend
        .render(mesh, scale, &aabb, settings.timeout)
        .save(path)
        .expect("Error in render function");
}
fn get_thumb_from_file(path: String, gcode_path: PathBuf) {
    let gcode_path_stem = format!("{}*.gcode", gcode_path.with_extension("").display());
    //println!("{}*", gcode_path.with_extension("").display());
    let options = MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };
    let gcode_path = glob_with(&gcode_path_stem, options)
        .expect("Failed to read glob pattern")
        .next()
        .unwrap()
        .expect("Failed to read glob pattern");

    let mut f = File::open(path).expect("could not open file");
    let mut buf = Vec::new();
    f.read_to_end(&mut buf).expect("could not read file");
    let b64buffer = general_purpose::STANDARD.encode(&buf);
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
    println!("{}", "Thumbnail finished".blue().bold());
}
